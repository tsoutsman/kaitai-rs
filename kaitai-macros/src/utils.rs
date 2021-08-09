use thiserror::Error;

pub type Result<T> = std::result::Result<T, MacroError>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StackTrace(pub Vec<String>);

impl std::fmt::Display for StackTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for f in &self.0 {
            result.push_str(f);
        }

        write!(f, "{}", result)
    }
}

impl StackTrace {
    pub fn from<T: AsRef<str>>(root: T) -> Self {
        Self(vec![root.as_ref().to_owned()])
    }
}

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MacroError {
    #[error("attribute in seq invalid: {0:?}\nstack trace: {1}")]
    InvalidAttribute(yaml_rust::Yaml, StackTrace),
    #[error("endianness must be `le` or `be`\nstack trace: {0}")]
    InvalidEndianness(StackTrace),
    #[error("{attr} does not match {pat} actual value {actual:?}\nstack trace: {st}")]
    InvalidAttrType {
        attr: String,
        pat: String,
        actual: yaml_rust::Yaml,
        st: StackTrace,
    },
    #[error("{0} not found\nstack trace: {1}")]
    RequiredAttrNotFound(String, StackTrace),
}

impl MacroError {
    /// Pushes a frame onto the [`StackFrame`] of the error.
    pub fn push<T: AsRef<str>>(&mut self, frame: T) {
        let st: &mut StackTrace = match self {
            MacroError::InvalidAttribute(_, ref mut st) => st,
            MacroError::InvalidEndianness(ref mut st) => st,
            MacroError::InvalidAttrType { ref mut st, .. } => st,
            MacroError::RequiredAttrNotFound(_, ref mut st) => st,
        };
        st.0.push(frame.as_ref().to_owned());
    }

    /// Returns a [`MacroError`] that is the product of pushing the given frame onto the end of self.
    /// The function does not actually mutate self.
    pub fn with<T: AsRef<str>>(&self, frame: T) -> Self {
        let mut result = self.clone();
        result.push(frame);
        result
    }
}

macro_rules! get_attribute {
    ($data:ident; $attr:literal as $pat:pat => $e:expr; $fname:expr$(;)?) => {
        match $data.get(&yaml_rust::Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $pat => Ok($e),
                _ => Err(crate::utils::MacroError::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                    actual: s.clone(),
                    st: crate::utils::StackTrace::from($fname),
                }),
            },
            None => Err(crate::utils::MacroError::RequiredAttrNotFound(
                $attr.to_owned(),
                crate::utils::StackTrace::from($fname),
            )),
        }
    };
}
pub(crate) use get_attribute;

macro_rules! assert_pattern {
    ($data:ident; $pat:pat => $e:expr; attr: $attr:expr, st: $fname:expr$(;)?) => {
        match $data {
            $pat => $e,
            _ => {
                return Err(crate::utils::MacroError::InvalidAttrType {
                    attr: $attr.to_owned(),
                    pat: stringify!($pat).to_owned(),
                    actual: $data.clone(),
                    st: crate::utils::StackTrace::from($fname),
                })
            }
        }
    };
}
pub(crate) use assert_pattern;

/// This macro, as its name suggests, propagates an error. It is used like the `?` operator except
/// it pushes the function name onto the stack frame if there is an error.
macro_rules! prop_err {
    ($e:expr; $s:expr) => {
        match $e {
            Ok(o) => o,
            Err(e) => return Err(e.with($s)),
        }
    };
}
pub(crate) use prop_err;

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
