extern crate proc_macro;
use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitInt, Token, Ident, token, bracketed, PathSegment, PathArguments};

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

        // Create the path for use statement
        let mut segments: Punctuated<PathSegment, token::PathSep> = Punctuated::new();
        segments.push(PathSegment {
            ident: Ident::new("solution", Span::call_site().into()),
            arguments: PathArguments::None,
        });
        segments.push(PathSegment {
            ident: Ident::new(format!("day_{}", day).as_str(), Span::call_site().into()),
            arguments: PathArguments::None,
        });
        segments.push(PathSegment {
            ident: Ident::new(format!("Day{}", day).as_str(), Span::call_site().into()),
            arguments: PathArguments::None,
        });

        let path = syn::Path {
            leading_colon: None,
            segments,
        };

        let quote = quote! {
            #day => {
                let solution = #path::new(#file);
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

