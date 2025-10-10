use mega_store_search::{Indexer, Product};

fn main() -> anyhow::Result<()> {
    let mut indexer = Indexer::new();

    let p1 = Product::new(1, "Smartphone Galaxy S", Some("Samsung"), Some("eletronicos"), 299_900);
    let p2 = Product::new(2, "Fone de Ouvido Bluetooth", Some("Sony"), Some("audio"), 99_900);
    let p3 = Product::new(3, "Smart TV 55\"", Some("LG"), Some("eletronicos"), 1299_00);
    let p4 = Product::new(4, "Galaxy Buds", Some("Samsung"), Some("audio"), 199_00);

    indexer.insert(p1);
    indexer.insert(p2);
    indexer.insert(p3);
    indexer.insert(p4);

    println!("Total indexados: {}", indexer.len());

    let results = indexer.search_by_brand("Samsung");
    println!("Busca por brand 'Samsung':");
    for r in results {
        println!("- {} (id={})", r.name, r.id);
    }

    let q = "smartphone";
    let by_name = indexer.search_by_name(q);
    println!("Busca por name '{}': found {}", q, by_name.len());
    for r in by_name {
        println!("- {} @ R${:.2}", r.name, r.price_cents as f64 / 100.0);
    }

    Ok(())
}
