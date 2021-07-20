macro_rules! get_attribute {
    ($data:ident | $attr:literal as $type:pat => $e:expr) => {
        match $data.get(&Yaml::String($attr.to_owned())) {
            Some(s) => match s {
                $type => Ok($e),
                _ => Err(concat!($attr, " is not a ", stringify!($type))),
            },
            None => Err(concat!($attr, " not found")),
        }
    };
}
pub(crate) use get_attribute;
