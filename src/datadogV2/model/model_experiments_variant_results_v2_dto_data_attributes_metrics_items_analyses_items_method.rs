// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod {
    FIXED_SAMPLE,
    BAYESIAN,
    SEQUENTIAL,
    SEQUENTIAL_FIXED_HYBRID,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod {
    fn to_string(&self) -> String {
        match self {
            Self::FIXED_SAMPLE => String::from("FIXED_SAMPLE"),
            Self::BAYESIAN => String::from("BAYESIAN"),
            Self::SEQUENTIAL => String::from("SEQUENTIAL"),
            Self::SEQUENTIAL_FIXED_HYBRID => String::from("SEQUENTIAL_FIXED_HYBRID"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod {
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

impl<'de> Deserialize<'de>
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "FIXED_SAMPLE" => Self::FIXED_SAMPLE,
            "BAYESIAN" => Self::BAYESIAN,
            "SEQUENTIAL" => Self::SEQUENTIAL,
            "SEQUENTIAL_FIXED_HYBRID" => Self::SEQUENTIAL_FIXED_HYBRID,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
