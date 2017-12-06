#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;
use syn::*;

#[macro_use]
extern crate quote;
use quote::Tokens;

#[derive(Debug)]
enum BitFieldPosition {
    Single(u8),
    Range(std::ops::Range<u8>)
}

impl BitFieldPosition {
    pub fn first_bit(&self) -> usize {
        match self {
            &BitFieldPosition::Single(x) => x as usize,
            &BitFieldPosition::Range(ref range) => range.start as usize
        }
    }

    pub fn last_bit(&self) -> usize {
        match self {
            &BitFieldPosition::Single(x) => x as usize,
            &BitFieldPosition::Range(ref range) => (range.end - 1) as usize
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &BitFieldPosition::Single(_) => 1,
            &BitFieldPosition::Range(ref range) => (range.end - range.start) as usize
        }
    }

    pub fn byte_len(&self) -> usize {
        let len = self.len();
        if len % 8 == 0 { len / 8 } else { len / 8 + 1 }
    }
}

#[derive(Debug)]
struct BitField {
    position: BitFieldPosition,
    ident: Ident,
    ty: Ty
}

fn filled_byte(from:u8, to:u8) -> u8 {
    let mut res = 0;
    for i in from..to {
        res |= 1 << i;
    }
    return res;
}

fn emit_read_single_byte(from:Tokens, from_bit:usize, to_bit:usize) -> Tokens {
    assert!(to_bit - from_bit <= 8);

    let bit_len = (to_bit - from_bit) as u8;
    let from_byte = (from_bit / 8) as usize;
    let from_bit_mask = (from_bit % 8) as u8;

    if from_bit % 8 == 0 {
        let mask = filled_byte(from_bit_mask, from_bit_mask + bit_len);
        return quote! { #from[#from_byte] & #mask };
    } else {
        let second_byte = ((to_bit - 1) / 8) as usize;
        if second_byte == from_byte {
            let mask = filled_byte(from_bit_mask, from_bit_mask + bit_len);
            let offset = from_bit % 8;

            println!("emit_read_single_byte {} & {} >> {}", from_byte, mask, offset);

            return quote! {
                (#from[#from_byte] & #mask) >> #offset
            };
        } else {
            let first_byte_mask = filled_byte(from_bit_mask, 8);
            let first_byte_offset = from_bit % 8;

            let second_byte_mask = filled_byte(0, from_bit_mask);
            let second_byte_offset = 8 - first_byte_offset;

            println!("emit_read_single_byte {} & {} >> {} | {} & {} << {}", from_byte, first_byte_mask, first_byte_offset, second_byte, second_byte_mask, second_byte_offset);

            return quote! {
                ((#from[#from_byte] & #first_byte_mask) >> #first_byte_offset)
                | ((#from[#second_byte] & #second_byte_mask) << #second_byte_offset)
            };
        }
    }
}

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

        let first_bit = bitfield.position.first_bit();
        let last_bit = bitfield.position.last_bit();
        let value_byte_len = bitfield.position.byte_len();

        let mut getter_body = quote! {
            let mut value_array: [u8;#value_byte_len] = [0;#value_byte_len];
        };

        for i in 0..value_byte_len {
            let from_bit_i = first_bit + 8 * i;
            let to_bit_i = usize::min(from_bit_i + 8, last_bit + 1);
            let read_byte = emit_read_single_byte(quote! { self.0 }, from_bit_i, to_bit_i);
            getter_body = quote! { #getter_body
                value_array[#i] = #read_byte;
            };
        }

        println!("getter body {}", getter_body);

        match &bitfield.position {
            &BitFieldPosition::Single(x) => {
                let mask: u8 = 1 << (x % 8);
                let nmask: u8 = !mask;

                let byteidx: usize = x as usize / 8;

                let shift = x % 8;

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        #getter_body
                        return bitfield_register::FromBitfield::from_bitfield(value_array);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: [u8;1] = bitfield_register::IntoBitfield::into_bitfield(value);
                        self.0[#byteidx] &= #nmask;
                        self.0[#byteidx] |= (raw[0] & 1) << #shift;
                    }
                }
            },
            &BitFieldPosition::Range(ref range) => {
                let from = range.start;
                let to = range.end - 1; // end is exclusive

                let value_byte_len = bitfield.position.byte_len();

                let from_byte: usize = from as usize / 8;
                let to_byte: usize = to as usize / 8;

                let raw_size = to_byte - from_byte + 1;

                println!("value_byte_len:{}", value_byte_len);
                println!("from_byte:{}, to_byte:{}", from_byte, to_byte);
                println!("raw_size:{}", raw_size);

                
                // let type_mask: u8 = (1 << type_bit_size) - 1;
                // let mask: u8 = type_mask << (from % 8);
                // let nmask: u8 = !mask;

                let bit_shift = from % 8;
                let bit_end = to % 8;

                let mut setter_body = quote! {
                    let value_array: [u8;#value_byte_len] = bitfield_register::IntoBitfield::into_bitfield(value);
                    let mut raw: u8;
                };

                for i in 0..raw_size {
                    
                    if i < value_byte_len {
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

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        #getter_body
                        return bitfield_register::FromBitfield::from_bitfield(value_array);
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
        impl bitfield_register::BitfieldRegister for #name {
            type Data = [u8;#base_size];
            const REGISTER_SIZE: usize = #base_size;
            fn data(&self) -> &[u8;#base_size] {
                &self.0
            }
        }
        impl From<[u8;#base_size]> for #name {
            fn from(buffer: [u8;#base_size]) -> Self {
                return #name(buffer);
            }
        }
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

    println!("{}", s);

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
            BitFieldPosition::Range(std::ops::Range{start: from.unwrap(), end: to.unwrap() + 1})
        } else {
            BitFieldPosition::Single(at.unwrap())
        };

        let ident = field.ident.clone().unwrap();

        bitfields.push(BitField {position: position, ident: ident, ty: ty});
    }

    let name = &ast.ident;

    return output_struct(name, &bitfields).parse().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_test() {
        let pos = BitFieldPosition::Single(0);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Single(1);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Single(7);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Single(8);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Single(9);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Single(15);

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((0..1));

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((0..5));

        assert_eq!(pos.len(), 5);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((0..8));

        assert_eq!(pos.len(), 8);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((0..9));

        assert_eq!(pos.len(), 9);
        assert_eq!(pos.byte_len(), 2);

        let pos = BitFieldPosition::Range((0..10));

        assert_eq!(pos.len(), 10);
        assert_eq!(pos.byte_len(), 2);

        let pos = BitFieldPosition::Range((0..15));

        assert_eq!(pos.len(), 15);
        assert_eq!(pos.byte_len(), 2);

        let pos = BitFieldPosition::Range((1..2));

        assert_eq!(pos.len(), 1);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((1..8));

        assert_eq!(pos.len(), 7);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((1..9));

        assert_eq!(pos.len(), 8);
        assert_eq!(pos.byte_len(), 1);

        let pos = BitFieldPosition::Range((1..10));

        assert_eq!(pos.len(), 9);
        assert_eq!(pos.byte_len(), 2);
    }

    #[test]
    fn emit_read_single_byte_test() {
        let from = quote!{arr};
        let res = emit_read_single_byte(from.clone(), 0, 8);
        assert_eq!(res, quote!{ #from[0usize] & 255u8 });

        let left_mask : u8 = 0b10000000;
        let from = quote!{arr};
        let res = emit_read_single_byte(from.clone(), 7, 8);
        assert_eq!(res, quote!{ (#from[0usize] & #left_mask) >> 7usize });

        let left_mask : u8 = 0b00011100;
        let from = quote!{arr};
        let res = emit_read_single_byte(from.clone(), 2, 5);
        assert_eq!(res, quote!{ (#from[0usize] & #left_mask) >> 2usize });


        let left_mask : u8 = 0b11111110;
        let right_mask : u8 = 0b00000001;
        let from = quote!{arr};
        let res = emit_read_single_byte(from.clone(), 9, 17);
        assert_eq!(res, quote!{ ((#from[1usize] & #left_mask) >> 1usize) | ((#from[2usize] & #right_mask) << 7usize) });
    }

    #[test]
    fn filled_byte_test() {
        assert_eq!(filled_byte(0,0), 0);
        assert_eq!(filled_byte(0,1), 0b1);
        assert_eq!(filled_byte(0,2), 0b11);

        assert_eq!(filled_byte(1,2), 0b10);
        assert_eq!(filled_byte(1,3), 0b110);

        assert_eq!(filled_byte(6,8), 0b11000000);
        assert_eq!(filled_byte(7,8), 0b10000000);

        assert_eq!(filled_byte(8,8), 0);
    }
}