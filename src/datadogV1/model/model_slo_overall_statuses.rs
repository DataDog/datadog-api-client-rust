// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Overall status of the SLO by timeframes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    /// The SLO time window options. Note that "custom" is not a valid option for creating
    /// or updating SLOs. It is only used when querying SLO history over custom timeframes.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<crate::datadogV1::model::SLOTimeframe>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: Option<String>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn error_budget_remaining(mut self, value: Option<f64>) -> Self {
        self.error_budget_remaining = Some(value);
        self
    }

    pub fn indexed_at(mut self, value: i64) -> Self {
        self.indexed_at = Some(value);
        self
    }

    pub fn raw_error_budget_remaining(
        mut self,
        value: Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>,
    ) -> Self {
        self.raw_error_budget_remaining = Some(value);
        self
    }

    pub fn span_precision(mut self, value: Option<i64>) -> Self {
        self.span_precision = Some(value);
        self
    }

    pub fn state(mut self, value: crate::datadogV1::model::SLOState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn status(mut self, value: Option<f64>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target(mut self, value: f64) -> Self {
        self.target = Some(value);
        self
    }

    pub fn timeframe(mut self, value: crate::datadogV1::model::SLOTimeframe) -> Self {
        self.timeframe = Some(value);
        self
    }
}

impl Default for SLOOverallStatuses {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOOverallStatuses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOOverallStatusesVisitor;
        impl<'a> Visitor<'a> for SLOOverallStatusesVisitor {
            type Value = SLOOverallStatuses;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<Option<String>> = None;
                let mut error_budget_remaining: Option<Option<f64>> = None;
                let mut indexed_at: Option<i64> = None;
                let mut raw_error_budget_remaining: Option<
                    Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>,
                > = None;
                let mut span_precision: Option<Option<i64>> = None;
                let mut state: Option<crate::datadogV1::model::SLOState> = None;
                let mut status: Option<Option<f64>> = None;
                let mut target: Option<f64> = None;
                let mut timeframe: Option<crate::datadogV1::model::SLOTimeframe> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_budget_remaining" => {
                            error_budget_remaining =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            indexed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "raw_error_budget_remaining" => {
                            raw_error_budget_remaining =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_precision" => {
                            span_precision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV1::model::SLOState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timeframe) = timeframe {
                                match _timeframe {
                                    crate::datadogV1::model::SLOTimeframe::UnparsedObject(
                                        _timeframe,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SLOOverallStatuses {
                    error,
                    error_budget_remaining,
                    indexed_at,
                    raw_error_budget_remaining,
                    span_precision,
                    state,
                    status,
                    target,
                    timeframe,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOOverallStatusesVisitor)
    }
}
