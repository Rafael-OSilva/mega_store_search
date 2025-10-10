use serde::{Deserialize, Serialize};

/// Representação simples de um produto no catálogo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub price_cents: u64,
}

impl Product {
    pub fn new(
        id: u64,
        name: &str,
        brand: Option<&str>,
        category: Option<&str>,
        price_cents: u64,
    ) -> Self {
        Self {
            id,
            name: name.to_lowercase(),
            brand: brand.map(|s| s.to_lowercase()),
            category: category.map(|s| s.to_lowercase()),
            price_cents,
        }
    }
}
