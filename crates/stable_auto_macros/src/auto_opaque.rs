use proc_macro::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};

use crate::{
    generics::{extract_generics, extract_where_clause},
    ident, punct,
};

use std::iter::FromIterator;

pub fn auto_opaque_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut output = TokenStream::new();

    loop {
        let Some(keyword) = tokens.next() else {
            return error!(Span::call_site(), "expected a type declaration");
        };

        if let TokenTree::Ident(ident) = &keyword {
            if !matches!(
                ident.to_string().as_str(),
                "struct" | "enum" | "union"
            ) {
                output.extend([keyword]);
                continue;
            }
        } else {
            output.extend([keyword]);
            continue;
        }

        let Some(TokenTree::Ident(mut name)) = tokens.next() else {
            return error!(keyword.span(), "expected type name after {keyword}");
        };
        name.set_span(Span::call_site());

        output.extend([keyword, TokenTree::Ident(name.clone())]);

        let generic_args = extract_generics(&mut tokens);

        if let Some((generics, _, _)) = &generic_args {
            output.extend(generics.clone());
        }

        let where_clause = extract_where_clause(&mut tokens).unwrap_or_default();

        output.extend(where_clause.clone());
        output.extend(tokens);

        let mut generated = TokenStream::new();

        generated.extend([
            ident("unsafe", Span::call_site()),
            ident("impl", Span::call_site()),
        ]);

        if let Some((_, generics, _)) = &generic_args {
            generated.extend(generics.clone());
        }

        generated.extend([
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("stable_auto", Span::call_site()),
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("Auto", Span::call_site()),
            ident("for", Span::call_site()),
            TokenTree::Ident(name),
        ]);

        if let Some((_, _, generics)) = generic_args {
            generated.extend(generics);
        }

        generated.extend(where_clause);

        let body = TokenStream::from_iter([
            ident("type", Span::call_site()),
            ident("Fields", Span::call_site()),
            punct('=', Spacing::Alone),
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("stable_auto", Span::call_site()),
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("AllImpl", Span::call_site()),
            punct('<', Spacing::Alone),
            ident("Self", Span::call_site()),
            punct('>', Spacing::Alone),
            punct(';', Spacing::Alone),
        ]);

        generated.extend([TokenTree::Group(Group::new(Delimiter::Brace, body))]);

        output.extend(generated);

        return output;
    }
}
