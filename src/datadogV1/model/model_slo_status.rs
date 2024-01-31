// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Status of the SLO's primary timeframe.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOStatus {
    /// Error message if SLO status or error budget could not be calculated.
    #[serde(
        rename = "calculation_error",
        default,
        with = "::serde_with::rust::double_option"
    )]
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
    pub raw_error_budget_remaining:
        Option<Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>>,
    /// The current service level indicator (SLI) of the SLO, also known as 'status'. This is a percentage value from 0-100 (inclusive).
    #[serde(rename = "sli", default, with = "::serde_with::rust::double_option")]
    pub sli: Option<Option<f64>>,
    /// The number of decimal places the SLI value is accurate to.
    #[serde(
        rename = "span_precision",
        default,
        with = "::serde_with::rust::double_option"
    )]
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

    pub fn with_calculation_error(&mut self, value: Option<String>) -> &mut Self {
        self.calculation_error = Some(value);
        self
    }

    pub fn with_error_budget_remaining(&mut self, value: Option<f64>) -> &mut Self {
        self.error_budget_remaining = Some(value);
        self
    }

    pub fn with_indexed_at(&mut self, value: i64) -> &mut Self {
        self.indexed_at = Some(value);
        self
    }

    pub fn with_raw_error_budget_remaining(
        &mut self,
        value: Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>,
    ) -> &mut Self {
        self.raw_error_budget_remaining = Some(value);
        self
    }

    pub fn with_sli(&mut self, value: Option<f64>) -> &mut Self {
        self.sli = Some(value);
        self
    }

    pub fn with_span_precision(&mut self, value: Option<i64>) -> &mut Self {
        self.span_precision = Some(value);
        self
    }

    pub fn with_state(&mut self, value: crate::datadogV1::model::SLOState) -> &mut Self {
        self.state = Some(value);
        self
    }
}
impl Default for SLOStatus {
    fn default() -> Self {
        Self::new()
    }
}
