use std::collections::HashMap;

pub struct Database {
    pub data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut db = Database::new();

        db.set("name", "john");

        assert_eq!(db.get("name"), Some("john"));
    }

    #[test]
    fn test_update() {
        let mut db = Database::new();

        db.set("name", "john");
        db.set("name", "mary");

        assert_eq!(db.get("name"), Some("mary"));
    }

    #[test]
    fn test_get_missing_key() {
        let db = Database::new();

        assert_eq!(db.get("missing"), None);
    }

    #[test]
    fn test_delete() {
        let mut db = Database::new();

        db.set("name", "john");

        assert!(db.delete("name"));
        assert!(!db.exists("name"));
    }

    #[test]
    fn test_exists() {
        let mut db = Database::new();

        db.set("name", "john");

        assert!(db.exists("name"));
        assert!(!db.exists("missing"));
    }

    #[test]
    fn test_clear() {
        let mut db = Database::new();

        db.set("name", "john");
        db.set("age", "20");

        db.clear();

        assert!(!db.exists("name"));
        assert!(!db.exists("age"));
    }
}
