use std::collections::HashMap;

use crate::product::Product;

/// Indexer que mantém índices invertidos simples para nome, marca e categoria.
#[derive(Debug, Default)]
pub struct Indexer {
    products: HashMap<u64, Product>,
    name_index: HashMap<String, Vec<u64>>,
    brand_index: HashMap<String, Vec<u64>>,
    category_index: HashMap<String, Vec<u64>>,
}

impl Indexer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insere um produto no catálogo e atualiza índices.
    pub fn insert(&mut self, product: Product) {
        let id = product.id;

        // Index name terms (split em palavras simples)
        let name_terms = product.name.split_whitespace().map(|s| s.to_string());

        for term in name_terms {
            self.name_index.entry(term).or_default().push(id);
        }

        if let Some(brand) = &product.brand {
            self.brand_index.entry(brand.clone()).or_default().push(id);
        }

        if let Some(category) = &product.category {
            self.category_index.entry(category.clone()).or_default().push(id);
        }

        self.products.insert(id, product);
    }

    /// Busca por nome (todas as palavras precisam aparecer).
    pub fn search_by_name(&self, query: &str) -> Vec<&Product> {
        let terms: Vec<String> = query
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if terms.is_empty() {
            return vec![];
        }

        let mut sets: Vec<&Vec<u64>> = Vec::new();
        for term in &terms {
            if let Some(v) = self.name_index.get(term) {
                sets.push(v);
            } else {
                return vec![];
            }
        }

        // Interseção simples
        let mut result_ids: Vec<u64> = sets[0].clone();
        for s in sets.iter().skip(1) {
            result_ids = result_ids.into_iter().filter(|id| s.contains(id)).collect();
        }

        result_ids.sort_unstable();
        result_ids
            .into_iter()
            .filter_map(|id| self.products.get(&id))
            .collect()
    }

    pub fn search_by_brand(&self, brand: &str) -> Vec<&Product> {
        let key = brand.to_lowercase();
        if let Some(ids) = self.brand_index.get(&key) {
            let mut ids = ids.clone();
            ids.sort_unstable();
            ids.into_iter().filter_map(|id| self.products.get(&id)).collect()
        } else {
            vec![]
        }
    }

    pub fn search_by_category(&self, category: &str) -> Vec<&Product> {
        let key = category.to_lowercase();
        if let Some(ids) = self.category_index.get(&key) {
            let mut ids = ids.clone();
            ids.sort_unstable();
            ids.into_iter().filter_map(|id| self.products.get(&id)).collect()
        } else {
            vec![]
        }
    }

    pub fn get(&self, id: u64) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn len(&self) -> usize {
        self.products.len()
    }
}
