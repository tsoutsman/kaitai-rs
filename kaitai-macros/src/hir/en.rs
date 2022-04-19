use crate::{de, hir::doc::Doc, util::sc_to_ucc};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Enumeration {
    ident: Ident,
    variants: Vec<Variant>,
}

impl From<(&str, de::en::Enum)> for Enumeration {
    fn from((id, en): (&str, de::en::Enum)) -> Self {
        Self {
            ident: Ident::new(&sc_to_ucc(&id), Span::call_site()),
            variants: en
                .0
                .into_iter()
                .map(|(value, de::en::EnumValue { id, doc })| Variant {
                    doc: (None, doc).into(),
                    ident: Ident::new(&sc_to_ucc(&id), Span::call_site()),
                    value,
                })
                .collect(),
        }
    }
}

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let variant_defs = self.variants.iter().map(|v| v.def());
        let variant_match_arms = self.variants.iter().map(|v| v.match_arm());

        tokens.extend(quote! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            // TODO: Is this repr ok?
            #[repr(u64)]
            pub enum #ident {
                #(#variant_defs),*
            }

            impl #ident {
                // TODO: For some reason using an Into bound on N doesn't work
                // so I have to use this weird where clause. TODO: Add doc
                // comment for this function.
                pub fn n<N>(n: N) -> Option<Self> where u64: From<N> {
                    match u64::from(n) {
                        #(#variant_match_arms),*,
                        _ => None,
                    }
                }
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Variant {
    doc: Doc,
    ident: Ident,
    value: u64,
}

impl Variant {
    fn def(&self) -> TokenStream {
        let Variant { doc, ident, value } = self;
        quote! {
            #doc
            #ident = #value
        }
    }

    fn match_arm(&self) -> TokenStream {
        let Variant { ident, value, .. } = self;
        quote! { #value => ::std::option::Option::Some(Self::#ident) }
    }
}
