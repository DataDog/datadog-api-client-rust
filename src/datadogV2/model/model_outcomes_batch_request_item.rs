// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scorecard outcome for a specific rule, for a given service within a batched update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesBatchRequestItem {
    /// Any remarks regarding the scorecard rule's evaluation, and supports HTML hyperlinks.
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    /// The unique ID for a scorecard rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// The unique name for a service in the catalog.
    #[serde(rename = "service_name")]
    pub service_name: String,
    /// The state of the rule evaluation.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::State,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesBatchRequestItem {
    pub fn new(
        rule_id: String,
        service_name: String,
        state: crate::datadogV2::model::State,
    ) -> OutcomesBatchRequestItem {
        OutcomesBatchRequestItem {
            remarks: None,
            rule_id,
            service_name,
            state,
            _unparsed: false,
        }
    }

    pub fn remarks(mut self, value: String) -> Self {
        self.remarks = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for OutcomesBatchRequestItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesBatchRequestItemVisitor;
        impl<'a> Visitor<'a> for OutcomesBatchRequestItemVisitor {
            type Value = OutcomesBatchRequestItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut remarks: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::State> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "remarks" => {
                            if v.is_null() {
                                continue;
                            }
                            remarks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::State::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = OutcomesBatchRequestItem {
                    remarks,
                    rule_id,
                    service_name,
                    state,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesBatchRequestItemVisitor)
    }
}
