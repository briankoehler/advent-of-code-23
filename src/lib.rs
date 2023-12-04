extern crate proc_macro;
use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, LitInt, LitStr, Token, Ident, token, punctuated::Punctuated, parenthesized, bracketed};

// (1, Day1)
struct DayHandler {
    _paren_token: token::Paren,
    day: u8,
    _comma: Token![,],
    day_struct: Ident,
}

// [(1, Day1), (2, Day2)]
struct DayHandlerList {
    _bracket_token: token::Bracket,
    data: Punctuated<DayHandler, Token![,]>,
}

impl Parse for DayHandler {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren_token: parenthesized!(content in input),
            day: content.parse::<LitInt>()?.base10_parse::<u8>()?,
            _comma: content.parse()?,
            day_struct: content.parse()?,
        })
    }
}

impl Parse for DayHandlerList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _bracket_token: bracketed!(content in input),
            data: content.parse_terminated(DayHandler::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn day_handler(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DayHandlerList);

    let mut meta = vec![];
    for day in input.data.into_iter() {
        let file = LitStr::new(format!("../etc/day_{}.txt", day.day).as_str(), Span::call_site().into());
        meta.push((day.day, day.day_struct, file));
    }

    let mut quotes = vec![];
    for m in meta {
        let day = m.0;
        let day_struct = m.1;
        let file = m.2;
        let quote = quote! {
            #day => {
                let solution = #day_struct::new(#file);
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

