use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};

use crate::{generics::extract_angle_group, ident, is_ident};

pub fn auto_leaf_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut output = TokenStream::new();

    loop {
        let Some(token) = tokens.next() else {
            break;
        };

        if !is_ident(&token, "impl") {
            return error!(token.span(), "expected `impl`");
        }

        let generics = match tokens.peek() {
            Some(TokenTree::Punct(punct)) if *punct == '<' => {
                let _ = tokens.next();
                extract_angle_group(&mut tokens)
            }
            _ => None,
        };

        let Some(TokenTree::Ident(trait_name)) = tokens.next() else {
            return error!(token.span(), "expected trait name after `impl`");
        };

        let Some(token) = tokens.next() else {
            return error!(token.span(), "expected `for` after trait name");
        };

        // interpret paths here

        if !is_ident(&token, "for") {
            return error!(token.span(), "expected `for` after trait name");
        }

        let mut ty = TokenStream::new();

        let body = loop {
            let Some(token) = tokens.next() else {
                return error!(
                    trait_name.span(),
                    "expected `{{ ... }}` after implementation type"
                );
            };

            match token {
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    break group;
                }

                token => {
                    ty.extend([token]);
                }
            }
        };

        let inner_name = Ident::new(&format!("__auto_{}_inner", trait_name), trait_name.span());

        output.extend([ident("impl", trait_name.span())]);

        if let Some(generics) = generics {
            output.extend(generics);
        }

        output.extend([
            TokenTree::Ident(inner_name),
            ident("for", trait_name.span()),
        ]);

        output.extend(ty);

        output.extend([TokenTree::Group(body)]);
    }

    output
}
