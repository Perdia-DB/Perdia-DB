use serde::{Deserialize, de::Visitor, Serialize};

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

pub struct Data {
    pub data_type: DataType,
    pub data: DataUnion,
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
        deserializer.deserialize_string(DATAVisitor)   
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
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as i64;
        Ok(Data {
            data_type: DataType::INTEGER,
            data: DataUnion { integer: v },
        })
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as f64;
        Ok(Data {
            data_type: DataType::FLOAT,
            data: DataUnion { float: v },
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v = v as f64;
        Ok(Data {
            data_type: DataType::FLOAT,
            data: DataUnion { float: v },
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v: &'static str = Box::leak(v.to_string().into_boxed_str());
        Ok(Data {
            data_type: DataType::STRING,
            data: DataUnion { string: v },
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
            E: serde::de::Error, 
    {
        let v: &'static str = Box::leak(v.into_boxed_str());
        Ok(Data {
            data_type: DataType::STRING,
            data: DataUnion { string: v },
        })
    }
}