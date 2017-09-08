#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::*;

#[derive(Debug)]
enum BitField {
    Single(u8),
    Range(std::ops::Range<u8>)
}

struct BitFields {
    bitfield: BitField,
    ident: Ident,
    ty: Ty
}

fn output_struct(name: &Ident, bitfields: &Vec<BitFields>) -> quote::Tokens {
    quote! {
        struct #name (u8);
        impl #name {
            pub fn get_bit() {

            }
        }
    }
}

#[proc_macro_attribute]
pub fn register(_: TokenStream, input: TokenStream) -> TokenStream {

    println!("{}", input);

    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();

    println!("ident: {}", ast.ident);

    let fields = match ast.body {
        Body::Enum(_) => panic!("enum not supported"),
        Body::Struct(x) => match x {
            VariantData::Struct(fields) => fields,
            _ => panic!("tuple and unit not supported")
        }
    };

    let mut bitfields: Vec<BitFields> = vec![];

    for field in &fields {
        let ident = field.ident.clone().unwrap();
        let ty_str = match field.clone().ty {
            Ty::Path(_, path) => path.segments[0].ident.clone(),
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

        let bitfield: BitField = if from.is_some() && to.is_some() {
            BitField::Range(std::ops::Range{start: from.unwrap(), end: to.unwrap()})
        } else {
            BitField::Single(at.unwrap())
        };

        println!("field {} @{:?}: {}", ident, bitfield, ty_str);
    }

    // return input;
    return output_struct(&ast.ident).parse().unwrap();
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