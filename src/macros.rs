extern crate proc_macro;
use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitInt,  Token, Ident, token, bracketed};

// [1, 2, 3]
struct DayList {
    _bracket_token: token::Bracket,
    data: Punctuated<LitInt, Token![,]>,
}

impl Parse for DayList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _bracket_token: bracketed!(content in input),
            data: content.parse_terminated(LitInt::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn day_handler(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DayList);

    let mut quotes = vec![];
    for lit_int in input.data.into_iter() {
        let day = lit_int.base10_parse::<u8>().unwrap();
        let file = format!("../etc/day_{}.txt", day);
        let struct_ident = Ident::new(format!("Day{}", day).as_str(), Span::call_site().into());

        let quote = quote! {
            #day => {
                let solution = #struct_ident::new(#file);
                let answer = match part {
                    1 => solution.part_1(),
                    2 => solution.part_2(),
                    _ => panic!("Unrecognized part"),
                };
                println!("Answer: {}", answer);
            },
        };
        quotes.push(quote);
    }

    quote! {
        match day {
            #(#quotes)*
            _ => panic!("Unrecognized day"),
        }
    }.into()
}

