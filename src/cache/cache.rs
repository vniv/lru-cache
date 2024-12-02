use std::collections::HashMap;
use std::fmt::Debug;
use crate::error::error::MyError;
use std::hash::Hash;

// Objet du cache
pub struct Cache<K, V> {
    size: usize,
    pub element: HashMap<K, V>,
    values_order: Vec<K>,
}

// Interface du cache
pub trait CacheTrait<K, V>
where
    K: Eq + Debug + Clone + Hash,
{
    fn new(size: usize) -> Self;

    fn put(&mut self, key: K, value: V) -> Result<(), MyError>;

    fn get(&self, key: &K) -> Result<&V, MyError>;
}

// Implémentation des méthodes pour le cache
impl<K, V> Cache<K, V>
where
    K: Eq + Debug + Clone + Hash,
{
    /// Création d'une nouvelle instance de cache
    ///
    /// # Exemple
    /// ```
    /// use evaluation::cache::cache::Cache;
    ///
    /// let cache: Cache<&str, String> = Cache::new(3);
    /// assert_eq!(cache.element.len(), 0);
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            size,
            element: HashMap::new(),
            values_order: Vec::new(),
        }
    }

    /// Ajoute une clé et une valeur au cache
    ///
    /// Retourne une erreur si la clé existe déjà.
    ///
    /// # Exemple
    /// ```
    /// use evaluation::cache::cache::Cache;
    /// use evaluation::error::error::MyError;
    ///
    /// let mut cache: Cache<&str, String> = Cache::new(3);
    /// assert!(cache.put("A", String::from("value_a")).is_ok());
    /// ```
    pub fn put(&mut self, key: K, value: V) -> Result<(), MyError> {
        if self.element.contains_key(&key) {
            // Si la valeur a enregistré existe déjà alors je lève un message d'erreur
            return Err(MyError::Custom(format!(
                "La clé existe déjà: {:?}",
                key
            )));
        }

        // Si le cache est déjà entièrement rempli, alors je supprime le dernier élément
        if self.values_order.len() == self.size {
            let oldest_key = self.values_order.remove(0);
            self.element.remove(&oldest_key);
        }

        // Ajoutez le nouvel élément et mettez à jour l'ordre
        self.values_order.push(key.clone());
        self.element.insert(key, value);
        Ok(())
    }

    /// Récupère une valeur associée à une clé
    ///
    /// Retourne une erreur si la clé n'existe pas.
    ///
    /// # Exemple
    /// ```
    /// use evaluation::cache::cache::Cache;
    /// use evaluation::error::error::MyError;
    ///
    /// let mut cache: Cache<&str, String> = Cache::new(3);
    /// cache.put("A", String::from("value_a")).unwrap();
    ///
    /// // Comparaison explicite des valeurs
    /// assert_eq!(cache.get(&"A").unwrap(), &String::from("value_a"));
    ///
    /// // Comparaison des erreurs
    /// let result = cache.get(&"B");
    /// assert!(matches!(result, Err(MyError::Custom(_))));
    /// ```
    pub fn get(&self, key: &K) -> Result<&V, MyError> {
        self.element
            .get(key)
            .ok_or_else(|| MyError::Custom(format!("La clé n'existe pas: {:?}", key)))
    }
}