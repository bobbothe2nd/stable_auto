use proc_macro::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};

use crate::{generics::extract_where_clause, ident, is_ident, punct, stable_auto_path};

use core::iter::{FromIterator, Peekable};

pub fn auto_trait_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut output = TokenStream::new();

    loop {
        let Some(mut token) = tokens.next() else {
            return error!(Span::call_site(), "expected a trait declaration");
        };

        let mut unsafe_keyword = None;

        if is_ident(&token, "unsafe") {
            output.extend([token.clone()]);
            unsafe_keyword = Some(token);
            let Some(trait_keyword) = tokens.next() else {
                return error!(Span::call_site(), "expected a trait declaration");
            };
            token = trait_keyword;
        }

        if !is_ident(&token, "trait") {
            output.extend([token]);
            continue;
        }

        let trait_name = match tokens.next() {
            Some(TokenTree::Ident(trait_name)) => trait_name,
            Some(token) => {
                return error!(token.span(), "expected trait name after `trait`");
            }
            None => {
                return error!(token.span(), "expected trait name after `trait`");
            }
        };

        if matches!(
            tokens.peek(),
            Some(TokenTree::Punct(punct)) if *punct == '<'
        ) {
            return error!(trait_name.span(), "auto traits cannot have generics");
        }

        let supertraits = extract_supertraits(&mut tokens).unwrap_or_default();

        let where_clause = extract_where_clause(&mut tokens).unwrap_or_default();

        token.set_span(trait_name.span());

        let inner_name = format!("__auto_{trait_name}_inner");
        let recursive_name = format!("__auto_{trait_name}_recursive");

        output.extend([token, TokenTree::Ident(trait_name.clone())]);

        if !supertraits.is_empty() {
            output.extend([punct(':', Spacing::Alone)]);
            output.extend(supertraits.clone());
        }

        output.extend(where_clause.clone());

        output.extend([empty_body()]);

        let mut generated = TokenStream::new();

        internal_trait(&mut generated, &unsafe_keyword, &inner_name);

        generated.extend(all_impl(&unsafe_keyword, &inner_name));
        generated.extend(leaf_impl(&unsafe_keyword, &recursive_name, &inner_name));
        generated.extend(combined_impl(&unsafe_keyword, &recursive_name, &inner_name));

        internal_trait(&mut generated, &unsafe_keyword, &recursive_name);

        generated.extend(recursive_impl(
            &unsafe_keyword,
            ident(&recursive_name, trait_name.span()),
            &inner_name,
        ));

        generated.extend(marker_impl(
            &unsafe_keyword,
            TokenTree::Ident(trait_name),
            &recursive_name,
            &supertraits,
            &where_clause,
        ));

        output.extend(generated);

        return output;
    }
}

fn internal_trait(
    generated: &mut TokenStream,
    unsafe_keyword: &Option<TokenTree>,
    inner_name: &str,
) {
    generated.extend([
        punct('#', Spacing::Alone),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter([
                ident("allow", Span::call_site()),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([ident("non_camel_case_types", Span::call_site())]),
                )),
            ]),
        )),
    ]);

    generated.extend([
        ident("pub", Span::call_site()),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(ident("crate", Span::call_site())),
        )),
    ]);

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| generated.extend([unsafe_keyword.clone()]));

    generated.extend([
        ident("trait", Span::call_site()),
        ident(inner_name, Span::call_site()),
        empty_body(),
    ]);
}

fn empty_body() -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new()))
}

fn recursive_impl(
    unsafe_keyword: &Option<TokenTree>,
    trait_name: TokenTree,
    inner_name: &str,
) -> TokenStream {
    let mut output = TokenStream::new();

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| output.extend([unsafe_keyword.clone()]));

    output.extend([
        ident("impl", Span::call_site()),
        generic_t(),
        trait_name.clone(),
        ident("for", Span::call_site()),
        ident("T", Span::call_site()),
        ident("where", Span::call_site()),
    ]);

    output.extend([ident("T", Span::call_site()), punct(':', Spacing::Alone)]);
    output.extend(stable_auto_path("Auto"));
    output.extend([punct(',', Spacing::Alone)]);

    output.extend([
        ident("T", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        punct(':', Spacing::Alone),
    ]);
    output.extend(stable_auto_path("Auto"));
    output.extend([punct(',', Spacing::Alone)]);

    let mut lhs = TokenStream::new();

    lhs.extend([
        punct('<', Spacing::Alone),
        ident("T", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        ident("as", Span::call_site()),
    ]);

    lhs.extend(stable_auto_path("Auto"));

    lhs.extend([
        punct('>', Spacing::Alone),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(inner_name, trait_name.span()),
    ]);

    output.extend(lhs);
    output.extend([punct(',', Spacing::Alone)]);

    output.extend([empty_body()]);

    output
}

fn marker_impl(
    unsafe_keyword: &Option<TokenTree>,
    trait_name: TokenTree,
    inner_name: &str,
    supertraits: &TokenStream,
    where_clause: &TokenStream,
) -> TokenStream {
    let mut output = TokenStream::new();

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| output.extend([unsafe_keyword.clone()]));

    output.extend([
        ident("impl", Span::call_site()),
        generic_t(),
        trait_name.clone(),
        ident("for", Span::call_site()),
        ident("T", Span::call_site()),
        ident("where", Span::call_site()),
    ]);

    output.extend([
        ident("Self", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(inner_name, trait_name.span()),
    ]);

    if !supertraits.is_empty() {
        output.extend([punct('+', Spacing::Alone)]);

        output.extend(supertraits.clone());
    }

    output.extend([punct(',', Spacing::Alone)]);

    if !where_clause.is_empty() {
        let predicates = where_clause.clone().into_iter().skip(1);

        output.extend(predicates);
    }

    output.extend([empty_body()]);

    output
}

fn all_impl(unsafe_keyword: &Option<TokenTree>, inner_name: &str) -> TokenStream {
    let mut output = TokenStream::new();

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| output.extend([unsafe_keyword.clone()]));

    output.extend([
        ident("impl", Span::call_site()),
        generic_t(),
        ident(inner_name, Span::call_site()),
        ident("for", Span::call_site()),
    ]);

    output.extend(stable_auto_path("AllImpl"));

    output.extend([
        punct('<', Spacing::Alone),
        ident("T", Span::call_site()),
        punct('>', Spacing::Alone),
        ident("where", Span::call_site()),
        ident("T", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(inner_name, Span::call_site()),
        punct(',', Spacing::Alone),
        empty_body(),
    ]);

    output
}

fn leaf_impl(
    unsafe_keyword: &Option<TokenTree>,
    trait_name: &str,
    inner_name: &str,
) -> TokenStream {
    let mut output = TokenStream::new();

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| output.extend([unsafe_keyword.clone()]));

    output.extend([
        ident("impl", Span::call_site()),
        generic_t(),
        ident(inner_name, Span::call_site()),
        ident("for", Span::call_site()),
    ]);

    output.extend(stable_auto_path("Leaf"));

    output.extend([
        punct('<', Spacing::Alone),
        ident("T", Span::call_site()),
        punct('>', Spacing::Alone),
        ident("where", Span::call_site()),
        ident("T", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(trait_name, Span::call_site()),
        punct(',', Spacing::Alone),
        empty_body(),
    ]);

    output
}

fn combined_impl(
    unsafe_keyword: &Option<TokenTree>,
    trait_name: &str,
    inner_name: &str,
) -> TokenStream {
    let mut output = TokenStream::new();

    unsafe_keyword
        .iter()
        .for_each(|unsafe_keyword| output.extend([unsafe_keyword.clone()]));

    output.extend([
        ident("impl", Span::call_site()),
        punct('<', Spacing::Alone),
        ident("A", Span::call_site()),
        punct(',', Spacing::Alone),
        ident("B", Span::call_site()),
        punct('>', Spacing::Alone),
        ident(inner_name, Span::call_site()),
        ident("for", Span::call_site()),
    ]);

    output.extend(stable_auto_path("Combined"));

    output.extend([
        punct('<', Spacing::Alone),
        ident("A", Span::call_site()),
        punct(',', Spacing::Alone),
        ident("B", Span::call_site()),
        punct('>', Spacing::Alone),
    ]);

    output.extend([ident("where", Span::call_site())]);

    output.extend([ident("A", Span::call_site()), punct(':', Spacing::Alone)]);
    output.extend(stable_auto_path("Auto"));
    output.extend([punct(',', Spacing::Alone)]);

    output.extend([ident("B", Span::call_site()), punct(':', Spacing::Alone)]);
    output.extend(stable_auto_path("Auto"));
    output.extend([punct(',', Spacing::Alone)]);

    output.extend([
        ident("A", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(trait_name, Span::call_site()),
        punct(',', Spacing::Alone),
    ]);

    output.extend([
        ident("B", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        punct(':', Spacing::Alone),
        ident(trait_name, Span::call_site()),
        punct(',', Spacing::Alone),
    ]);

    output.extend([empty_body()]);

    output
}

fn generic_t() -> TokenTree {
    TokenTree::Group(Group::new(
        Delimiter::None,
        TokenStream::from_iter([
            punct('<', Spacing::Alone),
            ident("T", Span::call_site()),
            punct('>', Spacing::Alone),
        ]),
    ))
}

pub fn extract_supertraits<I>(tokens: &mut Peekable<I>) -> Option<TokenStream>
where
    I: Iterator<Item = TokenTree>,
{
    match tokens.peek() {
        Some(TokenTree::Punct(punct)) if *punct == ':' => {}
        _ => return None,
    }

    tokens.next()?;

    let mut supertraits = TokenStream::new();
    let mut depth = 0;

    while let Some(token) = tokens.peek() {
        match token {
            TokenTree::Punct(punct) if *punct == '<' => {
                depth += 1;
            }

            TokenTree::Punct(punct) if *punct == '>' && depth > 0 => {
                depth -= 1;
            }

            TokenTree::Ident(ident) if ident.to_string() == "where" && depth == 0 => {
                break;
            }

            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace && depth == 0 => {
                break;
            }

            TokenTree::Punct(punct) if *punct == ';' && depth == 0 => {
                break;
            }

            _ => {}
        }

        supertraits.extend([tokens.next()?]);
    }

    Some(supertraits)
}
