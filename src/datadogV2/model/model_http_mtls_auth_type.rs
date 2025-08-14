// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HTTPMtlsAuthType {
    HTTPMTLSAUTH,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for HTTPMtlsAuthType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTPMTLSAUTH => String::from("HTTPMtlsAuth"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for HTTPMtlsAuthType {
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

impl<'de> Deserialize<'de> for HTTPMtlsAuthType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "HTTPMtlsAuth" => Self::HTTPMTLSAUTH,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
