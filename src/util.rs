use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub const NAMESPACE_SEPARATOR: char = ':';

    pub fn new(namespace: &str, path: &str) -> Self {
        Self {
            namespace: namespace.to_owned(),
            path: path.to_string(),
        }
    }

    pub fn from(id: String) -> Option<Self> {
        let s = id.split_once(':')?;
        Some(Self::new(s.0, s.1))
    }

    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}
