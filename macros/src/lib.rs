///! Macros for the Embedded Power Sequencing trait
extern crate proc_macro;

use quote::quote;
use std::ops::DerefMut;
use syn::visit_mut::{self, VisitMut};

struct PowerState {
    prefix: String,
}

impl PowerState {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: format!("{}_", prefix),
        }
    }
}

impl VisitMut for PowerState {
    fn visit_signature_mut(&mut self, signature: &mut syn::Signature) {
        visit_mut::visit_signature_mut(self, signature);
        signature.ident = quote::format_ident!("{}{}", self.prefix, signature.ident);
    }

    fn visit_expr_method_call_mut(&mut self, method_call: &mut syn::ExprMethodCall) {
        visit_mut::visit_expr_method_call_mut(self, method_call);
        method_call.method = quote::format_ident!("{}{}", self.prefix, method_call.method);
    }

    fn visit_expr_call_mut(&mut self, call: &mut syn::ExprCall) {
        visit_mut::visit_expr_call_mut(self, call);

        match call.func.deref_mut() {
            syn::Expr::Path(expr) => {
                let method_name = expr.path.segments.last_mut().expect("must have a segment");

                if method_name.ident != quote::format_ident!("Ok") {
                    method_name.ident = quote::format_ident!("{}{}", self.prefix, method_name.ident);
                }
            }
            _ => {}
        }
    }
}

#[proc_macro_attribute]
pub fn power_state(_args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::TraitItemFn = syn::parse2(item.into()).unwrap();

    let mut pre = PowerState::new("pre");
    let mut post = PowerState::new("post");

    let mut pre_input = input.clone();
    pre.visit_trait_item_fn_mut(&mut pre_input);

    let mut post_input = input.clone();
    post.visit_trait_item_fn_mut(&mut post_input);

    quote!(
    #pre_input
    #input
    #post_input
    )
    .into()
}
