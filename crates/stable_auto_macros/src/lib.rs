use proc_macro::{TokenStream, TokenTree};

macro_rules! error {
    ($span:expr, $($args:tt)*) => {{
        let msg = ::std::format!($($args)*);

        let mut tokens = ::proc_macro::TokenStream::new();

        tokens.extend([
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Joint)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)),
            ::proc_macro::TokenTree::Ident(::proc_macro::Ident::new("core", $span)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Joint)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)),
            ::proc_macro::TokenTree::Ident(::proc_macro::Ident::new("compile_error", $span)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new('!', ::proc_macro::Spacing::Alone)),
        ]);

        let args = ::proc_macro::TokenStream::from(::proc_macro::TokenTree::Literal(::proc_macro::Literal::string(&msg)));

        tokens.extend([
            ::proc_macro::TokenTree::Group(::proc_macro::Group::new(::proc_macro::Delimiter::Parenthesis, args)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(';', ::proc_macro::Spacing::Alone)),
        ]);

        tokens
    }};
}

fn is_ident(token: &TokenTree, expected: &str) -> bool {
    matches!(
        token,
        TokenTree::Ident(ident) if ident.to_string() == expected
    )
}

mod auto_leaf;

#[proc_macro]
pub fn auto_leaf(input: TokenStream) -> TokenStream {
    auto_leaf::auto_leaf_inner(input)
}
