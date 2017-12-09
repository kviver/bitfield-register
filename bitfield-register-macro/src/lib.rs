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

// from - expr of type &[u8], array of bytes, interpreted as array of bits
// will emit expression of type u8, reading [from_bit, from_bit + bit_length) from array
fn emit_read_single_byte(from:Tokens, from_bit:usize, bit_length:u8) -> Tokens {
    assert!(bit_length <= 8);

    let to_bit = from_bit + bit_length as usize;
    let from_byte = (from_bit / 8) as usize;
    let from_bit_mask = (from_bit % 8) as u8;

    if from_bit % 8 == 0 {
        let mask = filled_byte(from_bit_mask, from_bit_mask + bit_length);
        return quote! { #from[#from_byte] & #mask };
    } else {
        let second_byte = ((to_bit - 1) / 8) as usize;
        if second_byte == from_byte {
            let mask = filled_byte(from_bit_mask, from_bit_mask + bit_length);
            let offset = from_bit % 8;

            return quote! {
                (#from[#from_byte] & #mask) >> #offset
            };
        } else {
            let first_byte_mask = filled_byte(from_bit_mask, 8);
            let first_byte_offset = from_bit % 8;

            let second_byte_mask = filled_byte(0, from_bit_mask);
            let second_byte_offset = 8 - first_byte_offset;

            return quote! {
                ((#from[#from_byte] & #first_byte_mask) >> #first_byte_offset)
                | ((#from[#second_byte] & #second_byte_mask) << #second_byte_offset)
            };
        }
    }
}

// from - expr of type u8
// to - expr of type &[u8], array of bytes, interpreted as array of bits
// will emit expression of type (), writing [from_bit, to_bit) to array
fn emit_write_single_byte(to:Tokens, from:Tokens, from_bit:usize, bit_length:u8) -> Tokens {
    assert!(bit_length <= 8);

    let to_bit = from_bit + bit_length as usize;
    let from_byte = (from_bit / 8) as usize;
    let from_bit_mask = (from_bit % 8) as u8;

    if from_bit % 8 == 0 {
        let src_mask = filled_byte(from_bit_mask, from_bit_mask + bit_length);
        let dst_mask = !src_mask;
        return quote! { #to[#from_byte] = (#from & #src_mask) | (#to[#from_byte] & #dst_mask) };
    } else {
        let second_byte = ((to_bit - 1) / 8) as usize;
        if second_byte == from_byte {
            let src_mask = filled_byte(from_bit_mask, from_bit_mask + bit_length);
            let dst_mask = !src_mask;
            let offset = from_bit % 8;

            return quote! {
                #to[#from_byte] = ((#from << #offset) & #src_mask) | (#to[#from_byte] & #dst_mask)
            };
        } else {
            let first_byte_src_mask = filled_byte(from_bit_mask, 8);
            let first_byte_dst_mask = !first_byte_src_mask;
            let first_byte_offset = from_bit % 8;

            let second_byte_src_mask = filled_byte(0, from_bit_mask);
            let second_byte_dst_mask = !second_byte_src_mask;
            let second_byte_offset = 8 - first_byte_offset;

            let set_first_byte = quote! {
                #to[#from_byte] = ((#from << #first_byte_offset) & #first_byte_src_mask)
                    | (#to[#from_byte] & #first_byte_dst_mask)
            };

            let set_second_byte = quote! {
                #to[#second_byte] = ((#from >> #second_byte_offset) & #second_byte_src_mask)
                    | (#to[#second_byte] & #second_byte_dst_mask)
            };

            return quote! {
                #set_first_byte;
                #set_second_byte;
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

        let mut setter_body = quote! {
            let value_array: [u8;#value_byte_len] = ::bitfield_register::IntoBitfield::into_bitfield(value);
        };

        for i in 0..value_byte_len {
            let from_bit_i = first_bit + 8 * i;
            let to_bit_i = usize::min(from_bit_i + 8, last_bit + 1);
            let bit_length = (to_bit_i -  from_bit_i) as u8;

            let read_byte = emit_read_single_byte(quote! { self.0 }, from_bit_i, bit_length);
            getter_body = quote! { #getter_body
                value_array[#i] = #read_byte;
            };

            let write_byte = emit_write_single_byte(quote!{ self.0 }, quote! { value_array[#i] }, from_bit_i, bit_length);
            setter_body = quote! { #setter_body
                #write_byte;
            };
        }

        println!("getter body {}", getter_body);
        println!("setter body {}", setter_body);

        impl_body = quote! {
            #impl_body

            pub fn #getter(&self) -> #ty {
                #getter_body
                return ::bitfield_register::FromBitfield::from_bitfield(value_array);
            }

            pub fn #setter(&mut self, value: #ty) -> () {
                #setter_body
            }
        };
    };

    return quote! {
        pub struct #name ([u8;#base_size]);
        impl ::bitfield_register::BitfieldRegister for #name {
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
        let res = emit_read_single_byte(from.clone(), 7, 1);
        assert_eq!(res, quote!{ (#from[0usize] & #left_mask) >> 7usize });

        let left_mask : u8 = 0b00011100;
        let res = emit_read_single_byte(from.clone(), 2, 3);
        assert_eq!(res, quote!{ (#from[0usize] & #left_mask) >> 2usize });


        let left_mask : u8 = 0b11111110;
        let right_mask : u8 = 0b00000001;
        let res = emit_read_single_byte(from.clone(), 9, 8);
        assert_eq!(res, quote!{ ((#from[1usize] & #left_mask) >> 1usize) | ((#from[2usize] & #right_mask) << 7usize) });
    }

    #[test]
    fn emit_write_single_byte_test() {
        let from = quote!{val};
        let to = quote!{arr};

        let res = emit_write_single_byte(to.clone(), from.clone(), 0, 8);
        assert_eq!(res, quote!{ #to[0usize] = (#from & 255u8) | (#to[0usize] & 0u8) });

        let left_src_mask : u8 = 0b00000011;
        let left_dst_mask : u8 = 0b11111100;
        let res = emit_write_single_byte(to.clone(), from.clone(), 0, 2);
        assert_eq!(res, quote!{ #to[0usize] = (#from & #left_src_mask) | (#to[0usize] & #left_dst_mask) });

        let left_src_mask : u8 = 0b10000000;
        let left_dst_mask : u8 = 0b01111111;
        let res = emit_write_single_byte(to.clone(), from.clone(), 7, 1);
        assert_eq!(res, quote!{ #to[0usize] = ((#from << 7usize) & #left_src_mask) | (#to[0usize] & #left_dst_mask) });

        let left_src_mask : u8 = 0b00011100;
        let left_dst_mask : u8 = 0b11100011;
        let res = emit_write_single_byte(to.clone(), from.clone(), 2, 3);
        assert_eq!(res, quote!{ #to[0usize] = ((#from << 2usize) & #left_src_mask) | (#to[0usize] & #left_dst_mask) });


        let left_src_mask : u8 = 0b11111110;
        let left_dst_mask : u8 = 0b00000001;
        let right_src_mask : u8 = 0b00000001;
        let right_dst_mask : u8 = 0b11111110;
        let res = emit_write_single_byte(to.clone(), from.clone(), 9, 8);
        assert_eq!(res, quote!{
            #to[1usize] = ((#from << 1usize) & #left_src_mask)  | (#to[1usize] & #left_dst_mask);
            #to[2usize] = ((#from >> 7usize) & #right_src_mask) | (#to[2usize] & #right_dst_mask);
        });
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