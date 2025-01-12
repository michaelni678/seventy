//! Helpers for expanding the `seventy` attribute proc-macro.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, Error, Fields, FieldsUnnamed, ItemStruct, Meta, Result, Token};

pub fn expand(metas: Punctuated<Meta, Token![,]>, item: ItemStruct) -> Result<TokenStream2> {
    let ident = &item.ident;

    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let unnamed = match &item.fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
        _ => return Err(Error::new_spanned(ident, "expected a newtype")),
    };

    if unnamed.len() != 1 {
        return Err(Error::new_spanned(ident, "expected a newtype"));
    }

    let inner = &unnamed[0].ty;

    let mut as_ref = false;
    let mut bypassable = false;
    let mut deref = false;
    let mut inherent = false;
    let mut unexposed = false;
    let mut try_from = false;

    let mut sanitizers = None;
    let mut validators = None;

    for meta in metas {
        if meta.path().is_ident("upgrades") {
            meta.require_list()?.parse_nested_meta(|meta| {
                if meta.path.is_ident("as_ref") {
                    as_ref = true;
                } else if meta.path.is_ident("bypassable") {
                    bypassable = true;
                } else if meta.path.is_ident("deref") {
                    deref = true;
                } else if meta.path.is_ident("inherent") {
                    inherent = true;
                } else if meta.path.is_ident("unexposed") {
                    unexposed = true;
                } else if meta.path.is_ident("try_from") {
                    try_from = true;
                } else {
                    return Err(meta.error("unrecognized upgrade"));
                }
                Ok(())
            })?;
        } else if meta.path().is_ident("sanitize") {
            let tokens = &meta.require_list()?.tokens;
            sanitizers = Some(quote!(#tokens));
        } else if meta.path().is_ident("validate") {
            let tokens = &meta.require_list()?.tokens;
            validators = Some(quote!(#tokens));
        } else {
            return Err(Error::new_spanned(meta, "unrecognized attribute"));
        }
    }

    let mut expansion = Vec::new();

    if !unexposed {
        expansion.push(quote! { #item });
    }

    expansion.push(quote! {
        impl #impl_generics ::seventy::core::Newtype for #ident #ty_generics #where_clause {
            type Inner = #inner;

            fn try_new(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner>
            where
                Self: ::seventy::core::Sanitizable + ::seventy::core::Validatable,
            {
                let mut inner = inner.into();

                <Self as ::seventy::core::Sanitizable>::sanitizer().sanitize(&mut inner);

                let is_valid = <Self as ::seventy::core::Validatable>::validator().validate(&inner);

                if is_valid {
                    Ok(Self(inner))
                } else {
                    Err(inner)
                }
            }

            fn to_inner(&self) -> &Self::Inner {
                &self.0
            }

            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }

        impl #impl_generics ::seventy::core::Sanitizable for #ident #ty_generics #where_clause {
            fn sanitizer() -> &'static dyn ::seventy::core::Sanitizer<Self::Inner> {
                static SANITIZER: ::std::sync::LazyLock<Box<dyn ::seventy::core::Sanitizer<#inner> + Send + Sync>> = ::std::sync::LazyLock::new(|| Box::new((#sanitizers)));
                std::sync::LazyLock::force(&SANITIZER).as_ref()
            }
        }

        impl #impl_generics ::seventy::core::Validatable for #ident #ty_generics #where_clause {
            fn validator() -> &'static dyn ::seventy::core::Validator<Self::Inner> {
                static VALIDATOR: ::std::sync::LazyLock<Box<dyn ::seventy::core::Validator<#inner> + Send + Sync>> = ::std::sync::LazyLock::new(|| Box::new((#validators)));
                std::sync::LazyLock::force(&VALIDATOR).as_ref()
            }
        }
    });

    if as_ref {
        expansion.push(quote! {
            impl #impl_generics AsRef<<Self as ::seventy::core::Newtype>::Inner> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &<Self as ::seventy::core::Newtype>::Inner {
                    self.to_inner()
                }
            }
        });
    }

    if bypassable {
        expansion.push(quote! {
            impl #impl_generics ::seventy::core::Bypassable for #ident #ty_generics #where_clause {
                unsafe fn unchecked_new(inner: impl Into<<Self as ::seventy::core::Newtype>::Inner>) -> Self {
                    Self(inner.into())
                }

                unsafe fn unsanitized_new(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner>
                where
                    Self: ::seventy::core::Validatable,
                {
                    let inner = inner.into();

                    let is_valid = <Self as ::seventy::core::Validatable>::validator().validate(&inner);

                    if is_valid {
                        Ok(Self(inner))
                    } else {
                        Err(inner)
                    }
                }

                unsafe fn unvalidated_new(inner: impl Into<<Self as ::seventy::core::Newtype>::Inner>) -> Self
                where
                    Self: ::seventy::core::Sanitizable,
                {
                    let mut inner = inner.into();

                    <Self as ::seventy::core::Sanitizable>::sanitizer().sanitize(&mut inner);

                    Self(inner)
                }

                unsafe fn to_inner_mut(&mut self) -> &mut <Self as ::seventy::core::Newtype>::Inner {
                    &mut self.0
                }
            }
        });
    }

    if deref {
        expansion.push(quote! {
            impl #impl_generics ::std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = <Self as ::seventy::core::Newtype>::Inner;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        });
    }

    if inherent {
        expansion.push(quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn try_new(inner: impl Into<<Self as ::seventy::core::Newtype>::Inner>) -> Result<Self, <Self as ::seventy::core::Newtype>::Inner> {
                    <Self as ::seventy::core::Newtype>::try_new(inner)
                }

                pub fn to_inner(&self) -> &<Self as ::seventy::core::Newtype>::Inner {
                    <Self as ::seventy::core::Newtype>::to_inner(self)
                }

                pub fn into_inner(self) -> <Self as ::seventy::core::Newtype>::Inner {
                    <Self as ::seventy::core::Newtype>::into_inner(self)
                }
            }
        });
    }

    if try_from {
        expansion.push(quote! {
            impl #impl_generics TryFrom<<Self as ::seventy::core::Newtype>::Inner> for #ident #ty_generics #where_clause {
                type Error = <Self as ::seventy::core::Newtype>::Inner;

                fn try_from(inner: <Self as ::seventy::core::Newtype>::Inner) -> Result<Self, Self::Error> {
                    <Self as ::seventy::core::Newtype>::try_new(inner)
                }
            }
        });
    }

    if unexposed {
        let module = format_ident!("__{ident}");

        Ok(quote! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub mod #module {
                use super::*;

                #item

                #(#expansion)*
            }

            #[doc(inline)]
            pub use #module::#ident;
        })
    } else {
        Ok(quote! { #(#expansion)* })
    }
}
