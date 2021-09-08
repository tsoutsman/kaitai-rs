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

/// Contains the information about all the attributes defined in the `seq` of a type.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    /// Generates the field declarations for all the attributes contained.
    ///
    /// Any attributes that have fixed contents are ignored.
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
    /// pub temp1: u32,
    /// pub temp2: CustomTypeDefinedElsewhere
    /// ```
    pub fn field_declarations(&self) -> TokenStream {
        let declarations = self
            .0
            .iter()
            .filter(|a| matches!(a.contents, Contents::Variable(_)))
            .map(|a| a.declaration());
        quote! { #(#declarations),* }
    }

    pub fn case_enum_decls(&self) -> TokenStream {
        self.0.iter().map(|a| a.enum_declaration()).collect()
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

    /// Generates the field definitions for all the attributes contained.
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
    /// temp1,
    /// temp2
    /// ```
    pub fn field_defs(&self) -> TokenStream {
        let defs = self
            .0
            .iter()
            .filter(|a| matches!(a.contents, Contents::Variable(_)))
            .map(|a| &a.ident);
        quote! { #(#defs),* }
    }
}

/// Contains the information about a single attribute defined in the `seq` of a type.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Attribute {
    pub ident: Ident,
    pub doc: DocSpec,
    pub contents: Contents,
}

impl Attribute {
    pub fn ucc_ident(&self) -> String {
        sc_to_ucc(&self.ident.to_string())
    }

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
            let id = &self.ident;

            let ty = match &c.enum_ident {
                Some(i) => TypeDef::Custom(Ident::new(&sc_to_ucc(i), Span::call_site())),
                None => c.ty.clone(),
            };
            let ty = match c.repeat {
                Some(_) => quote! { Vec<#ty> },
                None => quote! { #ty },
            };

            quote! {
                #doc
                pub #id: #ty
            }
        } else {
            TokenStream::new()
        }
    }

    pub fn enum_declaration(&self) -> TokenStream {
        if let Contents::Variable(VariableContents {
            ty: TypeDef::Switch { cases, .. },
            ..
        }) = &self.contents
        {
            let ident = &self.ucc_ident();
            let fields = cases.iter().map(|c| c.declaration());
            quote! {
                pub enum #ident {
                    #(#fields),*
                }
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
                let mut assignment = format!("let {} = ", self.ident);

                let read_call;

                match c.ty {
                    TypeDef::BuiltIn(ref t) => {
                        let temp = format!("buf.read_{}{}()?", t.ks_type(), c.endianness(meta));

                        read_call = match c.enum_ident {
                            Some(ref i) => format!(
                                "{}::n({}).ok_or(::kaitai::error::Error::NoEnumMatch)?",
                                i, temp
                            ),
                            None => temp,
                        }
                    }
                    TypeDef::Custom(ref t) => {
                        // Generates something like: "CustomType::new(buf)?"
                        // We are banking on the fact that this type is defined as a subtype
                        // in the ksy file and that its name will be the same.
                        read_call = format!("{}::new(buf)?", t);
                    }
                    TypeDef::Switch { .. } => {
                        todo!();
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
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TypeDef {
    /// The type is a builtin (e.g. [`u8`], [`i32`], [`f64`]).
    BuiltIn(BuiltIn),
    /// The type is a custom struct (i.e. defined in `types` in the KS file).
    /// The type is an enum (i.e. defined in `enums` in the KS file).
    /// TODO
    Custom(Ident),
    Switch {
        on: String,
        cases: Vec<Case>,
    },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuiltIn {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl BuiltIn {
    pub fn ks_type(&self) -> String {
        String::from(match self {
            BuiltIn::U8 => "u1",
            BuiltIn::U16 => "u2",
            BuiltIn::U32 => "u4",
            BuiltIn::U64 => "u8",
            BuiltIn::I8 => "s1",
            BuiltIn::I16 => "s2",
            BuiltIn::I32 => "s4",
            BuiltIn::I64 => "s8",
            BuiltIn::F32 => "f4",
            BuiltIn::F64 => "f8",
        })
    }
}

impl ToTokens for BuiltIn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BuiltIn::U8 => quote! { u8 },
            BuiltIn::U16 => quote! { u16 },
            BuiltIn::U32 => quote! { u32 },
            BuiltIn::U64 => quote! { u64 },
            BuiltIn::I8 => quote! { i8 },
            BuiltIn::I16 => quote! { i16 },
            BuiltIn::I32 => quote! { i32 },
            BuiltIn::I64 => quote! { i64 },
            BuiltIn::F32 => quote! { f32 },
            BuiltIn::F64 => quote! { f64 },
        });
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Case {
    value: CaseValue,
    ty: TypeDef,
}

impl Case {
    pub fn declaration(&self) -> TokenStream {
        let ident = &self.ty;
        quote! {
            #ident(#ident)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CaseValue {
    Enum(Ident),
    Int(i64),
}

impl ToTokens for CaseValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            CaseValue::Enum(i) => quote! { #i },
            CaseValue::Int(n) => quote! { #n },
        });
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            TypeDef::BuiltIn(t) => tokens.extend(t.to_token_stream()),
            // TODO get rid of unwrap
            TypeDef::Custom(t) => tokens.extend(quote! { #t }),
            TypeDef::Switch { .. } => todo!(),
        };
    }
}

impl From<&str> for TypeDef {
    fn from(s: &str) -> Self {
        match s {
            "u1" => TypeDef::BuiltIn(BuiltIn::U8),
            "u2" => TypeDef::BuiltIn(BuiltIn::U16),
            "u4" => TypeDef::BuiltIn(BuiltIn::U32),
            "u8" => TypeDef::BuiltIn(BuiltIn::U64),
            "s1" => TypeDef::BuiltIn(BuiltIn::I8),
            "s2" => TypeDef::BuiltIn(BuiltIn::I16),
            "s4" => TypeDef::BuiltIn(BuiltIn::I32),
            "s8" => TypeDef::BuiltIn(BuiltIn::I64),
            "f4" => TypeDef::BuiltIn(BuiltIn::F32),
            "f8" => TypeDef::BuiltIn(BuiltIn::F64),
            // The type is a user-defined type, meaning a struct or enum has (hopefully) been
            // generated somewhere with the name in ucc.
            _ => TypeDef::Custom(Ident::new(&sc_to_ucc(s), Span::call_site())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Repeat {
    Eos,
    #[allow(dead_code)]
    Expr(u32),
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
    ty: TypeDef,
    enum_ident: Option<String>,
    repeat: Option<Repeat>,
}

impl VariableContents {
    /// Returns a [`String`] describing the endianness of the `VariableContents`.
    ///
    /// Little-endian contents return "le". Big-endian contents return "be".
    ///
    /// If the contents are of KS type `u1` or `s1`, the function will return an empty string.
    pub fn endianness(&self, meta: &MetaSpec) -> String {
        match &self.ty {
            TypeDef::BuiltIn(BuiltIn::U8 | BuiltIn::I8) => "".to_owned(),
            _ => meta.endianness.to_string(),
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

/// Gets the [`TypeDef`] corresponding to a [`yaml::Hash`] representing an [`Attribute`].
///
/// The `map` argument should be a [`yaml::Hash`] that corresponds to an attribute, for example:
/// ```yaml
/// id: type
/// type: u4
/// enum: chunk_type
/// ```
/// or
/// ```yaml
/// id: data
/// size: len_data
/// type:
///   switch-on: type
///   cases:
///     'chunk_type::json': json
///     'chunk_type::bin': bin
///
pub fn ty(map: &yaml::Hash) -> Result<TypeDef> {
    match map.get(&yaml_rust::Yaml::String("type".to_owned())) {
        Some(c) => match c {
            Yaml::String(s) => Ok(TypeDef::try_from(&s[..])?),
            Yaml::Hash(h) => {
                let on = get_required_attr!(h; "switch-on" as Yaml::String(s) => s.clone())?;

                let mut cases = Vec::new();
                for case in get_required_attr!(h; "cases" as Yaml::Hash(m) => m.clone())? {
                    let value = match case.0 {
                        Yaml::Integer(n) => CaseValue::Int(n),
                        Yaml::String(s) => CaseValue::Enum(Ident::new(&s, Span::call_site())),
                        _ => todo!(),
                    };
                    let ty = case.1;
                    let ty = TypeDef::try_from(
                        &assert_pattern!(ty; Yaml::String(s) => s; attr: "case type")?[..],
                    )?;

                    cases.push(Case { value, ty });
                }

                Ok(TypeDef::Switch { on, cases })
            }
            _ => Err(Error::InvalidAttrType {
                attr: "type".to_owned(),
                pat: "Yaml::String(s) or Yaml::Hash(h)".to_owned(),
                actual: c.clone(),
            }
            .into()),
        },
        None => Err(Error::RequiredAttrNotFound("type".to_owned()).into()),
    }
}

pub fn seq(map: &yaml::Hash) -> Result<Attributes> {
    let seq = get_required_attr!(map; "seq" as Yaml::Array(a) => a)
        .context("seq: seq is not an array")?;
    let mut result = Vec::new();

    for item in seq {
        let item = match item {
            Yaml::Hash(m) => {
                let ident = get_required_attr!(m; "id" as Yaml::String(s) => Ident::new(s, Span::call_site()))
                    .context("seq: id not found or it is not a string")?;
                let doc = doc(m).context("seq: error parsing doc/doc-ref")?;

                let contents = match fixed_contents(m).context("seq: error parsing contents")? {
                    Some(c) => Contents::Fixed(c),
                    None => {
                        let ty = ty(m).context("ty: error parsing type")?;
                        let enum_ident =
                            get_attr!(m; "enum" as Yaml::String(s) => sc_to_ucc(&s.clone()))
                                .context("seq: enum ident is not a string")?;
                        let repeat = get_attr!(m; "repeat" as Yaml::String(s) => match s.as_ref() {
                            "eos" => Repeat::Eos,
                            _ => todo!()
                        })
                        .context("seq: repeat is not a string")?;

                        Contents::Variable(VariableContents {
                            ty,
                            enum_ident,
                            repeat,
                        })
                    }
                };

                Attribute {
                    ident,
                    doc,
                    contents,
                }
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
                    ident: Ident::new("example_id", Span::call_site()),
                    doc: DocSpec {
                        description: Some("foo".to_owned()),
                        reference: Some("bar".to_owned())
                    },
                    contents: Contents::Variable(VariableContents {
                        ty: TypeDef::Custom(Ident::new("ExampleType", Span::call_site())),
                        enum_ident: None,
                        repeat: Some(Repeat::Eos),
                    })
                };
                2
            ])
        );
    }

    #[test]
    fn attribute_enum_declaration() {
        let map = &YamlLoader::load_from_str(
            "
seq:
  - id: protocol
    type: u4
  - id: another_thing
    type:
      switch-on: protocol
      cases:
        0x5: json
        0x6: bin\0",
        )
        .unwrap()[0];
        let map = assert_pattern!(map; Yaml::Hash(m) => m; attr: "irrelevant").unwrap();

        let result = seq(map).unwrap();
        eprintln!("RESULT: {}", result.case_enum_decls());
    }
}
