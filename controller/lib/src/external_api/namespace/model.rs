use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NamespaceDTO {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Namespace {
    pub id: String,
    pub name: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {}
