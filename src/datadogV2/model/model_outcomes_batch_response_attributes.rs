// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API attributes for an outcome.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesBatchResponseAttributes {
    /// Creation time of the rule outcome.
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    /// Time of last rule outcome modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<DateTime<Utc>>,
    /// Any remarks regarding the scorecard rule's evaluation, and supports HTML hyperlinks.
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    /// The unique name for a service in the catalog.
    #[serde(rename = "service_name")]
    pub service_name: Option<String>,
    /// The state of the rule evaluation.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::State>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesBatchResponseAttributes {
    pub fn new() -> OutcomesBatchResponseAttributes {
        OutcomesBatchResponseAttributes {
            created_at: None,
            modified_at: None,
            remarks: None,
            service_name: None,
            state: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn remarks(mut self, value: String) -> Self {
        self.remarks = Some(value);
        self
    }

    pub fn service_name(mut self, value: String) -> Self {
        self.service_name = Some(value);
        self
    }

    pub fn state(mut self, value: crate::datadogV2::model::State) -> Self {
        self.state = Some(value);
        self
    }
}

impl Default for OutcomesBatchResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesBatchResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesBatchResponseAttributesVisitor;
        impl<'a> Visitor<'a> for OutcomesBatchResponseAttributesVisitor {
            type Value = OutcomesBatchResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<DateTime<Utc>> = None;
                let mut modified_at: Option<DateTime<Utc>> = None;
                let mut remarks: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::State> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remarks" => {
                            if v.is_null() {
                                continue;
                            }
                            remarks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            if v.is_null() {
                                continue;
                            }
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = OutcomesBatchResponseAttributes {
                    created_at,
                    modified_at,
                    remarks,
                    service_name,
                    state,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesBatchResponseAttributesVisitor)
    }
}
