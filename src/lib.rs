#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;
use syn::*;

#[macro_use]
extern crate quote;

#[derive(Debug)]
enum BitFieldPosition {
    Single(u8),
    Range(std::ops::Range<u8>)
}

#[derive(Debug)]
struct BitField {
    position: BitFieldPosition,
    ident: Ident,
    ty: Ty
}

// TODO add tunable bit order for register as a whole
// TODO add tunable byte order for register as a whole
fn output_struct(name: &Ident, bitfields: &Vec<BitField>) -> quote::Tokens {
    
    let ends: Vec<u8> = bitfields.iter().map(|x| match &x.position {
        &BitFieldPosition::Single(x) => x,
        &BitFieldPosition::Range(ref range) => range.end
    }).collect();

    let max_ends = ends.iter().max().unwrap();

    let base_size: usize = (*max_ends as usize / 8) + 1;

    let mut impl_body = quote! {};

    for bitfield in bitfields {
        println!("iter field {} @{:?}", bitfield.ident, bitfield.position);

        let getter_str = format!("get_{}", bitfield.ident.as_ref());
        let getter: Ident = From::from(getter_str.as_str());

        let setter_str = format!("set_{}", bitfield.ident.as_ref());
        let setter: Ident = From::from(setter_str.as_str());

        let ty = &bitfield.ty;

        match &bitfield.position {
            &BitFieldPosition::Single(x) => {
                let mask: u8 = 1 << (x % 8);
                let nmask: u8 = !mask;

                let byteidx: usize = x as usize / 8;

                let shift = x % 8;

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        let raw: [u8;1] = [(self.0[#byteidx] & #mask) >> #shift];
                        return FromBitfield::from_bitfield(raw);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: [u8;1] = IntoBitfield::into_bitfield(value);
                        self.0[#byteidx] &= #nmask;
                        self.0[#byteidx] |= (raw[0] & 1) << #shift;
                    }
                }
            },
            &BitFieldPosition::Range(ref range) => {
                let from = range.start;
                let to = range.end; 

                let value_size: usize = (to - from) as usize / 8 + 1;
                // let value_bit_size = to - from + 1;

                let from_byte: usize = from as usize / 8;
                let to_byte: usize = to as usize / 8;

                let raw_size = to_byte - from_byte + 1;

                println!("value_byte_size:{}", value_size);
                println!("from_byte:{}, to_byte:{}", from_byte, to_byte);
                println!("raw_size:{}", raw_size);

                
                // let type_mask: u8 = (1 << type_bit_size) - 1;
                // let mask: u8 = type_mask << (from % 8);
                // let nmask: u8 = !mask;

                let bit_shift = from % 8;
                let bit_end = to % 8;

                let mut setter_body = quote! {
                    let value_array: [u8;#value_size] = IntoBitfield::into_bitfield(value);
                    let mut raw: u8;
                };

                for i in 0..raw_size {
                    
                    if i < value_size {
                        if bit_shift == 0 {
                            setter_body = quote! { #setter_body
                                 raw = value_array[#i];
                            }
                        } else {
                            setter_body = quote! { #setter_body
                                raw = value_array[#i] << #bit_shift;
                            }
                        }
                    } else {
                        setter_body = quote! { #setter_body
                            raw = 0;
                        }
                    }

                    if i != 0 {
                        let i_1 = i - 1;
                        if bit_shift != 0 {
                            setter_body = quote! { #setter_body
                                raw |= value_array[#i_1] >> (8 - #bit_shift);
                            }
                        }
                    }

                    let mask: u8 = if bit_end == 7 || i != raw_size - 1 {0xff}
                    else {(1 << (bit_end + 1)) - 1};

                    let i_from_byte = i + from_byte;

                    if mask != 0xff {
                        setter_body = quote! { #setter_body
                            self.0[#i_from_byte] &= !#mask;
                            self.0[#i_from_byte] |= raw & #mask;
                        };
                    } else {
                        setter_body = quote! { #setter_body
                            self.0[#i_from_byte] = raw;
                        };
                    }
                }

                let mut getter_body = quote! {
                    let mut value_array: [u8;#value_size] = [0;#value_size];
                };

                for i in 0..value_size {

                    let i_from_byte = i + from_byte;

                    let mask: u8 = if bit_end == 7 || i != raw_size - 1 {0xff}
                    else {(1 << (bit_end + 1)) - 1};
                    
                    getter_body = quote! { #getter_body
                        value_array[#i] = self.0[#i_from_byte] >> #bit_shift;
                    };

                    if i < raw_size - 1 {
                        let i_from_byte = i + from_byte + 1;
                        if bit_shift != 0 {
                            getter_body = quote! { #getter_body
                                value_array[#i] |= self.0[#i_from_byte] << (8 - #bit_shift);
                            }
                        }
                    }

                    if mask != 0xff {
                        getter_body = quote! { #getter_body
                            value_array[#i] &= #mask;
                        };
                    }
                }

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        #getter_body
                        return FromBitfield::from_bitfield(value_array);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        #setter_body
                    }
                }
            },
        }
    };

    return quote! {
        pub struct #name ([u8;#base_size]);
        impl Default for #name {
            fn default() -> Self {
                return #name ([0;#base_size]);
            }
        }
        impl #name {

            #impl_body
        }
        impl Clone for #name {
            fn clone(&self) -> Self {
                return #name (self.0.clone());
            }
        }
    }
}

#[proc_macro_attribute]
pub fn register(_: TokenStream, input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();

    let fields = match ast.body {
        Body::Enum(_) => panic!("enum not supported"),
        Body::Struct(x) => match x {
            VariantData::Struct(fields) => fields,
            _ => panic!("tuple and unit not supported")
        }
    };

    #[derive(Debug)]
    let mut bitfields: Vec<BitField> = vec![];

    for field in &fields {
        let ty = field.clone().ty;

        let mut from: Option<u8> = None;
        let mut to: Option<u8> = None;
        let mut at: Option<u8> = None;

        for attr in &field.attrs {
            if let MetaItem::List(attr_ident, attr_nest) = attr.clone().value {
                if attr_ident == "bitfield" {
                    for attr_nest_item in &attr_nest {
                        match attr_nest_item.clone() {
                            NestedMetaItem::MetaItem(MetaItem::NameValue(nv_ident, Lit::Int(nv_value, _))) => {
                                match nv_ident.as_ref() {
                                    "at" => (at = Some(nv_value as u8)),
                                    "from" => (from = Some(nv_value as u8)),
                                    "to" => (to = Some(nv_value as u8)),
                                    _ => panic!("unsupported param name (use 'at' or 'from'/'to')"),
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if (from.is_some() || to.is_some()) && at.is_some() {
            panic!("select 'at' or 'from'/'to' parameters, not both");
        }

        if from.is_some() ^ to.is_some() {
            panic!("select 'from' and 'to' parameters together");
        }

        if from.is_none() && to.is_none() && at.is_none() {
            panic!("select bit parameters (use #[bitfield(at=x or from=x to=y)])");
        }

        let position: BitFieldPosition = if from.is_some() && to.is_some() {
            BitFieldPosition::Range(std::ops::Range{start: from.unwrap(), end: to.unwrap()})
        } else {
            BitFieldPosition::Single(at.unwrap())
        };

        let ident = field.ident.clone().unwrap();

        bitfields.push(BitField {position: position, ident: ident, ty: ty});
    }

    let name = &ast.ident;

    return output_struct(name, &bitfields).parse().unwrap();
}