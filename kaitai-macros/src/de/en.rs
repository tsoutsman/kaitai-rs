use std::collections::HashMap;

use crate::de::data::deserialize_string_or_seq;

use serde::{
    de::{self, MapAccess},
    Deserialize, Deserializer,
};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Enum(HashMap<u64, EnumValue>);

#[derive(Clone, Debug)]
pub struct EnumValue {
    id: String,
    doc: Option<String>,
    doc_ref: Option<Vec<String>>,
}

impl<'de> Deserialize<'de> for EnumValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "kebab-case")]
        enum Field {
            Id,
            Doc,
            DocRef,
        }

        struct EnumValueVisitor;

        impl<'de> de::Visitor<'de> for EnumValueVisitor {
            type Value = EnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct EnumValue")
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EnumValue {
                    id: id.to_owned(),
                    doc: None,
                    doc_ref: None,
                })
            }

            fn visit_string<E>(self, id: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EnumValue {
                    id,
                    doc: None,
                    doc_ref: None,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut doc = None;
                let mut doc_ref = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Doc => {
                            if doc.is_some() {
                                return Err(de::Error::duplicate_field("doc"));
                            }
                            doc = Some(map.next_value()?);
                        }
                        Field::DocRef => {
                            struct DocRefDeserialize(Option<Vec<String>>);

                            impl<'de> Deserialize<'de> for DocRefDeserialize {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: Deserializer<'de>,
                                {
                                    Ok(Self(deserialize_string_or_seq(deserializer)?))
                                }
                            }

                            if doc_ref.is_some() {
                                return Err(de::Error::duplicate_field("doc_ref"));
                            }
                            doc_ref = map.next_value::<DocRefDeserialize>()?.0;
                        }
                    }
                }

                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;

                Ok(EnumValue { id, doc, doc_ref })
            }
        }

        deserializer.deserialize_any(EnumValueVisitor)
    }
}