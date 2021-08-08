use thiserror::Error;

pub type Result<T> = std::result::Result<T, MacroError>;

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MacroError {
    #[error("attribute in seq invalid: {0:?}")]
    InvalidAttribute(yaml_rust::Yaml),
    #[error("endianness must be `le` or `be`")]
    InvalidEndianness,
    #[error("{attr} does not match {pat} actual value {actual:?}")]
    InvalidAttrType {
        attr: String,
        pat: String,
        actual: yaml_rust::Yaml,
    },
    #[error("{0} not found")]
    RequiredAttrNotFound(String),
}

macro_rules! get_attribute {
    ($data:ident | $attr:literal as $pat:pat => $e:expr) => {
        match $data.get(&Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $pat => Ok($e),
                _ => Err(crate::utils::MacroError::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                    actual: s.clone(),
                }),
            },
            None => Err(crate::utils::MacroError::RequiredAttrNotFound(
                $attr.to_owned(),
            )),
        }
    };
}
pub(crate) use get_attribute;

/// Converts a snake case string to an upper camel case string.
pub fn sc_to_ucc<T: AsRef<str>>(string: T) -> String {
    let mut result = String::new();

    for w in string.as_ref().split('_') {
        let first_letter = w[0..1].to_uppercase();
        let rest_of_word = &w[1..w.len()];
        result.push_str(&first_letter);
        result.push_str(rest_of_word);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc_to_ucc_test() {
        let input = vec!["example_id", "oneword", "num_at_end1", "num_at_end_2"];

        let output: Vec<String> = input.into_iter().map(sc_to_ucc).collect();

        assert_eq!(
            output,
            vec!["ExampleId", "Oneword", "NumAtEnd1", "NumAtEnd2"]
        );
    }
}
