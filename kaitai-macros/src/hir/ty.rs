use crate::{
    de,
    hir::{
        attr::{Attribute, Attributes},
        doc::Doc,
        en::Enumeration,
        meta::Endianness,
        param::Parameter,
    },
    util::sc_to_ucc,
};

use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use quote::ToTokens;

#[derive(Debug)]
pub struct Type {
    id: Ident,
    endianness: Endianness,
    doc: Doc,
    params: Vec<Parameter>,
    seq: Attributes,
    types: Vec<Type>,
    instances: HashMap<String, Attribute>,
    enums: Vec<Enumeration>,
}

pub struct InheritedMeta {
    pub id: Option<(Ident, bool)>,
    pub endianness: Option<Endianness>,
}

impl TryFrom<(InheritedMeta, de::ty::Type)> for Type {
    type Error = ();

    fn try_from((inherited_meta, ty): (InheritedMeta, de::ty::Type)) -> Result<Self, Self::Error> {
        let meta_id = ty.meta.as_ref().and_then(|m| {
            m.id.as_ref()
                .map(|id| Ident::new(&sc_to_ucc(&id), Span::call_site()))
        });
        let id = match inherited_meta.id {
            Some((id, overwrite)) => {
                if overwrite {
                    id
                } else if let Some(id) = meta_id {
                    id
                } else {
                    id
                }
            }
            None => meta_id.unwrap(),
        };

        let endianness = ty
            .meta
            .as_ref()
            .and_then(|m| m.endianness)
            .or(inherited_meta.endianness)
            .expect("no endianness inherited");
        // TODO: All the meta doc clones.
        let doc = (ty.meta.as_ref().map(|meta| meta.doc.clone()), ty.doc).into();
        let seq = (ty.meta.as_ref().map(|m| m.doc.clone()), ty.seq)
            .try_into()
            .expect("seq validation failed");
        let types = ty
            .types
            .into_iter()
            .map(|(id, ty)| {
                let inherited_meta = InheritedMeta {
                    id: Some((Ident::new(&sc_to_ucc(&id), Span::call_site()), false)),
                    endianness: Some(endianness),
                };
                Type::try_from((inherited_meta, ty)).expect("type validation failed")
            })
            .collect();
        let enums = ty
            .enums
            .into_iter()
            .map(|(id, en)| (id.as_ref(), en).into())
            .collect();

        Ok(Self {
            id,
            endianness,
            doc,
            // TODO
            params: Default::default(),
            seq,
            types,
            // TODO
            instances: Default::default(),
            enums,
        })
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let type_defs = self.types.iter().map(|ty| ty.into_token_stream());
        let enum_defs = self.enums.iter().map(|en| en.into_token_stream());
        let doc = &self.doc;
        let id = &self.id;
        let field_defs = self.seq.field_definitions();
        let var_assignments = self.seq.variable_assignments(self.endianness);
        let field_assignments = self.seq.field_assignments();

        tokens.extend(quote::quote! {
            #(#type_defs)*
            #(#enum_defs)*

            #doc
            // TODO: Pass down attributes from struct
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct #id {
                #(#field_defs),*
            }

            #[automatically_derived]
            impl ::kaitai::KaitaiStruct for #id {
                fn new<S: ::kaitai::__private::KaitaiStream>(buf: &mut S) -> ::kaitai::error::Result<Self> {
                    #(#var_assignments);*;
                    Ok(Self {
                        #(#field_assignments),*
                    })
                }
                fn read<S: ::kaitai::__private::KaitaiStream>(&mut self, _: &mut S) -> ::kaitai::error::Result<()> {
                    todo!();
                }
            }
        });
    }
}
