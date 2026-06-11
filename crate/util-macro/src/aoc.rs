use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::{LitInt, Token, parse::Parse};

pub struct Days(Vec<LitInt>);

impl Parse for Days {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day = input.parse::<LitInt>()?;
        let mut days = vec![day];

        while input.parse::<Token![,]>().is_ok() {
            let day = input.parse::<LitInt>()?;
            days.push(day);
        }

        Ok(Self(days))
    }
}

impl ToTokens for Days {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        fn parse_lit_to_ident(lit: &LitInt) -> Ident {
            let day_num = lit.base10_parse::<u8>().unwrap();
            Ident::new(&format!("day{:02}", day_num), lit.span())
        }

        let module_idents: Vec<Ident> = self.0.iter().map(parse_lit_to_ident).collect();

        let match_arms: Vec<_> = self
            .0
            .iter()
            .flat_map(|lit| {
                let day_num = lit.base10_parse::<u8>().unwrap();
                let module_ident = Ident::new(&format!("day{:02}", day_num), lit.span());
                vec![
                    quote! {
                        (#day_num, 1) => #module_ident::part_one(input),
                    },
                    quote! {
                        (#day_num, 2) => #module_ident::part_two(input),
                    },
                ]
            })
            .collect();

        let tok = quote! {
            #(mod #module_idents;)*

            fn main() {
              let args = util::parse_args();
              let input = util::read_input_from_stdin();

              let result: i64 = match (args.day, args.part) {
                  #(#match_arms)*
                  _ => panic!("Day {}, part {} not implemented", args.day, args.part),
              };

              println!("{result}");
            }
        };

        tokens.extend(tok);
    }
}
