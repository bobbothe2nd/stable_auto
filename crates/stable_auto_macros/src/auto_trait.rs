use proc_macro::{Ident, Span, TokenStream, TokenTree};

use crate::{ident, is_ident};

pub fn auto_trait_inner(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    while let Some(token) = tokens.next() {
        if !is_ident(&token, "trait") {
            continue;
        }

        let Some(TokenTree::Ident(trait_name)) = tokens.next() else {
            return error!(token.span(), "expected trait name after `trait`");
        };

        let inner_name = Ident::new(&format!("__auto_{trait_name}_inner"), trait_name.span());

        let mut output = TokenStream::new();

        output.extend([
            ident("trait", token.span()),
            TokenTree::Ident(trait_name.clone()),
        ]);

        output.extend(tokens);

        output.extend(
            format!(
                "::stable_auto::__stable_auto_macro_inner!(@auto_trait {trait_name} {inner_name});"
            )
            .parse::<TokenStream>()
            .expect("generated auto_trait invocation must parse"),
        );

        return output;
    }

    error!(Span::call_site(), "expected a trait declaration")
}
