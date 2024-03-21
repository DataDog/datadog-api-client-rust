// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetentionFilterAllType {
    SPANS_SAMPLING_PROCESSOR,
    SPANS_ERRORS_SAMPLING_PROCESSOR,
    SPANS_APPSEC_SAMPLING_PROCESSOR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RetentionFilterAllType {
    fn to_string(&self) -> String {
        match self {
            Self::SPANS_SAMPLING_PROCESSOR => String::from("spans-sampling-processor"),
            Self::SPANS_ERRORS_SAMPLING_PROCESSOR => {
                String::from("spans-errors-sampling-processor")
            }
            Self::SPANS_APPSEC_SAMPLING_PROCESSOR => {
                String::from("spans-appsec-sampling-processor")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RetentionFilterAllType {
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

impl<'de> Deserialize<'de> for RetentionFilterAllType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "spans-sampling-processor" => Self::SPANS_SAMPLING_PROCESSOR,
            "spans-errors-sampling-processor" => Self::SPANS_ERRORS_SAMPLING_PROCESSOR,
            "spans-appsec-sampling-processor" => Self::SPANS_APPSEC_SAMPLING_PROCESSOR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
