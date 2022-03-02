use crate::hydrus_serializable::wrapper::HydrusSerWrapper;
use crate::FromJson;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt::Formatter;
use std::marker::PhantomData;

pub mod dictionary;
pub mod wrapper;

pub trait HydrusSerializable: DeserializeOwned {
    fn type_id() -> u64;
}

impl<T> FromJson for T
where
    T: HydrusSerializable,
{
    fn from_json(value: Value) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let wrapper = serde_json::from_value::<HydrusSerWrapper<T>>(value)?;

        Ok(wrapper.inner)
    }
}

pub trait ConstNumberTrait {
    fn value() -> u64;
}

impl<T> ConstNumberTrait for T
where
    T: HydrusSerializable,
{
    fn value() -> u64 {
        T::type_id()
    }
}

#[derive(Clone, Debug)]
pub struct SerializableId<T: ConstNumberTrait>(u64, PhantomData<T>);

impl<'de, T: ConstNumberTrait> Deserialize<'de> for SerializableId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(SerIdVisitor(PhantomData))
    }
}

struct SerIdVisitor<T>(PhantomData<T>);

impl<'de, T: ConstNumberTrait> Visitor<'de> for SerIdVisitor<T> {
    type Value = SerializableId<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "an unsigned integer equal to {}", T::value())
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
        let expected_value = T::value();
        if v != expected_value {
            Err(E::custom(format!("value not equal to {}", expected_value)))
        } else {
            Ok(SerializableId(expected_value, PhantomData))
        }
    }
}
