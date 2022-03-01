use crate::constants::HYDRUS_TYPE_DICTIONARY;
use crate::hydrus_serializable::HydrusSerializable;
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
    pub fn get_one(&self, key: &Value) -> Option<&Value> {
        self.get(key).into_iter().next()
    }

    /// Returns all values for a given key
    pub fn get(&self, key: &Value) -> Vec<&Value> {
        self.list_sim_sim
            .iter()
            .chain(self.list_sim_ser.iter())
            .chain(self.list_ser_sim.iter())
            .chain(self.list_ser_ser.iter())
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v)
            .collect()
    }
}
