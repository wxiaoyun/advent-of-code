use quote::{ToTokens, quote};
use syn::{ExprClosure, Token, parse::Parse};

pub struct RayonFns(Vec<ExprClosure>);

impl Parse for RayonFns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let closure = input.parse::<ExprClosure>()?;
        let mut closures = vec![closure];

        while input.parse::<Token![,]>().is_ok() {
            let closure = input.parse::<ExprClosure>()?;
            closures.push(closure);
        }

        Ok(Self(closures))
    }
}

impl ToTokens for RayonFns {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.0.is_empty() {
            tokens.extend(quote! { () });
            return;
        }

        if self.0.len() == 1 {
            let closure = &self.0[0];
            tokens.extend(quote! { (#closure)() });
            return;
        }

        fn helper(
            closures: &[ExprClosure],
            current_path: proc_macro2::TokenStream,
            leaf_paths: &mut Vec<proc_macro2::TokenStream>,
        ) -> proc_macro2::TokenStream {
            match closures.len() {
                0 => proc_macro2::TokenStream::new(),
                1 => {
                    let closure = &closures[0];
                    leaf_paths.push(current_path);
                    quote! { #closure }
                }
                n => {
                    let m = n / 2;

                    let left_path = quote! { #current_path.0 };
                    let right_path = quote! { #current_path.1 };

                    let left_rhs = helper(&closures[0..m], left_path, leaf_paths);
                    let right_rhs = helper(&closures[m..], right_path, leaf_paths);

                    let left_expr = if m > 1 {
                        quote! { || #left_rhs }
                    } else {
                        quote! { #left_rhs }
                    };
                    let right_expr = if n - m > 1 {
                        quote! { || #right_rhs }
                    } else {
                        quote! { #right_rhs }
                    };

                    quote! { ::rayon::join(#left_expr, #right_expr) }
                }
            }
        }

        let mut leaf_paths = Vec::new();
        let root_path = quote! { _joined };
        let right_expr = helper(&self.0, root_path, &mut leaf_paths);

        tokens.extend(quote! {
            {
                let _joined = #right_expr;
                (#(#leaf_paths),*)
            }
        });
    }
}
