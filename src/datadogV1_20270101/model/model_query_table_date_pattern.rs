// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryTableDatePattern {
	MMM_DD_YYYY,
	MMM_D,
	MMM_D__YYYY,
	MM_DD_YYYY,
	DD_MM_YYYY,
	MMMM_D__YYYY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for QueryTableDatePattern {
    fn to_string(&self) -> String {
        match self {
            Self::MMM_DD_YYYY => String::from("MMM dd yyyy"),
            Self::MMM_D => String::from("MMM d"),
            Self::MMM_D__YYYY => String::from("MMM d, yyyy"),
            Self::MM_DD_YYYY => String::from("MM/dd/yyyy"),
            Self::DD_MM_YYYY => String::from("dd/MM/yyyy"),
            Self::MMMM_D__YYYY => String::from("MMMM d, yyyy"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for QueryTableDatePattern {
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

impl<'de> Deserialize<'de> for QueryTableDatePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"MMM dd yyyy" => Self::MMM_DD_YYYY,"MMM d" => Self::MMM_D,"MMM d, yyyy" => Self::MMM_D__YYYY,"MM/dd/yyyy" => Self::MM_DD_YYYY,"dd/MM/yyyy" => Self::DD_MM_YYYY,"MMMM d, yyyy" => Self::MMMM_D__YYYY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
