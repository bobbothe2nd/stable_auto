use core::iter::Peekable;
use proc_macro::{Delimiter, Ident, Spacing, Span, TokenStream, TokenTree};

use crate::punct;

pub fn extract_generics<I>(
    iter: &mut Peekable<I>,
) -> Option<(TokenStream, TokenStream, TokenStream)>
where
    I: Iterator<Item = TokenTree>,
{
    match iter.peek() {
        Some(TokenTree::Punct(punct)) if *punct == '<' => {}
        _ => return None,
    }

    iter.next();

    let mut original = TokenStream::new();
    let mut params = TokenStream::new();
    let mut args = TokenStream::new();

    original.extend([punct('<', Spacing::Alone)]);
    params.extend([punct('<', Spacing::Alone)]);
    args.extend([punct('<', Spacing::Alone)]);

    let mut first = true;

    loop {
        if matches!(
            iter.peek(),
            Some(TokenTree::Punct(punct)) if *punct == '>'
        ) {
            iter.next();

            original.extend([punct('>', Spacing::Alone)]);
            params.extend([punct('>', Spacing::Alone)]);
            args.extend([punct('>', Spacing::Alone)]);

            return Some((original, params, args));
        }

        if !first {
            original.extend([punct(',', Spacing::Alone)]);
            params.extend([punct(',', Spacing::Alone)]);
            args.extend([punct(',', Spacing::Alone)]);
        }
        first = false;

        match iter.next()? {
            TokenTree::Punct(p) if p == '\'' => {
                let TokenTree::Ident(name) = iter.next()? else {
                    return None;
                };

                let apostrophe = TokenTree::Punct(p);

                original.extend([apostrophe.clone(), TokenTree::Ident(name.clone())]);

                params.extend([apostrophe, TokenTree::Ident(name.clone())]);

                args.extend([punct('\'', Spacing::Joint), TokenTree::Ident(name)]);

                copy_parameter_tail(iter, &mut original, &mut params)?;
            }

            TokenTree::Ident(keyword) if keyword.to_string() == "const" => {
                let TokenTree::Ident(name) = iter.next()? else {
                    return None;
                };

                original.extend([
                    TokenTree::Ident(keyword.clone()),
                    TokenTree::Ident(name.clone()),
                ]);

                params.extend([TokenTree::Ident(keyword), TokenTree::Ident(name.clone())]);

                args.extend([TokenTree::Ident(name)]);

                copy_parameter_tail(iter, &mut original, &mut params)?;
            }

            TokenTree::Ident(name) => {
                original.extend([TokenTree::Ident(name.clone())]);
                params.extend([TokenTree::Ident(name.clone())]);
                args.extend([TokenTree::Ident(name)]);

                copy_parameter_tail(iter, &mut original, &mut params)?;
            }

            _ => return None,
        }

        match iter.peek() {
            Some(TokenTree::Punct(punct)) if *punct == ',' => {
                iter.next();
            }

            Some(TokenTree::Punct(punct)) if *punct == '>' => {}

            _ => return None,
        }
    }
}

fn copy_parameter_tail<I>(
    iter: &mut Peekable<I>,
    original: &mut TokenStream,
    impl_generics: &mut TokenStream,
) -> Option<()>
where
    I: Iterator<Item = TokenTree>,
{
    let mut depth = 0usize;
    let mut has_default = false;

    loop {
        let token = iter.peek()?;

        match token {
            TokenTree::Punct(punct) if *punct == '<' => {
                depth += 1;

                let token = iter.next()?;
                original.extend([token.clone()]);

                if !has_default {
                    impl_generics.extend([token]);
                }
            }

            TokenTree::Punct(punct) if *punct == '>' => {
                if depth == 0 {
                    return Some(());
                }

                depth -= 1;

                let token = iter.next()?;
                original.extend([token.clone()]);

                if !has_default {
                    impl_generics.extend([token]);
                }
            }

            TokenTree::Punct(punct) if *punct == ',' && depth == 0 => {
                return Some(());
            }

            TokenTree::Punct(punct) if *punct == '=' && depth == 0 => {
                has_default = true;

                let token = iter.next()?;
                original.extend([token]);
            }

            _ => {
                let token = iter.next()?;

                original.extend([token.clone()]);

                if !has_default {
                    impl_generics.extend([token]);
                }
            }
        }
    }
}

pub fn extract_angle_group<I>(iter: &mut I) -> Option<TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    let mut depth = 1usize;
    let mut tokens = TokenStream::from(punct('<', Spacing::Alone));

    for token in iter {
        match &token {
            TokenTree::Punct(punct) if *punct == '<' => {
                depth += 1;
            }

            TokenTree::Punct(punct) if *punct == '>' => {
                depth -= 1;

                if depth == 0 {
                    tokens.extend([token]);

                    return Some(tokens);
                }
            }

            _ => {}
        }

        tokens.extend([token]);
    }

    None
}

pub fn extract_where_clause<I>(tokens: &mut Peekable<I>) -> Option<TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    match tokens.peek() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "where" => {}
        _ => return None,
    }

    let mut where_tokens = TokenStream::from(tokens.next()?);
    let mut depth = 0;

    while let Some(token) = tokens.peek() {
        match token {
            TokenTree::Punct(punct) if *punct == '<' => {
                depth += 1;
            }

            TokenTree::Punct(punct) if *punct == '>' && depth > 0 => {
                depth -= 1;
            }

            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace && depth == 0 => {
                break;
            }

            TokenTree::Punct(punct) if *punct == ';' && depth == 0 => {
                break;
            }

            _ => {}
        }

        where_tokens.extend([tokens.next()?]);
    }

    Some(where_tokens)
}

pub fn parse_path<I>(tokens: &mut Peekable<I>) -> Result<(TokenStream, Ident), TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    let Some(TokenTree::Ident(mut current)) = tokens.next() else {
        return Err(error!(
            tokens
                .peek()
                .map(TokenTree::span)
                .unwrap_or(Span::call_site()),
            "expected trait path after `impl`"
        ));
    };

    let mut path = TokenStream::new();

    while let Some(TokenTree::Punct(colon)) = tokens.peek() {
        if *colon != ':' {
            break;
        }

        let colon = tokens.next().unwrap();

        let Some(TokenTree::Punct(second_colon)) = tokens.next() else {
            return Err(error!(colon.span(), "expected `:` in trait path"));
        };

        if second_colon != ':' {
            return Err(error!(second_colon.span(), "expected `:` in trait path"));
        }

        let Some(TokenTree::Ident(segment)) = tokens.next() else {
            return Err(error!(
                second_colon.span(),
                "expected identifier after `::`"
            ));
        };

        path.extend([
            TokenTree::Ident(current),
            colon,
            TokenTree::Punct(second_colon),
        ]);

        current = segment;
    }

    Ok((path, current))
}
