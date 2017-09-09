#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::*;

#[derive(Debug)]
enum BitFieldPosition {
    Single(u8),
    Range(std::ops::Range<u8>)
}

struct BitField {
    position: BitFieldPosition,
    ident: Ident,
    ty: Ty
}

/*
fn output_struct(name: &Ident, bitfields: &Vec<BitField>) -> quote::Tokens {
    let mut impl_body = quote! {};

    for bitfield in &bitfields.into_iter() {
        println!("field {} @{:?}", bitfield.ident, bitfield.position);
        let ident = bitfield.ident;
        let ty = bitfield.ty;

        impl_body = quote! {
            #impl_body
            get_#ident(&self) -> #ty {

            }
        }
    };

    quote! {
        struct #name (u8);
        impl #name {
            #impl_body
        }
    }
}
*/

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

    let mut bitfields: Vec<BitField> = vec![];

    let mut impl_body = quote! {};

    for field in &fields {
        let ident = field.ident.clone().unwrap();
        let ty_str = match &field.ty {
            &Ty::Path(_, ref path) => path.segments[0].ident.clone(),
            _ => panic!("only path types supported"),
        };

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

        println!("field {} @{:?}: {}", ident, position, ty_str);

        // println!("field {} @{:?}", ident, position);

        let getter_str = format!("get_{}", ident.as_ref());
        let getter: Ident = From::from(getter_str.as_str());

        let setter_str = format!("set_{}", ident.as_ref());
        let setter: Ident = From::from(setter_str.as_str());

        match &position {
            &BitFieldPosition::Single(x) => {
                let mask: u8 = 1 << x;
                let nmask: u8 = !mask;

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        let raw: u8 = (self.0[0] & #mask) >> #x;
                        return std::convert::From::from(raw);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: u8 = std::convert::Into::into(value);
                        self.0[0] &= #nmask;
                        self.0[0] |= (raw & 1) << #x;
                    }
                }
            },
            &BitFieldPosition::Range(ref range) => {
                let from: u8 = range.start;
                let to = range.end;

                let size = to - from + 1;
                let type_mask: u8 = (1 << size) - 1;
                let mask: u8 = type_mask << from;
                let nmask: u8 = !mask;

                impl_body = quote! {
                    #impl_body

                    pub fn #getter(&self) -> #ty {
                        let raw: u8 = (self.0[0] & #mask) >> #from;
                        return std::convert::From::from(raw);
                    }

                    pub fn #setter(&mut self, value: #ty) {
                        let raw: u8 = std::convert::Into::into(value);
                        self.0[0] &= #nmask;
                        self.0[0] |= (raw & #type_mask) << #from;
                    }
                }
            },
        };

        bitfields.push(BitField {position: position, ident: ident, ty: ty});
    }

    let name = &ast.ident;

    return (quote! {
        struct #name ([u8;3]);
        impl Default for #name {
            fn default() -> Self {
                return #name ([0,0,0]);
            }
        }
        impl #name {

            #impl_body
        }
    }).parse().unwrap();

    // return input;
    // return output_struct(&ast.ident, &bitfields).parse().unwrap();
}

/*
Struct(
    Struct([
        Field {
            ident: Some(Ident("foo")),
            vis: Inherited,
            attrs: [
                Attribute {
                    style: Outer,
                    value: List(
                        Ident("bitfield"),
                        [
                            MetaItem(
                                NameValue(
                                    Ident("at"), Int(0, Unsuffixed)
                                )
                            )
                        ]
                    ),
                    is_sugared_doc: false
                }
            ],
            ty: Path(
                None,
                Path {
                    global: false,
                    segments: [
                        PathSegment {
                            ident: Ident("u8"),
                            parameters: AngleBracketed(
                                AngleBracketedParameterData {
                                    lifetimes: [],
                                    types: [],
                                    bindings: []
                                }
                            )
                        }
                    ]
                }
            )
        }
    ])
)
*/