#![feature(iter_intersperse)]
//mod beacon_macro;
mod tm_definition_macro_attribute;
mod tm_value_macro_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(TMValue)]
pub fn tm_value_macro_derive(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    
    // Build the trait implementation
    tm_value_macro_derive::impl_macro(ast).into()
}

// #[proc_macro]
// pub fn beacon(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();
// 
//     // Build the beacon definition and implementation
//     beacon_macro::impl_macro(ast).into()
// }


#[proc_macro_attribute]
pub fn tm_definition_macro_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    //let path = syn::parse_macro_input!(attr as syn::Path);

    // Build the beacon definition and implementation
    tm_definition_macro_attribute::impl_macro(ast).into()
}
