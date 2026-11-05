// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayModelPricingEffectiveSource {
    CATALOG_AUTO,
    MODEL_OVERRIDE,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayModelPricingEffectiveSource {
    fn to_string(&self) -> String {
        match self {
            Self::CATALOG_AUTO => String::from("catalog_auto"),
            Self::MODEL_OVERRIDE => String::from("model_override"),
            Self::UNKNOWN => String::from("unknown"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayModelPricingEffectiveSource {
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

impl<'de> Deserialize<'de> for AiGatewayModelPricingEffectiveSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "catalog_auto" => Self::CATALOG_AUTO,
            "model_override" => Self::MODEL_OVERRIDE,
            "unknown" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
