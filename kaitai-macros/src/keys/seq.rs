use crate::{
    error::Error,
    keys::{
        doc::{doc, DocSpec},
        meta::MetaSpec,
    },
    util::{assert_pattern, get_attr, get_required_attr, sc_to_ucc},
};

use std::convert::TryFrom;

use anyhow::{Context, Result};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Ident;
use yaml_rust::{yaml, Yaml};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    pub fn field_declarations(&self) -> TokenStream {
        let declarations = self
            .0
            .iter()
            .filter(|a| matches!(a.contents, Contents::Variable(_)))
            .map(|a| a.declaration());
        quote! { #(#declarations),* }
    }

    /// Generates the variable definitions for all the attributes contained.
    ///
    /// The output will also contain any checks defined by a `contents` key.
    ///
    /// # Example
    ///
    /// Assuming that encoding is little-endian.
    ///
    /// ```yaml
    /// seq:
    ///   - id: temp1
    ///     type: u4
    ///   - id: temp2
    ///     type: custom_type_defined_elsewhere
    ///   - id: temp3
    ///     contents: hello
    /// ```
    /// would result in:
    /// ```ignore
    /// let temp1 = buf.read_u4le();
    /// let temp2 = CustomTypeDefinedElsewhere::new(buf);
    /// buf.ensure_fixed_contents("hello".as_bytes())?;
    /// ```
    pub fn var_defs(&self, meta: &MetaSpec) -> TokenStream {
        self.0.iter().map(|a| a.var_def(meta)).collect()
    }

    pub fn field_defs(&self) -> TokenStream {
        let defs = self
            .0
            .iter()
            .filter(|a| matches!(a.contents, Contents::Variable(_)))
            .map(|a| &a.id);
        quote! { #(#defs),* }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Attribute {
    pub id: Ident,
    pub doc: DocSpec,
    pub contents: Contents,
}

impl Attribute {
    /// Returns a [`TokenStream`] containing the assignment of the `Attribute`.
    ///
    /// # Examples
    ///
    /// ## Built-in
    ///
    /// ```yaml
    /// name: example_attr
    /// type: u4
    /// ```
    /// results in
    /// ```ignore
    /// pub example_attr: u32
    /// ```
    ///
    /// Note that a `u4` in KS is the equivalent of a [`u32`] in Rust.
    ///
    /// ## Custom type
    ///
    /// ```yaml
    /// name: example_attr
    /// type: example_type
    /// ## where example_type is a custom type defined in the `types` part of the file.
    /// ```
    /// results in
    /// ```ignore
    /// pub example_attr: ExampleType
    /// ```
    ///
    /// Note that the name of the type is converted into upper camel case.
    ///
    /// ## Enum
    ///
    /// ```yaml
    /// name: example_attr
    /// type: example_enum
    /// ## where example_enum is an enum defined in the `enums` part of the file.
    /// ```
    /// results in
    /// ```ignore
    /// pub example_attr: ExampleEnum
    /// ```
    ///
    /// Note that the name of the enum is converted into upper camel case.
    ///
    /// ## Fixed Contents
    ///
    /// Fixed contents attributes are only checked and are not stored in the struct.
    /// Hence, this method return an empty [`TokenStream`] if the attribute has fixed
    /// contents.
    pub fn declaration(&self) -> TokenStream {
        if let Contents::Variable(ref c) = self.contents {
            let doc = &self.doc;
            let id = &self.id;

            let rust_type = c.rust_type();
            let ty = match c.repeat {
                Some(_) => quote! { Vec<#rust_type> },
                None => quote! { #rust_type },
            };

            quote! {
                #doc
                pub #id: #ty
            }
        } else {
            TokenStream::new()
        }
    }

    /// Returns a [`TokenStream`] containing the definition of the variable containing
    /// the `Attribute`.
    ///
    /// # Examples
    /// All the following examples assume the format is little endian.
    ///
    /// ## Built-in
    ///
    /// ```yaml
    /// name: example_attr
    /// type: u4
    /// ```
    /// results in
    /// ```ignore
    /// let example_attr = buf.read_u4le()?;
    /// ```
    ///
    /// ## Custom type
    ///
    /// ```yaml
    /// name: example_attr
    /// type: example_type
    /// ## where example_type is a custom type defined in the `types` part of the file.
    /// ```
    /// results in
    /// ```ignore
    /// let example_attr = ExampleType::new(buf)?;
    /// ```
    ///
    /// Note that the name of the type is converted into upper camel case.
    ///
    /// ## Enum
    ///
    /// ```yaml
    /// name: example_attr
    /// type: example_enum
    /// ## where example_enum is an enum defined in the `enums` part of the file.
    /// ```
    /// results in
    /// ```ignore
    /// let example_attr = ExampleEnum::n(buf.read_u4le()?).ok_or(::kaitai::error::Error::NoEnumMatch)?;
    /// ```
    ///
    /// Note that the name of the enum is converted into upper camel case.
    ///
    /// ## Fixed Contents
    ///
    /// Fixed contents attributes are only checked and are not stored in the struct.
    ///
    /// ```yaml
    /// name: id
    /// contents: glTF
    /// ```
    /// results is
    /// ```ignore
    /// buf.ensure_fixed_contents("glTF".as_bytes())?;
    /// ```
    ///
    pub fn var_def(&self, meta: &MetaSpec) -> TokenStream {
        match self.contents {
            Contents::Fixed(ref c) => {
                quote! { buf.ensure_fixed_contents(&#c)?; }
            }
            Contents::Variable(ref c) => {
                let mut assignment = format!("let {} = ", self.id);

                let read_call;

                match c.rust_type() {
                    TypeDef::BuiltIn(_) => {
                        read_call = format!("buf.read_{}{}()?", c.ks_type, c.endianness(meta));
                    }
                    TypeDef::Struct(t) => {
                        // Generates something like: "CustomType::new(buf)?"
                        // We are banking on the fact that this type is defined as a subtype
                        // in the ksy file and that its name will be the same.
                        read_call = format!("{}::new(buf)?", t);
                    }
                    TypeDef::Enum(t) => {
                        read_call = format!(
                            "{}::n(buf.read_{}{}()?).ok_or(::kaitai::error::Error::NoEnumMatch)?",
                            t,
                            c.ks_type,
                            c.endianness(meta)
                        );
                    }
                }

                match c.repeat {
                    Some(ref r) => match r {
                        Repeat::Eos => {
                            assignment += &format!(
                                "
{{
    let mut result = Vec::new();
    while !buf.is_eof()? {{
        result.push({});
    }}
    result
}}
",
                                read_call
                            )
                        }
                        Repeat::Expr(_) => todo!("Repeat::Expr"),
                        Repeat::Until(_) => todo!("Repeat::Until"),
                    },
                    None => {
                        assignment += &read_call;
                    }
                }

                assignment += ";";

                // TODO handle unwrap
                assignment.parse().unwrap()
            }
        }
    }
}

/// Describes the rust type of a Kaitai Struct attribute.
#[derive(Clone, Debug)]
pub enum TypeDef {
    /// The type is a builtin (e.g. [`u8`], [`i32`], [`f64`]).
    BuiltIn(TokenStream),
    /// The type is a custom struct (i.e. defined in `types` in the KS file).
    Struct(TokenStream),
    /// The type is an enum (i.e. defined in `enums` in the KS file).
    Enum(TokenStream),
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            TypeDef::BuiltIn(t) => tokens.extend(t.clone()),
            TypeDef::Struct(t) => tokens.extend(t.clone()),
            TypeDef::Enum(t) => tokens.extend(t.clone()),
        };
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Repeat {
    Eos,
    #[allow(dead_code)]
    Expr(String),
    #[allow(dead_code)]
    Until(String),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Contents {
    Fixed(FixedContents),
    Variable(VariableContents),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FixedContents {
    String(String),
    Array(Vec<u8>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariableContents {
    ks_type: String,
    enum_ident: Option<String>,
    repeat: Option<Repeat>,
}

impl VariableContents {
    pub fn endianness(&self, meta: &MetaSpec) -> String {
        match &self.ks_type[..] {
            "u1" | "s1" => "".to_owned(),
            _ => meta.endianness.to_string(),
        }
    }

    /// Returns a [`TypeDef`] representing the Rust type of the `VariableContents`.
    pub fn rust_type(&self) -> TypeDef {
        if let Some(ref i) = self.enum_ident {
            return TypeDef::Enum(Ident::new(&sc_to_ucc(i), Span::call_site()).to_token_stream());
        }

        match &self.ks_type[..] {
            "u1" => TypeDef::BuiltIn(quote! { u8 }),
            "u2" => TypeDef::BuiltIn(quote! { u16 }),
            "u4" => TypeDef::BuiltIn(quote! { u32 }),
            "u8" => TypeDef::BuiltIn(quote! { u64 }),
            "s1" => TypeDef::BuiltIn(quote! { i8 }),
            "s2" => TypeDef::BuiltIn(quote! { i16 }),
            "s4" => TypeDef::BuiltIn(quote! { i32 }),
            "s8" => TypeDef::BuiltIn(quote! { i64 }),
            "f4" => TypeDef::BuiltIn(quote! { f32 }),
            "f8" => TypeDef::BuiltIn(quote! { f64 }),
            // The type is a user-defined type, meaning a struct has been generated somewhere with
            // the name in ucc.
            &_ => TypeDef::Struct(
                Ident::new(&sc_to_ucc(&self.ks_type), Span::call_site()).to_token_stream(),
            ),
        }
    }
}

impl ToTokens for FixedContents {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            FixedContents::String(s) => tokens.extend(quote! { #s.as_bytes() }),
            FixedContents::Array(a) => {
                let items = a.iter().map(|i| quote! { #i });
                tokens.extend(quote! {
                    [ #(#items),* ]
                })
            }
        }
    }
}

pub fn fixed_contents(map: &yaml::Hash) -> Result<Option<FixedContents>> {
    match map.get(&yaml_rust::Yaml::String("contents".to_owned())) {
        Some(c) => match c {
            Yaml::String(s) => Ok(Some(FixedContents::String(s.clone()))),
            Yaml::Array(a) => {
                let a = a.iter().map(|i| -> Result<u8> {
                    let i = assert_pattern!(i; Yaml::Integer(i) => i; attr: "yaml integer")?;
                    match u8::try_from(*i) {
                        Ok(u) => Ok(u),
                        Err(_) => anyhow::bail!("contents: arrays can only be composed of u8s"),
                    }
                });
                match a.collect::<Result<Vec<u8>>>() {
                    Ok(a) => Ok(Some(FixedContents::Array(a))),
                    Err(e) => Err(e),
                }
            }
            _ => Err(Error::InvalidAttrType {
                attr: "contents".to_owned(),
                pat: "Yaml::String(s) or Yaml::Array(a)".to_owned(),
                actual: c.clone(),
            }
            .into()),
        },
        None => Ok(None),
    }
}

pub fn seq(map: &yaml::Hash) -> Result<Attributes> {
    let seq = get_required_attr!(map; "seq" as Yaml::Array(a) => a)
        .context("seq: seq is not an array")?;
    let mut result = Vec::new();

    for item in seq {
        let item = match item {
            Yaml::Hash(m) => {
                let id = get_required_attr!(m; "id" as Yaml::String(s) => Ident::new(s, Span::call_site()))
                    .context("seq: id not found or it is not a string")?;
                let doc = doc(m).context("seq: error parsing doc/doc-ref")?;

                let contents = match fixed_contents(m)? {
                    Some(c) => Contents::Fixed(c),
                    None => {
                        let ks_type = get_required_attr!(
                                                            m;
                                                            "type" as Yaml::String(s) => s.clone())
                        .context("seq: type is not found or it is not a string")?;
                        let enum_ident = get_attr!(m; "enum" as Yaml::String(s) => s.clone())
                            .context("seq: enum ident is not a string")?;
                        let repeat = get_attr!(m; "repeat" as Yaml::String(s) => match s.as_ref() {
                            "eos" => Repeat::Eos,
                            _ => todo!()
                        })
                        .context("seq: repeat is not a string")?;

                        Contents::Variable(VariableContents {
                            ks_type,
                            enum_ident,
                            repeat,
                        })
                    }
                };

                Attribute { id, doc, contents }
            }

            _ => {
                return Err(Error::InvalidAttribute(item.clone()))
                    .context("seq: attribute is not a hashmap")
            }
        };

        result.push(item);
    }

    Ok(Attributes(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_pattern;
    use yaml_rust::YamlLoader;

    #[test]
    fn no_seq() {
        let input = yaml::Hash::new();

        let result = seq(&input);

        assert_eq!(
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::RequiredAttrNotFound("seq".to_owned())
        );
    }

    #[test]
    fn missing_type() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: foo\0",
        )
        .unwrap()[0];
        let map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        let result = seq(map);

        assert_eq!(
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::RequiredAttrNotFound("type".to_owned())
        );
    }

    #[test]
    fn wrong_id_type() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: 3
    type: example_type\0",
        )
        .unwrap()[0];
        let map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        let result = seq(map);

        assert_eq!(
            result.unwrap_err().downcast_ref::<Error>().unwrap(),
            &Error::InvalidAttrType {
                attr: "id".to_owned(),
                pat: "Yaml::String(s)".to_owned(),
                actual: Yaml::Integer(3),
            }
        );
    }

    #[test]
    fn all_attributes() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: example_id
    type: example_type
    doc: foo
    doc-ref: bar
    repeat: eos
  - id: example_id
    type: example_type
    doc: foo
    doc-ref: bar
    repeat: eos\0",
        )
        .unwrap()[0];
        let map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        let result = seq(map);

        assert_eq!(
            result.unwrap(),
            Attributes(vec![
                Attribute {
                    id: Ident::new("example_id", Span::call_site()),
                    doc: DocSpec {
                        description: Some("foo".to_owned()),
                        reference: Some("bar".to_owned())
                    },
                    contents: Contents::Variable(VariableContents {
                        ks_type: "example_type".to_owned(),
                        enum_ident: None,
                        repeat: Some(Repeat::Eos),
                    })
                };
                2
            ])
        );
    }

    #[test]
    fn attribute_enum() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: protocol
    enum: ip_protocol
  - id: another_thing
    enum: enum_id\0",
        )
        .unwrap()[0];
        let _map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        // let result = gen_field_assignments(map).unwrap();
        // eprintln!("RESULT: {:?}", result);
    }
}
