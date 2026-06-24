// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineClickhouseDestinationFormat {
    JSON_EACH_ROW,
    JSON_AS_OBJECT,
    JSON_AS_STRING,
    ARROW_STREAM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineClickhouseDestinationFormat {
    fn to_string(&self) -> String {
        match self {
            Self::JSON_EACH_ROW => String::from("json_each_row"),
            Self::JSON_AS_OBJECT => String::from("json_as_object"),
            Self::JSON_AS_STRING => String::from("json_as_string"),
            Self::ARROW_STREAM => String::from("arrow_stream"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineClickhouseDestinationFormat {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestinationFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "json_each_row" => Self::JSON_EACH_ROW,
            "json_as_object" => Self::JSON_AS_OBJECT,
            "json_as_string" => Self::JSON_AS_STRING,
            "arrow_stream" => Self::ARROW_STREAM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
