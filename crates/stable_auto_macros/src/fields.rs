use proc_macro::{Delimiter, Span, TokenStream, TokenTree};

pub fn extract_tuple_fields(
    fields: TokenStream,
) -> Result<Vec<TokenStream>, TokenStream> {
    let mut tokens = fields.into_iter().peekable();
    let mut types = Vec::new();

    while tokens.peek().is_some() {
        let ty = extract_field_type(&mut tokens)?;
        types.push(ty);

        if matches!(
            tokens.peek(),
            Some(TokenTree::Punct(punct)) if *punct == ','
        ) {
            tokens.next();
        } else {
            break;
        }
    }

    Ok(types)
}

pub fn extract_named_fields(
    fields: TokenStream,
) -> Result<Vec<TokenStream>, TokenStream> {
    let mut tokens = fields.into_iter().peekable();
    let mut types = Vec::new();

    while tokens.peek().is_some() {
        loop {
            let Some(token) = tokens.next() else {
                return Err(error!(
                    Span::call_site(),
                    "expected `:` after field"
                ));
            };

            if let TokenTree::Punct(punct) = &token
                && *punct == ':'
            {
                break;
            }
        }

        types.push(extract_field_type(&mut tokens)?);

        if matches!(
            tokens.peek(),
            Some(TokenTree::Punct(punct)) if *punct == ','
        ) {
            tokens.next();
        } else {
            break;
        }
    }

    Ok(types)
}

pub fn extract_field_type<I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<TokenStream, TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    let mut ty = TokenStream::new();
    let mut depth = 0usize;

    while let Some(token) = tokens.peek() {
        match token {
            TokenTree::Punct(punct) if *punct == '<' => {
                depth += 1;
                ty.extend([tokens.next().unwrap()]);
            }

            TokenTree::Punct(punct) if *punct == '>' && depth > 0 => {
                depth -= 1;
                ty.extend([tokens.next().unwrap()]);
            }

            TokenTree::Punct(punct) if *punct == ',' && depth == 0 => {
                break;
            }

            _ => {
                ty.extend([tokens.next().unwrap()]);
            }
        }
    }

    Ok(ty)
}

pub fn extract_enum_types(
    body: TokenStream,
) -> Result<Vec<TokenStream>, TokenStream> {
    let mut tokens = body.into_iter().peekable();
    let mut types = Vec::new();

    while tokens.peek().is_some() {
        let Some(TokenTree::Ident(_)) = tokens.next() else {
            return Err(error!(
                Span::call_site(),
                "expected enum variant"
            ));
        };

        match tokens.peek() {
            Some(TokenTree::Group(group))
                if group.delimiter() == Delimiter::Parenthesis =>
            {
                let group = match tokens.next().unwrap() {
                    TokenTree::Group(group) => group,
                    _ => unreachable!(),
                };

                types.extend(extract_tuple_fields(group.stream())?);
            }

            Some(TokenTree::Group(group))
                if group.delimiter() == Delimiter::Brace =>
            {
                let group = match tokens.next().unwrap() {
                    TokenTree::Group(group) => group,
                    _ => unreachable!(),
                };

                types.extend(extract_named_fields(group.stream())?);
            }

            _ => {}
        }

        if matches!(
            tokens.peek(),
            Some(TokenTree::Punct(punct)) if *punct == ','
        ) {
            tokens.next();
        }
    }

    Ok(types)
}
