use crate::de::{doc::Doc, meta::MetaDoc};

use proc_macro2::TokenStream;

macro_rules! gen_lists {
    ($vec:ident; $($e:expr => $i:expr);*;) => {
        $(
            if !$i.is_empty() {
                $vec.push(header($e) + &list($i));
            }
        )*
    };
}

fn doc_attr(doc: Doc, meta_doc: Option<MetaDoc>) -> TokenStream {
    let mut parts = Vec::new();

    if let Some(ref meta_doc) = meta_doc {
        if !meta_doc.title.is_empty() {
            parts.push(format!("A {}, deserializer", meta_doc.title));
        }
    }

    if !doc.doc.is_empty() {
        parts.push(doc.doc);
    }

    if let Some(ref meta_doc) = meta_doc {
        parts.push(header("Format License") + &meta_doc.license);
    }

    gen_lists! {
        parts;
        "References" => doc.doc_ref;
    }

    if let Some(meta_doc) = meta_doc {
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
    quote::quote! { #[doc = #doc_comment] }
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
