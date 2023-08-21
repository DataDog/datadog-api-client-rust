// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOOverallStatuses {
    /// Error message if SLO status or error budget could not be calculated.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub error: Option<String>,
    /// Remaining error budget of the SLO in percentage.
    #[serde(rename = "error_budget_remaining", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub error_budget_remaining: Option<Float64>,
    /// timestamp (UNIX time in seconds) of when the SLO status and error budget
were calculated.
    #[serde(rename = "indexed_at", skip_serializing_if = "Option::is_none")]
    pub indexed_at: i64,
    /// Error budget remaining for an SLO.
    #[serde(rename = "raw_error_budget_remaining", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub raw_error_budget_remaining: NullableSLORawErrorBudgetRemaining,
    /// The amount of decimal places the SLI value is accurate to.
    #[serde(rename = "span_precision", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub span_precision: Option<Int64>,
    /// State of the SLO.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: SLOState,
    /// The status of the SLO.
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub status: Option<Float64>,
    /// The target of the SLO.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: f64,
    /// The SLO time window options.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: SLOTimeframe,
}

