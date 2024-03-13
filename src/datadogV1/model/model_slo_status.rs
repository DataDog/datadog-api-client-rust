// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Status of the SLO's primary timeframe.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn calculation_error(mut self, value: Option<String>) -> Self {
        self.calculation_error = Some(value);
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

    pub fn sli(mut self, value: Option<f64>) -> Self {
        self.sli = Some(value);
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
}

impl Default for SLOStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOStatusVisitor;
        impl<'a> Visitor<'a> for SLOStatusVisitor {
            type Value = SLOStatus;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculation_error: Option<Option<String>> = None;
                let mut error_budget_remaining: Option<Option<f64>> = None;
                let mut indexed_at: Option<i64> = None;
                let mut raw_error_budget_remaining: Option<
                    Option<crate::datadogV1::model::SLORawErrorBudgetRemaining>,
                > = None;
                let mut sli: Option<Option<f64>> = None;
                let mut span_precision: Option<Option<i64>> = None;
                let mut state: Option<crate::datadogV1::model::SLOState> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calculation_error" => {
                            calculation_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "sli" => {
                            sli = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {}
                    }
                }

                let content = SLOStatus {
                    calculation_error,
                    error_budget_remaining,
                    indexed_at,
                    raw_error_budget_remaining,
                    sli,
                    span_precision,
                    state,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOStatusVisitor)
    }
}
