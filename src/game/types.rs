#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum NodeType {
    #[default]
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
    const ALL: [NodeType; 9] = [
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

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum ToolType {
    Add(NodeType),
    Delete,
    Select,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Add(NodeType::default())
    }
}
