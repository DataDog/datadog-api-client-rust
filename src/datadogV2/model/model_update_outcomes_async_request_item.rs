// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scorecard outcome for a single entity and rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateOutcomesAsyncRequestItem {
    /// The unique reference for an IDP entity.
    #[serde(rename = "entity_reference")]
    pub entity_reference: String,
    /// Any remarks regarding the scorecard rule's evaluation. Supports HTML hyperlinks.
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    /// The unique ID for a scorecard rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// The state of the rule evaluation.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::State,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateOutcomesAsyncRequestItem {
    pub fn new(
        entity_reference: String,
        rule_id: String,
        state: crate::datadogV2::model::State,
    ) -> UpdateOutcomesAsyncRequestItem {
        UpdateOutcomesAsyncRequestItem {
            entity_reference,
            remarks: None,
            rule_id,
            state,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn remarks(mut self, value: String) -> Self {
        self.remarks = Some(value);
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

impl<'de> Deserialize<'de> for UpdateOutcomesAsyncRequestItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateOutcomesAsyncRequestItemVisitor;
        impl<'a> Visitor<'a> for UpdateOutcomesAsyncRequestItemVisitor {
            type Value = UpdateOutcomesAsyncRequestItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entity_reference: Option<String> = None;
                let mut remarks: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::State> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entity_reference" => {
                            entity_reference =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remarks" => {
                            if v.is_null() {
                                continue;
                            }
                            remarks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let entity_reference =
                    entity_reference.ok_or_else(|| M::Error::missing_field("entity_reference"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = UpdateOutcomesAsyncRequestItem {
                    entity_reference,
                    remarks,
                    rule_id,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateOutcomesAsyncRequestItemVisitor)
    }
}
