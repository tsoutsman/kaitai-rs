use crate::utils::get_attribute;

use yaml_rust::Yaml;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct SeqItem {
    id: String,
    ks_type: String,
}

impl SeqItem {
    #[allow(dead_code)]
    pub fn rust_type(&self) -> &str {
        match &self.ks_type[..] {
            "u1" => "u8",
            "u2" => "u16",
            "u4" => "u32",
            "u8" => "u64",
            "s1" => "i8",
            "s2" => "i16",
            "s4" => "i32",
            "s8" => "i64",
            "f4" => "f32",
            "f8" => "f64",
            &_ => panic!(),
        }
    }
}

pub(crate) fn parse_seq(seq: &[Yaml]) -> Vec<SeqItem> {
    seq.iter()
        .map(|item| match item {
            Yaml::Hash(h) => SeqItem {
                id: get_attribute!(h | "id" as Yaml::String(s) => s.clone())
                    .expect("error fetching meta > id: "),
                ks_type: get_attribute!(h | "type" as Yaml::String(s) => s.clone())
                    .expect("error fetching meta > type: "),
            },
            _ => panic!(),
        })
        .collect()
}
