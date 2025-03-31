///! Macros for the Embedded Power Sequencing trait
extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, TraitItemFn};

#[proc_macro_attribute]
pub fn power_state(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = power_state_inner(args.into(), item.into());
    result.into()
}

fn power_state_inner(_args: TokenStream, item: TokenStream) -> TokenStream {
    let f: TraitItemFn = syn::parse2(item.clone()).unwrap();

    let fargs = f.sig.inputs.clone();
    let fout = &f.sig.output;
    let fname = &f.sig.ident;
    let fbody = f.default;
    let fsemi = f.semi_token;

    let pre_fname = Ident::new(&format!("pre_{}", fname), Span::call_site());
    let post_fname = Ident::new(&format!("post_{}", fname), Span::call_site());

    let result = quote! {
    async fn #pre_fname(#fargs) #fout {
        Ok(())
    }

    async fn #fname(#fargs) #fout #fbody #fsemi

    async fn #post_fname(#fargs) #fout {
        Ok(())
    }
    };

    result
}
