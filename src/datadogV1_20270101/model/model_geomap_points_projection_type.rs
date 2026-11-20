// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeomapPointsProjectionType {
	GEOMAP_POINTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for GeomapPointsProjectionType {
    fn to_string(&self) -> String {
        match self {
            Self::GEOMAP_POINTS => String::from("geomap_points"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for GeomapPointsProjectionType {
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

impl<'de> Deserialize<'de> for GeomapPointsProjectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"geomap_points" => Self::GEOMAP_POINTS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
