#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum NodeType {
    #[default]
    None,
    Internet,
    LoadBalancer,
    Firewall,
    Database,
    Compute,
    Storage,
    Queue,
    Cache,
    CDN,
}

impl NodeType {
    const ALL: [NodeType; 10] = [
        Self::None,
        Self::Internet,
        Self::LoadBalancer,
        Self::Firewall,
        Self::Database,
        Self::Compute,
        Self::Storage,
        Self::Queue,
        Self::Cache,
        Self::CDN,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "Dlt",
            Self::Internet => "W.W.W",
            Self::LoadBalancer => "LB",
            Self::Firewall => "FW",
            Self::Database => "DB",
            Self::Compute => "Cmput",
            Self::Storage => "Strg",
            Self::Queue => "Que",
            Self::Cache => "Cch",
            Self::CDN => "CDN",
        }
    }

    pub fn all() -> impl Iterator<Item = (NodeType, &'static str)> {
        Self::ALL.into_iter().map(|t| (t, t.name()))
    }
}
