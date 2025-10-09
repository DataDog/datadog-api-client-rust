// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceTableSourceType {
    LOCAL_FILE,
    S3,
    GCS,
    AZURE,
    SERVICENOW,
    SALESFORCE,
    DATABRICKS,
    SNOWFLAKE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ReferenceTableSourceType {
    fn to_string(&self) -> String {
        match self {
            Self::LOCAL_FILE => String::from("LOCAL_FILE"),
            Self::S3 => String::from("S3"),
            Self::GCS => String::from("GCS"),
            Self::AZURE => String::from("AZURE"),
            Self::SERVICENOW => String::from("SERVICENOW"),
            Self::SALESFORCE => String::from("SALESFORCE"),
            Self::DATABRICKS => String::from("DATABRICKS"),
            Self::SNOWFLAKE => String::from("SNOWFLAKE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ReferenceTableSourceType {
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

impl<'de> Deserialize<'de> for ReferenceTableSourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LOCAL_FILE" => Self::LOCAL_FILE,
            "S3" => Self::S3,
            "GCS" => Self::GCS,
            "AZURE" => Self::AZURE,
            "SERVICENOW" => Self::SERVICENOW,
            "SALESFORCE" => Self::SALESFORCE,
            "DATABRICKS" => Self::DATABRICKS,
            "SNOWFLAKE" => Self::SNOWFLAKE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
