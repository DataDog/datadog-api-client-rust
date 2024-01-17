// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Overall status of the SLO by timeframes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOOverallStatuses {
    /// Error message if SLO status or error budget could not be calculated.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<String>>,
    /// Remaining error budget of the SLO in percentage.
    #[serde(
        rename = "error_budget_remaining",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_budget_remaining: Option<Option<f64>>,
    /// timestamp (UNIX time in seconds) of when the SLO status and error budget
    /// were calculated.
    #[serde(rename = "indexed_at")]
    pub indexed_at: Option<i64>,
    /// Error budget remaining for an SLO.
    #[serde(
        rename = "raw_error_budget_remaining",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub raw_error_budget_remaining:
        Option<Option<Box<crate::datadogV1::model::SLORawErrorBudgetRemaining>>>,
    /// The amount of decimal places the SLI value is accurate to.
    #[serde(
        rename = "span_precision",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub span_precision: Option<Option<i64>>,
    /// State of the SLO.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV1::model::SLOState>,
    /// The status of the SLO.
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option")]
    pub status: Option<Option<f64>>,
    /// The target of the SLO.
    #[serde(rename = "target")]
    pub target: Option<f64>,
    /// The SLO time window options.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<crate::datadogV1::model::SLOTimeframe>,
}

impl SLOOverallStatuses {
    pub fn new() -> SLOOverallStatuses {
        SLOOverallStatuses {
            error: None,
            error_budget_remaining: None,
            indexed_at: None,
            raw_error_budget_remaining: None,
            span_precision: None,
            state: None,
            status: None,
            target: None,
            timeframe: None,
        }
    }
}
impl Default for SLOOverallStatuses {
    fn default() -> Self {
        Self::new()
    }
}
