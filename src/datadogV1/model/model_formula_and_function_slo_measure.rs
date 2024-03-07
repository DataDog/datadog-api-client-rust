// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormulaAndFunctionSLOMeasure {
    #[serde(rename = "good_events")]
    GOOD_EVENTS,
    #[serde(rename = "bad_events")]
    BAD_EVENTS,
    #[serde(rename = "slo_status")]
    SLO_STATUS,
    #[serde(rename = "error_budget_remaining")]
    ERROR_BUDGET_REMAINING,
    #[serde(rename = "burn_rate")]
    BURN_RATE,
    #[serde(rename = "error_budget_burndown")]
    ERROR_BUDGET_BURNDOWN,
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
        }
    }
}
