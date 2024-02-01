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
        Option<Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>>,
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

    pub fn error(&mut self, value: Option<String>) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn error_budget_remaining(&mut self, value: Option<f64>) -> &mut Self {
        self.error_budget_remaining = Some(value);
        self
    }

    pub fn indexed_at(&mut self, value: i64) -> &mut Self {
        self.indexed_at = Some(value);
        self
    }

    pub fn raw_error_budget_remaining(
        &mut self,
        value: Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>,
    ) -> &mut Self {
        self.raw_error_budget_remaining = Some(value);
        self
    }

    pub fn span_precision(&mut self, value: Option<i64>) -> &mut Self {
        self.span_precision = Some(value);
        self
    }

    pub fn state(&mut self, value: crate::datadogV1::model::SLOState) -> &mut Self {
        self.state = Some(value);
        self
    }

    pub fn status(&mut self, value: Option<f64>) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn target(&mut self, value: f64) -> &mut Self {
        self.target = Some(value);
        self
    }

    pub fn timeframe(&mut self, value: crate::datadogV1::model::SLOTimeframe) -> &mut Self {
        self.timeframe = Some(value);
        self
    }
}

impl Default for SLOOverallStatuses {
    fn default() -> Self {
        Self::new()
    }
}
