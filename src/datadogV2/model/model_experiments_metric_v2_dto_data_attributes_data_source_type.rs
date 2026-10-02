// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsMetricV2DTODataAttributesDataSourceType {
    DATADOG,
    DATADOG_REFERENCE_TABLE,
    CUSTOMER_WAREHOUSE,
    IMPORTED,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsMetricV2DTODataAttributesDataSourceType {
    fn to_string(&self) -> String {
        match self {
            Self::DATADOG => String::from("DATADOG"),
            Self::DATADOG_REFERENCE_TABLE => String::from("DATADOG_REFERENCE_TABLE"),
            Self::CUSTOMER_WAREHOUSE => String::from("CUSTOMER_WAREHOUSE"),
            Self::IMPORTED => String::from("IMPORTED"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsMetricV2DTODataAttributesDataSourceType {
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

impl<'de> Deserialize<'de> for ExperimentsMetricV2DTODataAttributesDataSourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DATADOG" => Self::DATADOG,
            "DATADOG_REFERENCE_TABLE" => Self::DATADOG_REFERENCE_TABLE,
            "CUSTOMER_WAREHOUSE" => Self::CUSTOMER_WAREHOUSE,
            "IMPORTED" => Self::IMPORTED,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
