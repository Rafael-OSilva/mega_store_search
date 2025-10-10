use mega_store_search::{Indexer, Product};

#[test]
fn test_insert_and_search() {
    let mut indexer = Indexer::new();
    let p1 = Product::new(10, "Cafeteira Eletrica", Some("Philips"), Some("cozinha"), 249_00);
    indexer.insert(p1.clone());

    assert_eq!(indexer.len(), 1);

    let res = indexer.search_by_name("cafeteira");
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], &p1);

    let res_brand = indexer.search_by_brand("philips");
    assert_eq!(res_brand.len(), 1);
}
