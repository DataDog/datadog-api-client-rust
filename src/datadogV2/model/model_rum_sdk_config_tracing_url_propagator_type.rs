// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RumSdkConfigTracingUrlPropagatorType {
    DATADOG,
    B3,
    B3MULTI,
    TRACECONTEXT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RumSdkConfigTracingUrlPropagatorType {
    fn to_string(&self) -> String {
        match self {
            Self::DATADOG => String::from("datadog"),
            Self::B3 => String::from("b3"),
            Self::B3MULTI => String::from("b3multi"),
            Self::TRACECONTEXT => String::from("tracecontext"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RumSdkConfigTracingUrlPropagatorType {
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

impl<'de> Deserialize<'de> for RumSdkConfigTracingUrlPropagatorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "datadog" => Self::DATADOG,
            "b3" => Self::B3,
            "b3multi" => Self::B3MULTI,
            "tracecontext" => Self::TRACECONTEXT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
