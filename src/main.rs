mod cache;
mod error;

use cache::cache::Cache;

fn main() {

    fn test_lru_cache() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));


        // Récupérer une valeur existante
        match cache.get(&"A") {
            Ok(value) => println!("Valeur trouvée : {}", value),
            Err(e) => println!("Erreur : {}", e),
        }

        // Récupérer une clé inexistante
        match cache.get(&"B") {
            Ok(value) => println!("Valeur trouvée : {}", value),
            Err(e) => println!("Erreur : {}", e),
        }
    }
}
