// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayModelMetadataSourceMatchedBy {
    OVERRIDE,
    WIRE_ID,
    CANONICAL_ID,
    WIRE_ID_DATE_SUFFIX,
    CANONICAL_MODEL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayModelMetadataSourceMatchedBy {
    fn to_string(&self) -> String {
        match self {
            Self::OVERRIDE => String::from("override"),
            Self::WIRE_ID => String::from("wire_id"),
            Self::CANONICAL_ID => String::from("canonical_id"),
            Self::WIRE_ID_DATE_SUFFIX => String::from("wire_id_date_suffix"),
            Self::CANONICAL_MODEL => String::from("canonical_model"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayModelMetadataSourceMatchedBy {
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

impl<'de> Deserialize<'de> for AiGatewayModelMetadataSourceMatchedBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "override" => Self::OVERRIDE,
            "wire_id" => Self::WIRE_ID,
            "canonical_id" => Self::CANONICAL_ID,
            "wire_id_date_suffix" => Self::WIRE_ID_DATE_SUFFIX,
            "canonical_model" => Self::CANONICAL_MODEL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
