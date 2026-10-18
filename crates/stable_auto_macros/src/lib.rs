use proc_macro::{Punct, Spacing, Span, TokenStream, TokenTree};

macro_rules! error {
    ($span:expr, $($args:tt)*) => {{
        let msg = ::std::format!($($args)*);

        let mut tokens = ::proc_macro::TokenStream::new();

        tokens.extend([
            $crate::punct(':', ::proc_macro::Spacing::Joint),
            $crate::punct(':', ::proc_macro::Spacing::Alone),
            $crate::ident("core", $span),
            $crate::punct(':', ::proc_macro::Spacing::Joint),
            $crate::punct(':', ::proc_macro::Spacing::Alone),
            $crate::ident("compile_error", $span),
            $crate::punct('!', ::proc_macro::Spacing::Alone),
        ]);

        let args = ::proc_macro::TokenStream::from(::proc_macro::TokenTree::Literal(::proc_macro::Literal::string(&msg)));

        tokens.extend([
            ::proc_macro::TokenTree::Group(::proc_macro::Group::new(::proc_macro::Delimiter::Parenthesis, args)),
            $crate::punct(';', ::proc_macro::Spacing::Alone),
        ]);

        tokens
    }};
}

fn ident(name: &str, span: proc_macro::Span) -> TokenTree {
    TokenTree::Ident(proc_macro::Ident::new(name, span))
}

fn punct(ch: char, spacing: Spacing) -> TokenTree {
    TokenTree::Punct(Punct::new(ch, spacing))
}

fn is_ident(token: &TokenTree, expected: &str) -> bool {
    matches!(
        token,
        TokenTree::Ident(ident) if ident.to_string() == expected
    )
}

pub(crate) mod generics;

mod auto_leaf;

#[proc_macro]
pub fn auto_leaf(input: TokenStream) -> TokenStream {
    auto_leaf::auto_leaf_inner(input)
}

mod auto_trait;

#[proc_macro_attribute]
pub fn auto_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return error!(Span::call_site(), "expected no attributes");
    }

    auto_trait::auto_trait_inner(item)
}

mod auto_opaque;

#[proc_macro_attribute]
pub fn auto_opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return error!(Span::call_site(), "expected no attributes");
    }

    auto_opaque::auto_opaque_inner(item)
}
