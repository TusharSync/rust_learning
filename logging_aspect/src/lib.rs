use proc_macro::TokenStream; // For defining the macro
use proc_macro2::TokenStream as TokenStream2; // For internal manipulation
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name: &syn::Ident = &input.sig.ident;
    let fn_block: &Box<syn::Block> = &input.block;
    let fn_sig: &syn::Signature = &input.sig;

    // Generate the modified function
    let expanded: TokenStream2 = quote! {
        #fn_sig {
            println!("Before executing function: {}", stringify!(#fn_name));
            let result = (|| #fn_block)();
            println!("After executing function: {}", stringify!(#fn_name));
            result
        }
    };

    // Convert TokenStream2 back to TokenStream
    expanded.into()
}
