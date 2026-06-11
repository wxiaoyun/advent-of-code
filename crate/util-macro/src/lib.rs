use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod aoc;
mod rayon;

#[proc_macro]
pub fn aoc_main(days: TokenStream) -> TokenStream {
    let days = parse_macro_input!(days as aoc::Days);
    days.to_token_stream().into()
}

#[proc_macro]
pub fn rayon_join_all(closures: TokenStream) -> TokenStream {
    let closures = parse_macro_input!(closures as rayon::RayonFns);
    closures.to_token_stream().into()
}
