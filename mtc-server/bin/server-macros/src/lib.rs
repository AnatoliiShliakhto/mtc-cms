extern crate proc_macro;

use ::proc_macro::TokenStream;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::syn::{
    ItemFn, Meta, Token, parse::Parser, parse_macro_input, parse_quote, punctuated::Punctuated,
};

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut result_token: TokenStream2 = Default::default();
    let mut check_permission_token: TokenStream2 = Default::default();

    let args_parsed = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(args)
        .unwrap();

    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = input;

    let stmts = &block.stmts;

    args_parsed.iter().for_each(|meta| {
        let path = meta.path();

        if path.is_ident("state") {
            sig.inputs.insert(0, parse_quote! {
                state: State<Arc<AppState>>
            });
        }

        if path.is_ident("session") {
            sig.inputs.insert(
                0,
                parse_quote! {
                    session: Session
                },
            );
        }

        if path.is_ident("result") {
            if let Ok(result) = meta.require_name_value() {
                let value = &result.value;

                result_token = quote! {
                    Ok(#value)
                }

            } else {
                result_token = quote! {
                    Ok(())
                }
            }
        }

        if path.is_ident("permission") {
            let permission = &meta
                .require_name_value()
                .expect("permission value must be set")
                .value;

            check_permission_token = quote! {
                session.has_permission(#permission).await?;
            };
        }
    });

    sig.output = parse_quote! { -> Result<impl ::axum::response::IntoResponse> };

    quote! {
        #(#attrs)* #vis #sig {
            #check_permission_token

            #(#stmts)*

            #result_token
        }
    }
        .into()
}
