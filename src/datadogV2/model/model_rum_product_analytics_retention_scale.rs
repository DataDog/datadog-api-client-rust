// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Product Analytics retention scale configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMProductAnalyticsRetentionScale {
    /// Timestamp in milliseconds when this scale was last modified.
    #[serde(rename = "last_modified_at")]
    pub last_modified_at: Option<i64>,
    /// Controls the retention policy for Product Analytics data derived from RUM events.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::RUMProductAnalyticsRetentionState>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMProductAnalyticsRetentionScale {
    pub fn new() -> RUMProductAnalyticsRetentionScale {
        RUMProductAnalyticsRetentionScale {
            last_modified_at: None,
            state: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_modified_at(mut self, value: i64) -> Self {
        self.last_modified_at = Some(value);
        self
    }

    pub fn state(
        mut self,
        value: crate::datadogV2::model::RUMProductAnalyticsRetentionState,
    ) -> Self {
        self.state = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for RUMProductAnalyticsRetentionScale {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMProductAnalyticsRetentionScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMProductAnalyticsRetentionScaleVisitor;
        impl<'a> Visitor<'a> for RUMProductAnalyticsRetentionScaleVisitor {
            type Value = RUMProductAnalyticsRetentionScale;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_modified_at: Option<i64> = None;
                let mut state: Option<crate::datadogV2::model::RUMProductAnalyticsRetentionState> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::RUMProductAnalyticsRetentionState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMProductAnalyticsRetentionScale {
                    last_modified_at,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMProductAnalyticsRetentionScaleVisitor)
    }
}
