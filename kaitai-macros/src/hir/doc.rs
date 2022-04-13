pub use crate::de::{doc, meta::MetaDoc};

pub struct Doc {
    meta_doc: Option<MetaDoc>,
    doc: doc::Doc,
}

impl Doc {
    pub fn new() -> Self {
        Self {
            meta_doc: None,
            doc: doc::Doc {
                doc: String::new(),
                doc_ref: Vec::new(),
            },
        }
    }
}

impl From<(Option<MetaDoc>, doc::Doc)> for Doc {
    fn from((meta_doc, doc): (Option<MetaDoc>, doc::Doc)) -> Self {
        Self { meta_doc, doc }
    }
}

macro_rules! gen_lists {
    ($vec:ident; $($e:expr => $i:expr);*;) => {
        $(
            if !$i.is_empty() {
                $vec.push(header($e) + &list($i));
            }
        )*
    };
}

impl quote::ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // TODO: Clones
        let mut parts = Vec::new();

        if let Some(ref meta_doc) = self.meta_doc {
            if !meta_doc.title.is_empty() {
                parts.push(format!("A {}, deserializer", meta_doc.title));
            }
        }

        if !self.doc.doc.is_empty() {
            parts.push(self.doc.doc.clone());
        }

        if let Some(ref meta_doc) = self.meta_doc {
            parts.push(header("Format License") + &meta_doc.license);
        }

        gen_lists! {
            parts;
            "References" => self.doc.doc_ref.clone();
        }

        if let Some(meta_doc) = self.meta_doc.clone() {
            // TODO: Generate links to wikis.
            gen_lists! {
                parts;
                "Applications" => meta_doc.application;
                "File Extensions" => meta_doc.file_extension;
                "Forensics Wiki Articles" => meta_doc.xref.forensicswiki;
                "ISOs" => meta_doc.xref.iso;
                "Just Solve Articles" => meta_doc.xref.justsolve;
                "US Library of Congress Digital Formats Identifiers" => meta_doc.xref.loc;
                "Mime Types" => meta_doc.xref.mime;
                "UK National Archives PRONOM Identifiers" => meta_doc.xref.pronom;
                "RFCs" => meta_doc.xref.rfc;
                "Wikidata Identifiers" => meta_doc.xref.wikidata;
            }
        }

        let doc_comment = parts.join("\n");
        tokens.extend(quote::quote! { #[doc = #doc_comment] })
    }
}

fn header(title: &str) -> String {
    format!("### {}\n", title)
}

fn list<T>(iter: T) -> String
where
    T: std::iter::IntoIterator<Item = String>,
{
    iter.into_iter().map(|i| format!("- {}\n", i)).collect()
}
