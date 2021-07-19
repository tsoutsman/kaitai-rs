use yaml_rust::yaml;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct MetaInformation {
    pub name: String,
    pub doc: String,
}

pub(crate) fn parse_meta(_meta: &yaml::Hash) -> Option<MetaInformation> {
    todo!()
}
