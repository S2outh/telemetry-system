use quote::quote;
use proc_macro2::TokenStream;

pub fn impl_macro(ast: syn::DeriveInput) -> TokenStream {
    let type_name = &ast.ident;
    let syn::Data::Struct(tm_value_struct) = ast.data else {
        panic!("tm value is not a struct");
    };
    let struct_byte_parsers = tm_value_struct.fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let ident = &f.ident;
            quote! {{
                let pos = Self::get_pos(#i);
                self.#ident.write(&mut mem[pos..(pos+Self::SIZES[#i])]);
            }}
        });
    let struct_type_parsers = tm_value_struct.fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let ident = &f.ident;
            quote! {{
                let pos = Self::get_pos(#i);
                self.#ident.read(&bytes[pos..(pos+Self::SIZES[#i])]);
            }}
        });
    let struct_types = tm_value_struct.fields.iter().map(|f| &f.ty);
    let struct_len = tm_value_struct.fields.len();
    quote! {
        impl #type_name {
            const SIZES: [usize; #struct_len] = [#(<#struct_types as TMValue>::BYTE_SIZE),*];
            const fn get_pos(index: usize) -> usize {
                let mut len = 0;
                let mut i = 0;
                while i < index {
                    len += Self::SIZES[i];
                    i += 1;
                }
                len
            }
        }
        impl DynTMValue for #type_name {
            fn read(&mut self, bytes: &[u8]) {
                #(#struct_type_parsers)*
            }
            fn write(&self, mem: &mut [u8]) {
                #(#struct_byte_parsers)*
            }
            fn type_name(&self) -> &str {
                todo!()
            }
        }
        impl TMValue for #type_name {
            const BYTE_SIZE: usize = Self::get_pos(Self::SIZES.len());
        }
    }
}
