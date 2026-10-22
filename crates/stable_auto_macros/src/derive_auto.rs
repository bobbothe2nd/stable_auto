use proc_macro::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};

use std::iter::FromIterator;

use crate::{
    fields::{extract_enum_types, extract_named_fields, extract_tuple_fields}, generics::{extract_generics, extract_where_clause}, ident, punct,
};

pub fn derive_auto_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut output = TokenStream::new();

    loop {
        let Some(TokenTree::Ident(keyword)) = tokens.next() else {
            return error!(Span::call_site(), "expected a type declaration");
        };

        let Some(TokenTree::Ident(name)) = tokens.next() else {
            return error!(keyword.span(), "expected type name after keyword");
        };

        let generic_args = extract_generics(&mut tokens);

        let where_clause = extract_where_clause(&mut tokens).unwrap_or_default();

        let fields = match tokens.next() {
            Some(TokenTree::Group(fields)) => fields,
            _ => Group::new(Delimiter::None, TokenStream::new()),
        };

        let types = match keyword.to_string().as_str() {
            "struct" => {
                match fields.delimiter() {
                    Delimiter::Brace => try_stream!(extract_named_fields(fields.stream())),
                    Delimiter::Parenthesis => try_stream!(extract_tuple_fields(fields.stream())),
                    Delimiter::None => Vec::new(),
                    _ => unreachable!(),
                }
            }
            "enum" => try_stream!(extract_enum_types(fields.stream())),
            "union" => try_stream!(extract_named_fields(fields.stream())),
            _ => unreachable!(),
        };

        let fields_ty = make_fields_type(&types);

        let mut generated = TokenStream::new();

        generated.extend([
            ident("unsafe", Span::call_site()),
            ident("impl", Span::call_site()),
        ]);

        if let Some((_, generics, _)) = &generic_args {
            generated.extend(generics.clone());
        }

        generated.extend([
            ident("Auto", Span::call_site()),
            ident("for", Span::call_site()),
            TokenTree::Ident(name),
        ]);

        if let Some((_, _, generics)) = generic_args {
            generated.extend(generics);
        }

        generated.extend(where_clause);

        let mut body = TokenStream::new();

        body.extend([
            ident("type", Span::call_site()),
            ident("Fields", Span::call_site()),
            punct('=', Spacing::Alone),
        ]);

        body.extend(fields_ty);

        body.extend([punct(';', Spacing::Alone)]);

        generated.extend([TokenTree::Group(Group::new(Delimiter::Brace, body))]);

        output.extend(generated);

        return output;
    }
}

fn make_fields_type(types: &[TokenStream]) -> TokenStream {
    match types {
        [] => TokenStream::from_iter([
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("stable_auto", Span::call_site()),
            punct(':', Spacing::Joint),
            punct(':', Spacing::Alone),
            ident("AllImpl", Span::call_site()),
            punct('<', Spacing::Alone),
            ident("Self", Span::call_site()),
            punct('>', Spacing::Alone),
        ]),

        [ty] => make_leaf(ty.clone()),

        [first, rest @ ..] => {
            let mut result = make_leaf(first.clone());

            for ty in rest {
                let mut tuple = TokenStream::new();

                tuple.extend(result);

                tuple.extend([punct(',', Spacing::Alone)]);

                tuple.extend(make_leaf(ty.clone()));

                result =
                    TokenStream::from(TokenTree::Group(Group::new(Delimiter::Parenthesis, tuple)));
            }

            result
        }
    }
}

fn make_leaf(ty: TokenStream) -> TokenStream {
    let mut leaf = TokenStream::new();

    leaf.extend([
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("stable_auto", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Leaf", Span::call_site()),
        punct('<', Spacing::Alone),
    ]);

    leaf.extend(ty);

    leaf.extend([punct('>', Spacing::Alone)]);

    leaf
}
