use serde::{Deserialize, de::{Visitor, self}, Serialize, ser::SerializeStruct};

use crate::query;

use super::structure::Instance;

pub union DataUnion {
    pub string: &'static str,
    pub integer: i64,
    pub float: f64,
}

impl From<Option<&'static str>> for DataUnion {
    fn from(string: Option<&'static str>) -> Self {
        DataUnion { string: string.unwrap_or_default() }
    }
}

impl From<Option<String>> for DataUnion {
    fn from(string: Option<String>) -> Self {
        DataUnion { string: Box::leak(string.unwrap_or_default().into_boxed_str()) }
    }
}

impl From<Option<i64>> for DataUnion {
    fn from(int: Option<i64>) -> Self {
        DataUnion { integer: int.unwrap_or_default() }
    }
}

impl From<Option<f64>> for DataUnion {
    fn from(float: Option<f64>) -> Self {
        DataUnion { float: float.unwrap_or_default() }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    STRING,
    INTEGER,
    FLOAT,
}

/// A wrapper for the [`DataUnion`] union to make it typesafe.
pub struct Data {
    pub data_type: DataType,
    pub data: DataUnion,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data").field("value", {
            match self.data_type {
                DataType::STRING => unsafe { &self.data.string },
                DataType::INTEGER => unsafe { &self.data.integer },
                DataType::FLOAT => unsafe { &self.data.float },
            }
        }).finish()
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.data_type == other.data_type && match self.data_type {
            DataType::STRING => unsafe {self.data.string == other.data.string},
            DataType::INTEGER => unsafe {self.data.integer == other.data.integer},
            DataType::FLOAT => unsafe {self.data.float == other.data.float},
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        let data = match self.data_type {
            DataType::STRING => unsafe { DataUnion { string: self.data.string } },
            DataType::INTEGER => unsafe { DataUnion { integer: self.data.integer } },
            DataType::FLOAT => unsafe { DataUnion { float: self.data.float } },
        };
        Self { data_type: self.data_type.clone(), data }
    }
}

impl From<&'static str> for Data {
    fn from(string: &'static str) -> Self {
        Self {
            data_type: DataType::STRING,
            data: DataUnion::from(Some(string))
        }
    }
}

impl From<String> for Data {
    fn from(string: String) -> Self {
        Self {
            data_type: DataType::STRING,
            data: DataUnion::from(Some(string))
        }
    }
}

impl From<i64> for Data {
    fn from(int: i64) -> Self {
        Self {
            data_type: DataType::INTEGER,
            data: DataUnion::from(Some(int))
        }
    }
}

impl From<f64> for Data {
    fn from(float: f64) -> Self {
        Self {
            data_type: DataType::FLOAT,
            data: DataUnion::from(Some(float))
        }
    }
}

pub struct DATAVisitor;

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        match self.data_type {
            DataType::STRING => unsafe { serializer.serialize_str(self.data.string) },
            DataType::INTEGER => unsafe { serializer.serialize_i64(self.data.integer) },
            DataType::FLOAT => unsafe { serializer.serialize_f64(self.data.float) },
        }
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_any(DATAVisitor)   
    }
}

impl<'de> Visitor<'de> for DATAVisitor {
    type Value = Data;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer, float or string")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data::from(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as f64;
        Ok(Data::from(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as f64;
        Ok(Data::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v: &'static str = Box::leak(v.to_string().into_boxed_str());
        Ok(Data::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v: &'static str = Box::leak(v.into_boxed_str());
        Ok(Data::from(v))
    }
}

impl Serialize for Instance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut structure = serializer.serialize_struct("Instance", 3)?;
        structure.serialize_field("name", &self.name)?;
        structure.serialize_field("template", &self.template.name)?;
        structure.serialize_field("data", &self.data)?;
        structure.end()
    }
}


impl<'de> Deserialize<'de> for Instance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: serde::Deserializer<'de> {
        enum Field {
            Name,
            Template,
            Data
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`name`, `template` or `data`.")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, {
                        match v {
                            "name" => Ok(Field::Name),
                            "template" => Ok(Field::Template),
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(v, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct InstanceVisitor;

        impl<'de> Visitor<'de> for InstanceVisitor {
            type Value = Instance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Instance")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>, {
                let name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let template = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let data = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let template = query::backend::copy_template(template, 0).expect("Failed to get Template");
                Ok(Self::Value {
                    name, template, data
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>, {
                let mut name = None;
                let mut template = None;
                let mut data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        },
                        Field::Template => {
                            if template.is_some() {
                                return Err(de::Error::duplicate_field("template"));
                            }
                            template = Some(map.next_value()?);
                        },
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let template = template.ok_or_else(|| de::Error::missing_field("template"))?;
                let template = query::backend::copy_template(template, 0).expect("Failed to get Template");
                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                Ok(Self::Value {
                    name, template, data
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "template", "data"];
        deserializer.deserialize_struct("Instance", FIELDS, InstanceVisitor)
    }
}