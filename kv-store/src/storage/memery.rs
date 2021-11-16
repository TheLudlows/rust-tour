use dashmap::{mapref::one::Ref, DashMap};

use crate::{error::KvError, Kvpair, Value};

use super::Storage;

#[derive(Default)]
pub struct Memtable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl Memtable {
    pub fn new() -> Self {
        Memtable::default()
    }

    pub fn get_or_default(&self, table: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(table) {
            Some(map) => map,
            None => {
                let entry = self.tables.entry(table.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for Memtable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let t = self.get_or_default(table);
        Ok(t.get(key).map(|entry| entry.value().clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let t = self.get_or_default(table);
        Ok(t.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let t = self.get_or_default(table);
        Ok(t.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let t = self.get_or_default(table);
        Ok(t.remove(key).map(|(_,v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let t = self.get_or_default(table);
        Ok(t.iter().map(|e| Kvpair::new(e.key().clone(), e.value().clone()))
        .collect())
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        let t = self.get_or_default(table).clone();
         Ok(Box::new(t.into_iter().map(|(k,v)| Kvpair::new(k,v))))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_or_create_table_should_work() {
        let store = Memtable::new();
        assert!(!store.tables.contains_key("t1"));
        store.get_or_default("t1");
        assert!(store.tables.contains_key("t1"));
    }
}
