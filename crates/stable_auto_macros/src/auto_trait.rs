use proc_macro::{Delimiter, Group, Ident, Spacing, Span, TokenStream, TokenTree};

use crate::{ident, is_ident, punct};

pub fn auto_trait_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut output = TokenStream::new();

    loop {
        let Some(token) = tokens.next() else {
            return error!(Span::call_site(), "expected a trait declaration");
        };

        if !is_ident(&token, "trait") {
            output.extend([token]);
            continue;
        }

        let trait_keyword = token;

        let Some(TokenTree::Ident(trait_name)) = tokens.next() else {
            return error!(trait_keyword.span(), "expected trait name after `trait`");
        };

        let inner_name = format!("__auto_{trait_name}_inner");

        output.extend([
            trait_keyword,
            TokenTree::Ident(trait_name.clone()),
            empty_body(),
        ]);

        let mut generated = TokenStream::new();

        generated.extend(marker_impl(&trait_name, &inner_name));

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
            ident("trait", Span::call_site()),
            ident(&inner_name, Span::call_site()),
            empty_body(),
        ]);

        generated.extend(all_impl(&inner_name));

        generated.extend(leaf_impl(&trait_name, &inner_name));

        generated.extend(tuple_impl(&trait_name, &inner_name));

        output.extend(generated);

        return output;
    }
}

fn empty_body() -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new()))
}

fn marker_impl(trait_name: &Ident, inner_name: &str) -> TokenStream {
    let mut output = TokenStream::new();

    output.extend([
        ident("impl", Span::call_site()),
        generic_t(),
        TokenTree::Ident(trait_name.clone()),
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
        ident(inner_name, Span::call_site()),
    ]);

    output.extend(lhs);
    output.extend([punct(',', Spacing::Alone)]);

    output.extend([empty_body()]);

    output
}

fn all_impl(inner_name: &str) -> TokenStream {
    let mut output = TokenStream::new();

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

fn leaf_impl(trait_name: &Ident, inner_name: &str) -> TokenStream {
    let mut output = TokenStream::new();

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
        TokenTree::Ident(trait_name.clone()),
        punct(',', Spacing::Alone),
        empty_body(),
    ]);

    output
}

fn tuple_impl(trait_name: &Ident, inner_name: &str) -> TokenStream {
    let mut output = TokenStream::new();

    output.extend([
        ident("impl", Span::call_site()),
        TokenTree::Group(Group::new(
            Delimiter::None,
            TokenStream::from_iter([
                punct('<', Spacing::Alone),
                ident("A", Span::call_site()),
                punct(',', Spacing::Alone),
                ident("B", Span::call_site()),
                punct('>', Spacing::Alone),
            ]),
        )),
        ident(inner_name, Span::call_site()),
        ident("for", Span::call_site()),
    ]);

    output.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter([
            ident("A", Span::call_site()),
            punct(',', Spacing::Alone),
            ident("B", Span::call_site()),
        ]),
    ))]);

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
        TokenTree::Ident(trait_name.clone()),
        punct(',', Spacing::Alone),
    ]);

    output.extend([
        ident("B", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("Fields", Span::call_site()),
        punct(':', Spacing::Alone),
        TokenTree::Ident(trait_name.clone()),
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

fn stable_auto_path(name: &str) -> TokenStream {
    TokenStream::from_iter([
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident("stable_auto", Span::call_site()),
        punct(':', Spacing::Joint),
        punct(':', Spacing::Alone),
        ident(name, Span::call_site()),
    ])
}
