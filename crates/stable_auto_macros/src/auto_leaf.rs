use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};

use crate::{
    generics::{extract_angle_group, parse_path},
    ident, is_ident,
};

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

        let (trait_path, trait_name) = match parse_path(&mut tokens) {
            Ok(path) => path,
            Err(error) => return error,
        };

        let Some(token) = tokens.next() else {
            return error!(token.span(), "expected `for` after trait name");
        };

        if !is_ident(&token, "for") {
            return error!(token.span(), "expected `for` after trait name");
        }

        let mut ty = TokenStream::new();
        let mut depth = 0;
        let mut arrow = false;

        let body = loop {
            let Some(token) = tokens.next() else {
                return error!(
                    trait_name.span(),
                    "expected `{{ ... }}` after implementation type"
                );
            };

            match token {
                TokenTree::Group(group) if depth == 0 && group.delimiter() == Delimiter::Brace => {
                    break group;
                }

                TokenTree::Punct(punct) if punct == '<' => {
                    depth += 1;
                    ty.extend([TokenTree::Punct(punct)]);
                }

                TokenTree::Punct(punct) if punct == '>' => {
                    if !arrow {
                        depth -= 1;
                    }
                    arrow = false;
                    ty.extend([TokenTree::Punct(punct)]);
                }

                TokenTree::Punct(punct) if punct == '-' => {
                    arrow = true;
                    ty.extend([TokenTree::Punct(punct)]);
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

        output.extend(trait_path);

        output.extend([
            TokenTree::Ident(inner_name),
            ident("for", trait_name.span()),
        ]);

        output.extend(ty);

        output.extend([TokenTree::Group(body)]);
    }

    output
}
