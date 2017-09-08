#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


fn output_struct(name: &syn::Ident) -> quote::Tokens {
    quote! {
        struct #name (u8);
        impl #name {
            pub fn get_bit() {

            }
        }
    }
}

#[derive(Debug)]
enum BitField {
    Single(u8),
    Range(std::ops::Range<u8>)
}

#[proc_macro_attribute]
pub fn register(_: TokenStream, input: TokenStream) -> TokenStream {
    println!("{}", input);

    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    println!("ident: {}", ast.ident);

    let fields = match ast.body {
        syn::Body::Enum(_) => panic!("enum not supported"),
        syn::Body::Struct(x) => match x {
            syn::VariantData::Struct(fields) => fields,
            _ => panic!("tuple and unit not supported")
        }
    };

    for field in &fields {
        let ident = field.ident.clone().unwrap();
        let ty = match field.clone().ty {
            syn::Ty::Path(_, path) => path.segments[0].ident.clone(),
            _ => panic!("only path types supported"),
        };

        let mut bitfield: Option<BitField> = None;

        for attr in &field.attrs {
            if let syn::MetaItem::List(attr_ident, attr_nest) = attr.clone().value {
                println!("found attribute: {}", attr_ident);
                if attr_ident == "bitfield" {
                    println!("found bitfield attribute: {:?}", attr_nest);
                }
            }
        }

        bitfield = match 1 {
            0 => Some(BitField::Single(10)),
            1 => Some(BitField::Range(std::ops::Range{start: 3, end: 5})),
            _ => panic!("wrong bitfield position"),
        };
        
        println!("field {} @{:?}: {}", ident, bitfield, ty);
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
                        [MetaItem(NameValue(Ident("at"), Int(0, Unsuffixed)))]
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