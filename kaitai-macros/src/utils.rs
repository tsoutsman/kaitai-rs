use yaml_rust::{yaml, Yaml};

pub(crate) fn get_item_attribute(item: &yaml::Hash, attribute: &str) -> Option<String> {
    item.get(&Yaml::String(attribute.to_owned())).map(|i| {
        if let Yaml::String(ref string) = i {
            string.clone()
        } else {
            panic!("#123")
        }
    })
}
