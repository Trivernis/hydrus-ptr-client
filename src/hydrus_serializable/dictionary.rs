#![allow(unused)]
use crate::constants::HYDRUS_TYPE_DICTIONARY;
use crate::hydrus_serializable::HydrusSerializable;
use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct HydrusDictionary {
    list_sim_sim: Vec<(Value, Value)>,
    list_sim_ser: Vec<(Value, Value)>,
    list_ser_sim: Vec<(Value, Value)>,
    list_ser_ser: Vec<(Value, Value)>,
}

impl HydrusSerializable for HydrusDictionary {
    fn type_id() -> u64 {
        HYDRUS_TYPE_DICTIONARY
    }
}

impl HydrusDictionary {
    /// Returns the first value for a given key
    pub fn get(&self, key: &Value) -> Option<&Value> {
        self._get(key).into_iter().next()
    }

    /// Returns a single value by using a string as a key
    pub fn get_by_str<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        self.get(&key.as_ref().into())
    }

    /// Removes an element deserialized as the given type from the dictionary
    pub fn take<D: DeserializeOwned>(&mut self, key: &Value) -> Result<D> {
        let value = self
            ._take(key)
            .ok_or_else(|| Error::MissingProperty(key.to_string()))?;

        serde_json::from_value::<D>(value).map_err(Error::from)
    }

    /// Removes an element with the given string key deserialized as the requested type from the dictionary
    pub fn take_by_str<D: DeserializeOwned>(&mut self, key: &str) -> Result<D> {
        self.take(&key.into())
    }

    /// Returns all values for a given key
    fn _get(&self, key: &Value) -> Vec<&Value> {
        self.list_sim_sim
            .iter()
            .chain(self.list_sim_ser.iter())
            .chain(self.list_ser_sim.iter())
            .chain(self.list_ser_ser.iter())
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v)
            .collect()
    }

    fn _take(&mut self, key: &Value) -> Option<Value> {
        for list in [
            &mut self.list_sim_sim,
            &mut self.list_sim_ser,
            &mut self.list_ser_sim,
            &mut self.list_ser_ser,
        ] {
            let index = list.iter().position(|(k, _)| k == key);

            if let Some(index) = index {
                return Some(list.swap_remove(index).1);
            }
        }

        None
    }
}
