use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};

use crate::is_ident;

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

        let Some(TokenTree::Ident(trait_name)) = tokens.next() else {
            return error!(token.span(), "expected trait name after `impl`");
        };

        let Some(token) = tokens.next() else {
            return error!(token.span(), "expected `for` after trait name");
        };

        if !is_ident(&token, "for") {
            return error!(token.span(), "expected `for` after trait name");
        }

        let mut ty = TokenStream::new();

        loop {
            let Some(token) = tokens.next() else {
                return error!(token.span(), "expected `{{ ... }}` after implementation type");
            };

            match token {
                TokenTree::Group(group)
                    if group.delimiter() == Delimiter::Brace =>
                {
                    break;
                }

                token => {
                    ty.extend([token]);
                }
            }
        }

        emit_impl(&mut output, &trait_name, ty);
    }

    output
}

fn emit_impl(
    output: &mut TokenStream,
    trait_name: &Ident,
    ty: TokenStream,
) {
    let inner_name = Ident::new(
        &format!("__auto_{}_inner", trait_name),
        trait_name.span(),
    );

    output.extend([
        TokenTree::Ident(Ident::new("impl", trait_name.span())),
        TokenTree::Ident(inner_name),
        TokenTree::Ident(Ident::new("for", trait_name.span())),
    ]);

    output.extend(ty);

    output.extend([TokenTree::Group(Group::new(
        Delimiter::Brace,
        TokenStream::new(),
    ))]);
}
