// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionSLOMeasure {
    GOOD_EVENTS,
    BAD_EVENTS,
    SLO_STATUS,
    ERROR_BUDGET_REMAINING,
    BURN_RATE,
    ERROR_BUDGET_BURNDOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FormulaAndFunctionSLOMeasure {
    fn to_string(&self) -> String {
        match self {
            Self::GOOD_EVENTS => String::from("good_events"),
            Self::BAD_EVENTS => String::from("bad_events"),
            Self::SLO_STATUS => String::from("slo_status"),
            Self::ERROR_BUDGET_REMAINING => String::from("error_budget_remaining"),
            Self::BURN_RATE => String::from("burn_rate"),
            Self::ERROR_BUDGET_BURNDOWN => String::from("error_budget_burndown"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormulaAndFunctionSLOMeasure {
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

impl<'de> Deserialize<'de> for FormulaAndFunctionSLOMeasure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "good_events" => Self::GOOD_EVENTS,
            "bad_events" => Self::BAD_EVENTS,
            "slo_status" => Self::SLO_STATUS,
            "error_budget_remaining" => Self::ERROR_BUDGET_REMAINING,
            "burn_rate" => Self::BURN_RATE,
            "error_budget_burndown" => Self::ERROR_BUDGET_BURNDOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
