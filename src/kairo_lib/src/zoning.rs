// Zoning Protocol structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zone {
    Public,
    Sensitive,
    Restricted,
    Experimental,
}

impl Zone {
    pub fn from_label(label: &str) -> Option<Self> {
        match label.to_lowercase().as_str() {
            "public" => Some(Self::Public),
            "sensitive" => Some(Self::Sensitive),
            "restricted" => Some(Self::Restricted),
            "experimental" => Some(Self::Experimental),
            _ => None,
        }
    }
}
