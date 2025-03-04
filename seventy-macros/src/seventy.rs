//! Helpers for expanding the `seventy` attribute proc-macro.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, Error, Fields, FieldsUnnamed, GenericParam, ItemStruct, Lifetime,
    LifetimeParam, Meta, Result, Token,
};

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
    let mut deref = false;
    let mut display = false;
    let mut try_from = false;

    let mut deserializable = false;
    let mut serializable = false;

    let mut bypassable = false;
    let mut inherent = false;
    let mut shared = false;
    let mut unexposed = false;

    let mut sanitizers = None;
    let mut validators = None;

    for meta in metas {
        if meta.path().is_ident("upgrades") {
            let metas = meta
                .require_list()?
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

            for meta in metas {
                if meta.path().is_ident("as_ref") {
                    as_ref = true;
                } else if meta.path().is_ident("deref") {
                    deref = true;
                } else if meta.path().is_ident("display") {
                    display = true;
                } else if meta.path().is_ident("try_from") {
                    try_from = true;
                } else if meta.path().is_ident("deserializable") {
                    deserializable = true;
                } else if meta.path().is_ident("serializable") {
                    serializable = true;
                } else if meta.path().is_ident("bypassable") {
                    bypassable = true;
                } else if meta.path().is_ident("inherent") {
                    inherent = true;
                } else if meta.path().is_ident("shared") {
                    shared = true;
                } else if meta.path().is_ident("unexposed") {
                    unexposed = true;
                } else {
                    return Err(Error::new_spanned(meta, "unrecognized upgrade"));
                }
            }
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

    expansion.push(quote! {
        impl #impl_generics ::seventy::core::Newtype for #ident #ty_generics #where_clause {
            type Inner = #inner;

            fn try_new(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner> {
                let mut inner = inner.into();

                <Self as ::seventy::core::Sanitizable>::sanitize(&mut inner);

                let is_valid = <Self as ::seventy::core::Validatable>::validate(&inner);

                if is_valid {
                    Ok(Self(inner))
                } else {
                    Err(inner)
                }
            }

            fn as_inner(&self) -> &Self::Inner {
                &self.0
            }

            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    });

    let sanitize;
    let validate;

    if shared {
        sanitize = quote! {
            static SANITIZER: ::std::sync::LazyLock<Box<dyn ::seventy::core::Sanitizer<#inner> + Send + Sync>> = ::std::sync::LazyLock::new(|| Box::new(::seventy::builtins::bundle::bundle!(#sanitizers)));
            std::sync::LazyLock::force(&SANITIZER).sanitize(target);
        };

        validate = quote! {
            static VALIDATOR: ::std::sync::LazyLock<Box<dyn ::seventy::core::Validator<#inner> + Send + Sync>> = ::std::sync::LazyLock::new(|| Box::new(::seventy::builtins::bundle::bundle!(#validators)));
            std::sync::LazyLock::force(&VALIDATOR).validate(target)
        };
    } else {
        sanitize = quote! {
            <_ as ::seventy::core::Sanitizer<Self::Inner>>::sanitize(&::seventy::builtins::bundle::bundle!(#sanitizers), target);
        };

        validate = quote! {
            <_ as ::seventy::core::Validator<Self::Inner>>::validate(&::seventy::builtins::bundle::bundle!(#validators), target)
        }
    }

    expansion.push(quote! {
        impl #impl_generics ::seventy::core::Sanitizable for #ident #ty_generics #where_clause {
            fn sanitize(target: &mut Self::Inner) {
                #sanitize
            }
        }

        impl #impl_generics ::seventy::core::Validatable for #ident #ty_generics #where_clause {
            fn validate(target: &Self::Inner) -> bool {
                #validate
            }
        }
    });

    if as_ref {
        expansion.push(quote! {
            impl #impl_generics AsRef<<Self as ::seventy::core::Newtype>::Inner> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &<Self as ::seventy::core::Newtype>::Inner {
                    self.as_inner()
                }
            }
        });
    }

    if bypassable {
        expansion.push(quote! {
            impl #impl_generics ::seventy::core::Bypassable for #ident #ty_generics #where_clause {
                unsafe fn new_unchecked(inner: impl Into<<Self as ::seventy::core::Newtype>::Inner>) -> Self {
                    Self(inner.into())
                }

                unsafe fn new_unsanitized(inner: impl Into<Self::Inner>) -> Result<Self, Self::Inner> {
                    let inner = inner.into();

                    let is_valid = <Self as ::seventy::core::Validatable>::validate(&inner);

                    if is_valid {
                        Ok(Self(inner))
                    } else {
                        Err(inner)
                    }
                }

                unsafe fn new_unvalidated(inner: impl Into<<Self as ::seventy::core::Newtype>::Inner>) -> Self {
                    let mut inner = inner.into();

                    <Self as ::seventy::core::Sanitizable>::sanitize(&mut inner);

                    Self(inner)
                }

                unsafe fn as_inner_mut(&mut self) -> &mut <Self as ::seventy::core::Newtype>::Inner {
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

    if deserializable {
        let mut generics = item.generics.clone();
        let lifetime = Lifetime::new("'de", Span::call_site());
        generics
            .params
            .push(GenericParam::from(LifetimeParam::new(lifetime)));
        let (impl_generics, _, _) = generics.split_for_impl();

        expansion.push(quote! {
            impl #impl_generics ::serde::Deserialize<'de> for #ident #ty_generics #where_clause {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let inner = <Self as ::seventy::core::Newtype>::Inner::deserialize(deserializer)?;
                    <Self as ::seventy::core::Newtype>::try_new(inner)
                        .map_err(|_| ::serde::de::Error::custom("Validation error"))
                }
            }
        });
    }

    if display {
        expansion.push(quote! {
            impl #impl_generics ::std::fmt::Display for #ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    <Self as ::seventy::core::Newtype>::as_inner(self).fmt(f)
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

                pub fn as_inner(&self) -> &<Self as ::seventy::core::Newtype>::Inner {
                    <Self as ::seventy::core::Newtype>::as_inner(self)
                }

                pub fn into_inner(self) -> <Self as ::seventy::core::Newtype>::Inner {
                    <Self as ::seventy::core::Newtype>::into_inner(self)
                }
            }
        });
    }

    if serializable {
        expansion.push(quote! {
            impl #impl_generics ::serde::Serialize for #ident #ty_generics #where_clause {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    serializer.serialize_newtype_struct(stringify!(#ident), self.as_inner())
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
            mod #module {
                use super::*;

                #item

                #(#expansion)*
            }

            #[doc(inline)]
            pub use #module::#ident;
        })
    } else {
        Ok(quote! {
            #item

            #(#expansion)*
        })
    }
}
