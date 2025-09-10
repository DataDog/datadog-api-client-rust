// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GCPMonitoredResourceConfigType {
    CLOUD_FUNCTION,
    CLOUD_RUN_REVISION,
    GCE_INSTANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for GCPMonitoredResourceConfigType {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUD_FUNCTION => String::from("cloud_function"),
            Self::CLOUD_RUN_REVISION => String::from("cloud_run_revision"),
            Self::GCE_INSTANCE => String::from("gce_instance"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for GCPMonitoredResourceConfigType {
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

impl<'de> Deserialize<'de> for GCPMonitoredResourceConfigType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cloud_function" => Self::CLOUD_FUNCTION,
            "cloud_run_revision" => Self::CLOUD_RUN_REVISION,
            "gce_instance" => Self::GCE_INSTANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
