use proc_macro::{Delimiter, Spacing, TokenStream, TokenTree};
use std::iter::Peekable;

use crate::punct;

pub fn extract_generics<I>(
    iter: &mut Peekable<I>,
) -> Option<(TokenStream, TokenStream, TokenStream)>
where
    I: Iterator<Item = TokenTree>,
{
    match iter.peek() {
        Some(TokenTree::Punct(p)) if *p == '<' => {}
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
            Some(TokenTree::Punct(p)) if *p == '>'
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
            Some(TokenTree::Punct(p)) if *p == ',' => {
                iter.next();
            }

            Some(TokenTree::Punct(p)) if *p == '>' => {}

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
    let mut angle_depth = 0usize;
    let mut has_default = false;

    loop {
        let token = iter.peek()?;

        match token {
            TokenTree::Punct(p) if *p == '<' => {
                angle_depth += 1;

                let token = iter.next()?;
                original.extend([token.clone()]);

                if !has_default {
                    impl_generics.extend([token]);
                }
            }

            TokenTree::Punct(p) if *p == '>' => {
                if angle_depth == 0 {
                    return Some(());
                }

                angle_depth -= 1;

                let token = iter.next()?;
                original.extend([token.clone()]);

                if !has_default {
                    impl_generics.extend([token]);
                }
            }

            TokenTree::Punct(p) if *p == ',' && angle_depth == 0 => {
                return Some(());
            }

            TokenTree::Punct(p) if *p == '=' && angle_depth == 0 => {
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
    let mut tokens = TokenStream::new();

    for token in iter {
        match &token {
            TokenTree::Punct(p) if *p == '<' => {
                depth += 1;
            }

            TokenTree::Punct(p) if *p == '>' => {
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
    let mut angle_depth = 0;

    while let Some(token) = tokens.peek() {
        match token {
            TokenTree::Punct(p) if *p == '<' => {
                angle_depth += 1;
            }

            TokenTree::Punct(p) if *p == '>' && angle_depth > 0 => {
                angle_depth -= 1;
            }

            TokenTree::Group(group)
                if group.delimiter() == Delimiter::Brace && angle_depth == 0 =>
            {
                break;
            }

            TokenTree::Punct(p) if *p == ';' && angle_depth == 0 => {
                break;
            }

            _ => {}
        }

        where_tokens.extend([tokens.next()?]);
    }

    Some(where_tokens)
}
