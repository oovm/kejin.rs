use super::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use crate::EnhanceMap;





impl<T> Serialize for EnhanceMap<T>  where T:Ord + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.mapping.serialize(serializer)
    }
}

impl <T> Serialize for EnhanceLevel<T> where T:Ord + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(4))?;
        if !self.relative_rate.is_empty() {
            map.serialize_entry("relative_rate", &self.relative_rate)?;
        }
        if !self.absolute_rate.is_empty() {
            map.serialize_entry("absolute_rate", &self.absolute_rate)?;
        }
        if self.broken_rate != 0.0 {
            map.serialize_entry("broken_rate", &self.broken_rate)?;
        }
        if !self.enhance_cost.is_empty() {
            map.serialize_entry("enhance_cost", &self.enhance_cost)?;
        }
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for EnhanceMap<T> where T: Ord +Deserialize<'de>  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut default = EnhanceMap::default();
        deserializer.deserialize_map(&mut default)?;
        Ok(default)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_map(place)
    }
}

impl<'de, T> Deserialize<'de> for EnhanceLevel<T> where T: Ord +Deserialize<'de>  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_map(EnhanceLevel::default())
    }
}



impl<'de, T> Visitor<'de> for &mut EnhanceMap<T> where T:Ord + Deserialize<'de> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        let mut mapping = BTreeMap::new();
        while let Some((key, value)) = map.next_entry::<u16, EnhanceLevel<T>>()? {
            mapping.insert(key, value);
        }
        Ok(())
    }
}

impl<'de, T> Visitor<'de> for  EnhanceLevel<T> where T:Ord + Deserialize<'de> {
    type Value = EnhanceLevel<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
              "relative"|  "relative_rate" => {
                    self.relative_rate = map.next_value()?;

                }
               "absolute"| "absolute_rate" => {
                    self.absolute_rate = map.next_value()?;
                }
             "broken"|   "broken_rate" => {
                    self.broken_rate = map.next_value()?;
                }
              "cost"|  "enhance_cost" => {
                    self.enhance_cost = map.next_value()?;
                }
                _ => {
                     Err(Error::unknown_field(&key, &["relative_rate", "absolute_rate", "broken_rate", "enhance_cost"]))?
                }
            }
        }
        Ok(self)
    }
}