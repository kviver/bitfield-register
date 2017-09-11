#![feature(proc_macro)]

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


fn output_struct(name: &Ident, bitfields: &Vec<BitField>) -> quote::Tokens {
    let mut impl_body = quote! {};
    
    let starts: Vec<u8> = bitfields.iter().map(|x| match &x.position {
        &BitFieldPosition::Single(x) => x,
        &BitFieldPosition::Range(ref range) => range.start
    }).collect();

    let max_starts = starts.iter().max().unwrap();

    let base_size: usize = (*max_starts as usize / 8) + 1;

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
                        return std::convert::From::from(raw);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: [u8;1] = std::convert::Into::into(value);
                        self.0[#byteidx] &= #nmask;
                        self.0[#byteidx] |= (raw[0] & 1) << #shift;
                    }
                }
            },
            &BitFieldPosition::Range(ref range) => {
                let from = range.start;
                let to = range.end;

                let size = to - from + 1;
                let type_mask: u8 = (1 << size) - 1;
                let mask: u8 = type_mask << (from % 8);
                let nmask: u8 = !mask;

                let byteidx: usize = from as usize / 8;

                let shift = from % 8;

                let typesize: usize = size as usize / 8 + 1;

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        let raw: [u8;#typesize] = [(self.0[#byteidx] & #mask) >> #shift];
                        return std::convert::From::from(raw);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: [u8;#typesize] = std::convert::Into::into(value);
                        self.0[#byteidx] &= #nmask;
                        self.0[#byteidx] |= (raw[0] & #type_mask) << #shift;
                    }
                }
            },
        }
    };

    return quote! {
        struct #name ([u8;#base_size]);
        impl Default for #name {
            fn default() -> Self {
                return #name ([0;#base_size]);
            }
        }
        impl #name {

            #impl_body
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
        let ty_str = match &field.ty {
            &Ty::Path(_, ref path) => path.segments[0].ident.clone(),
            _ => panic!("only path types supported"),
        };

        println!("field {} @{:?}: {}", ident, position, ty_str);

        bitfields.push(BitField {position: position, ident: ident, ty: ty});
    }

    let name = &ast.ident;

    return output_struct(name, &bitfields).parse().unwrap();
}