// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason {
    CONTROL_DENOMINATOR_NEAR_ZERO,
    TREATMENT_DENOMINATOR_NEAR_ZERO,
    CONTROL_AND_TREATMENT_DENOMINATORS_NEAR_ZERO,
    CONTROL_MEAN_NEAR_ZERO,
    ZERO_VARIANCE,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason
{
    fn to_string(&self) -> String {
        match self {
            Self::CONTROL_DENOMINATOR_NEAR_ZERO => String::from("CONTROL_DENOMINATOR_NEAR_ZERO"),
            Self::TREATMENT_DENOMINATOR_NEAR_ZERO => {
                String::from("TREATMENT_DENOMINATOR_NEAR_ZERO")
            }
            Self::CONTROL_AND_TREATMENT_DENOMINATORS_NEAR_ZERO => {
                String::from("CONTROL_AND_TREATMENT_DENOMINATORS_NEAR_ZERO")
            }
            Self::CONTROL_MEAN_NEAR_ZERO => String::from("CONTROL_MEAN_NEAR_ZERO"),
            Self::ZERO_VARIANCE => String::from("ZERO_VARIANCE"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason
{
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
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CONTROL_DENOMINATOR_NEAR_ZERO" => Self::CONTROL_DENOMINATOR_NEAR_ZERO,
            "TREATMENT_DENOMINATOR_NEAR_ZERO" => Self::TREATMENT_DENOMINATOR_NEAR_ZERO,
            "CONTROL_AND_TREATMENT_DENOMINATORS_NEAR_ZERO" => {
                Self::CONTROL_AND_TREATMENT_DENOMINATORS_NEAR_ZERO
            }
            "CONTROL_MEAN_NEAR_ZERO" => Self::CONTROL_MEAN_NEAR_ZERO,
            "ZERO_VARIANCE" => Self::ZERO_VARIANCE,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
