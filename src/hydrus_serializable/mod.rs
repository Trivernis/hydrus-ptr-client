use crate::hydrus_serializable::dictionary::HydrusDictionary;
use serde::de::{DeserializeOwned, EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub mod dictionary;
pub mod wrapper;

pub trait HydrusSerializable: DeserializeOwned {
    fn type_id() -> u64;
}

#[derive(Clone, Debug)]
pub struct SerializableId<T: HydrusSerializable>(u64, PhantomData<T>);

impl<'de, T: HydrusSerializable> Deserialize<'de> for SerializableId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(SerIdVisitor(PhantomData))
    }
}

struct SerIdVisitor<T>(PhantomData<T>);

impl<'de, T: HydrusSerializable> Visitor<'de> for SerIdVisitor<T> {
    type Value = SerializableId<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "an unsigned integer equal to {}", T::type_id())
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let expected_value = T::type_id();
        if v != expected_value {
            Err(E::custom(format!("type not equal to {}", expected_value)))
        } else {
            Ok(SerializableId(expected_value, PhantomData))
        }
    }
}
