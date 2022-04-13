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

#[derive(Debug)]
pub struct Type {
    id: Ident,
    endianness: Endianness,
    doc: Doc,
    params: Vec<Parameter>,
    seq: Attributes,
    types: HashMap<String, Type>,
    instances: HashMap<String, Attribute>,
    enums: HashMap<String, Enumeration>,
}

pub struct InheritedMeta {
    pub id: Option<(Ident, bool)>,
    pub endianness: Option<Endianness>,
}

impl TryFrom<(InheritedMeta, de::ty::Type)> for Type {
    type Error = ();

    fn try_from((inherited_meta, ty): (InheritedMeta, de::ty::Type)) -> Result<Self, Self::Error> {
        let meta_id = ty
            .meta
            .as_ref()
            .map(|m| {
                m.id.as_ref()
                    .map(|id| Ident::new(&sc_to_ucc(&id), Span::call_site()))
            })
            .flatten();
        let id = match inherited_meta.id {
            Some((id, overwrite)) => {
                if overwrite {
                    id
                } else {
                    meta_id.unwrap()
                }
            }
            None => meta_id.unwrap(),
        };

        let endianness = ty
            .meta
            .as_ref()
            .map(|m| m.endianness)
            .flatten()
            .or(inherited_meta.endianness)
            .unwrap();
        // TODO: All the meta doc clones.
        let doc = (ty.meta.as_ref().map(|meta| meta.doc.clone()), ty.doc).into();
        let seq = (ty.meta.as_ref().map(|m| m.doc.clone()), ty.seq)
            .try_into()
            .unwrap();

        Ok(Self {
            id,
            endianness,
            doc,
            // TODO
            params: Default::default(),
            seq,
            // TODO
            types: Default::default(),
            // TODO
            instances: Default::default(),
            // TODO
            enums: Default::default(),
        })
    }
}

impl quote::ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let doc = &self.doc;
        let id = &self.id;
        let field_defs = self.seq.field_definitions();
        let var_assignments = self.seq.variable_assignments(self.endianness);
        let field_assignments = self.seq.field_assignments();

        tokens.extend(quote::quote! {
            #doc
            // TODO: Pass down attributes from struct
            #[derive(Debug)]
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
