// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Comparator {
	EQ,
	GT,
	GT_EQ,
	LT,
	LT_EQ,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for Comparator {
    fn to_string(&self) -> String {
        match self {
            Self::EQ => String::from("="),
            Self::GT => String::from(">"),
            Self::GT_EQ => String::from(">="),
            Self::LT => String::from("<"),
            Self::LT_EQ => String::from("<="),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for Comparator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for Comparator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"=" => Self::EQ,">" => Self::GT,">=" => Self::GT_EQ,"<" => Self::LT,"<=" => Self::LT_EQ,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
