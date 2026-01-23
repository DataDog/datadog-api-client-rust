// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the SLO status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SloStatusDataAttributes {
    /// The percentage of error budget remaining.
    #[serde(rename = "error_budget_remaining")]
    pub error_budget_remaining: f64,
    /// The raw error budget remaining for the SLO.
    #[serde(rename = "raw_error_budget_remaining")]
    pub raw_error_budget_remaining: crate::datadogV2::model::RawErrorBudgetRemaining,
    /// The current Service Level Indicator (SLI) value as a percentage.
    #[serde(rename = "sli")]
    pub sli: f64,
    /// The precision of the time span in seconds.
    #[serde(rename = "span_precision")]
    pub span_precision: i64,
    /// The current state of the SLO (for example, `breached`, `warning`, `ok`).
    #[serde(rename = "state")]
    pub state: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SloStatusDataAttributes {
    pub fn new(
        error_budget_remaining: f64,
        raw_error_budget_remaining: crate::datadogV2::model::RawErrorBudgetRemaining,
        sli: f64,
        span_precision: i64,
        state: String,
    ) -> SloStatusDataAttributes {
        SloStatusDataAttributes {
            error_budget_remaining,
            raw_error_budget_remaining,
            sli,
            span_precision,
            state,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SloStatusDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SloStatusDataAttributesVisitor;
        impl<'a> Visitor<'a> for SloStatusDataAttributesVisitor {
            type Value = SloStatusDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_budget_remaining: Option<f64> = None;
                let mut raw_error_budget_remaining: Option<
                    crate::datadogV2::model::RawErrorBudgetRemaining,
                > = None;
                let mut sli: Option<f64> = None;
                let mut span_precision: Option<i64> = None;
                let mut state: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_budget_remaining" => {
                            error_budget_remaining =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error_budget_remaining = error_budget_remaining
                    .ok_or_else(|| M::Error::missing_field("error_budget_remaining"))?;
                let raw_error_budget_remaining = raw_error_budget_remaining
                    .ok_or_else(|| M::Error::missing_field("raw_error_budget_remaining"))?;
                let sli = sli.ok_or_else(|| M::Error::missing_field("sli"))?;
                let span_precision =
                    span_precision.ok_or_else(|| M::Error::missing_field("span_precision"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = SloStatusDataAttributes {
                    error_budget_remaining,
                    raw_error_budget_remaining,
                    sli,
                    span_precision,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SloStatusDataAttributesVisitor)
    }
}
