extern crate proc_macro;

use proc_macro::{Punct, Spacing, Span, TokenStream, TokenTree};

macro_rules! try_stream {
    ($stream:expr) => {
        match $stream {
            Ok(stream) => stream,
            Err(stream) => return stream,
        }
    };
}

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

mod fields;
mod generics;

mod auto_leaf;

/// Transforms a clean `impl Marker` into the uglier internal implementation for auto traits.
///
/// Use this for defining the base cases for a marker trait on foreign types.
#[proc_macro]
pub fn auto_leaf(input: TokenStream) -> TokenStream {
    auto_leaf::auto_leaf_inner(input)
}

mod auto_trait;

/// Defines an auto trait, to be implemented on every auto type which has only explicitly defined base case fields (must be a marker trait).
///
/// # Safety
///
/// For any unsafe auto trait, its own safety preconditions must be satisfied by any type made up of other types which implement that trait.
///
/// # Codegen
///
/// ```rust
/// # use stable_auto::*;
/// trait Marker {}
///
/// impl __auto_Marker_inner for u16 {}
/// impl __auto_Marker_inner for u32 {}
/// impl __auto_Marker_inner for usize {}
///
/// impl<T> Marker for T
/// where
///     T: Auto,
///     T::Fields: __auto_Marker_inner,
/// {
/// }
///
/// #[allow(non_camel_case_types)]
/// trait __auto_Marker_inner {}
///
/// impl<T: __auto_Marker_inner> __auto_Marker_inner for AllImpl<T> {}
///
/// impl<T: Marker> __auto_Marker_inner for Leaf<T> {}
///
/// impl<A, B> __auto_Marker_inner for (A, B)
/// where
///     A: Auto,
///     B: Auto,
///     A::Fields: Marker,
///     B::Fields: Marker,
/// {
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return error!(Span::call_site(), "expected no attributes");
    }

    auto_trait::auto_trait_inner(item)
}

mod auto_opaque;

/// Implements `Auto` for a type as opaque to the trait solver.
///
/// This will implement auto traits only if they explicitly implement it. Useful for [`PhantomPinned`](core::marker::PhantomPinned)-like markers.
///
/// # Codegen
///
/// ```rust
/// # use stable_auto::*;
/// struct ImplAuto1;
///
/// unsafe impl Auto for ImplAuto1 {
///     type Fields = AllImpl<u32>;
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return error!(Span::call_site(), "expected no attributes");
    }

    auto_opaque::auto_opaque_inner(item)
}

mod derive_auto;

/// Derives an auto implementation from the acutal field types.
///
/// This is usually the macro to safely implement `Auto`.
///
/// # Codegen
///
/// ```rust
/// # use stable_auto::*;
/// struct ImplAuto1 {
///     _a: u32,
///     _b: u16,
/// }
///
/// unsafe impl Auto for ImplAuto1 {
///     type Fields = (Leaf<u32>, Leaf<u16>);
/// }
/// ```
#[proc_macro_derive(Auto)]
pub fn derive_auto(input: TokenStream) -> TokenStream {
    derive_auto::derive_auto_inner(input)
}
