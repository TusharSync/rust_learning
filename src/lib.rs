// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields};

// // #[proc_macro_derive(AutoDisplay)]
// pub fn auto_display_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let expanded = match input.data {
//         Data::Struct(data_struct) => {
//             match data_struct.fields {
//                 Fields::Named(fields_named) => {
//                     let field_names = fields_named.named.iter().map(|f| {
//                         let field_name = &f.ident;
//                         quote! {
//                             write!(f, "{}: {:?}, ", stringify!(#field_name), &self.#field_name)?;
//                         }
//                     });

//                     quote! {
//                         impl std::fmt::Display for #name {
//                             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                                 write!(f, "{} {{ ", stringify!(#name))?;
//                                 #(#field_names)*
//                                 write!(f, "}}")
//                             }
//                         }
//                     }
//                 }
//                 _ => panic!("AutoDisplay only supports structs with named fields"),
//             }
//         }
//         _ => panic!("AutoDisplay only supports structs"),
//     };

//     TokenStream::from(expanded)
// }
