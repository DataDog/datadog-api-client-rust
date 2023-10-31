// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Status of the SLO's primary timeframe.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOStatus {
    /// Error message if SLO status or error budget could not be calculated.
    #[serde(rename = "calculation_error", default, with = "::serde_with::rust::double_option")]
    pub calculation_error: Option<Option<String>>,
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
    pub raw_error_budget_remaining: Option<Option<Box<crate::datadogV1::model::SLORawErrorBudgetRemaining>>>,
    /// The current service level indicator (SLI) of the SLO, also known as 'status'. This is a percentage value from 0-100 (inclusive).
    #[serde(rename = "sli", default, with = "::serde_with::rust::double_option")]
    pub sli: Option<Option<f64>>,
    /// The number of decimal places the SLI value is accurate to.
    #[serde(rename = "span_precision", default, with = "::serde_with::rust::double_option")]
    pub span_precision: Option<Option<i64>>,
    /// State of the SLO.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV1::model::SLOState>,
}

impl SLOStatus {
    pub fn new() -> SLOStatus {
        SLOStatus {
            calculation_error: None,
            error_budget_remaining: None,
            indexed_at: None,
            raw_error_budget_remaining: None,
            sli: None,
            span_precision: None,
            state: None,
        }
    }
}
