//! Actual macro code
//!

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, NestedMeta};

pub(crate) enum FnTemplate {
    After,
    Before,
}

pub(crate) struct FnArgs {
    call: Option<syn::Expr>,
}

/// Args passed to the proc macro
impl FnArgs {
    pub fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut call = None;

        for arg in args {
            match arg {
                // arg is a literal
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                    return Err(syn::Error::new_spanned(lit, "Unsupported value"));
                }
                NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                                                          path,
                                                          lit: syn::Lit::Str(lit_str),
                                                          ..
                                                      })) => {
                    if path.is_ident("call") {
                        match lit_str.parse() {
                            Ok(lit) => { call = Some(lit) },
                            Err(e) => return Err(syn::Error::new_spanned(lit_str, format!("Can't parse call value: {}", e))),
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            path,
                            "Unknown identifier. Available: 'call'",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(arg, "Unknown attribute."));
                }
            }
        }

        Ok(Self {
            call,
        })
    }
}

/// Helper parsing attribute args
pub(crate) fn parse_args(args: AttributeArgs) -> FnArgs {
    match FnArgs::new(args) {
        Ok(args) => args,
        Err(_) => panic!("Can't parse args"),
    }
}

pub(crate) fn get_altered_fn(current_fn: ItemFn, args: FnArgs, template: FnTemplate) -> TokenStream {
    let fn_vis = current_fn.vis;
    let fn_block = current_fn.block;

    // Function signature
    let fn_sig = current_fn.sig;
    // Additional outer attributes to the function (passed via another proc macro)
    let fn_attrs = current_fn.attrs;
    // Function name
    let fn_name = &fn_sig.ident;
    // Generic params for function
    let fn_generics = &fn_sig.generics;
    // Additional function parameters
    let fn_args = &fn_sig.inputs;
    // Is function async?
    let fn_async = &fn_sig.asyncness;
    let fn_output = &fn_sig.output;

    // Get auth args from attribute args
    let call = args
        .call
        .as_ref()
        .map(|t| t.to_token_stream())
        .unwrap_or(
            syn::Error::new_spanned(
                "call",
                "Argument 'call' can't be parsed",
            ).to_compile_error()
        );

    match template {
        FnTemplate::After => {
            quote! {
                #(#fn_attrs)*
                #fn_vis #fn_async fn #fn_name #fn_generics(
                    #fn_args
                ) #fn_output {
                    #fn_block
                    #call();
                }
            }
        },
        FnTemplate::Before => {
            quote! {
                #(#fn_attrs)*
                #fn_vis #fn_async fn #fn_name #fn_generics(
                    #fn_args
                ) #fn_output {
                    let _before_values = #call();
                    #fn_block
                }
            }
        },
    }
}
