// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a holdout that the experiment is attached to.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentHoldoutV2DTODataAttributes {
    /// Zero-based position of this MANAGED holdout in the experiment's evaluation order. The GLOBAL holdout is evaluated first. Null for EXTERNAL holdouts.
    #[serde(
        rename = "evaluation_order",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub evaluation_order: Option<Option<i64>>,
    /// How Datadog serves a MANAGED holdout. GLOBAL holdouts apply to every eligible experiment of a subject type. SELECTIVE holdouts apply only to the experiments attached to them. Null for EXTERNAL holdouts.
    #[serde(
        rename = "holdout_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub holdout_type: Option<
        Option<crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesHoldoutType>,
    >,
    /// Key of the holdout.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Whether Datadog serves the holdout. Datadog serves MANAGED holdouts. EXTERNAL holdouts record what your own feature flagging system holds back, and Datadog does not serve them.
    #[serde(rename = "kind")]
    pub kind: Option<crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesKind>,
    /// Display name of the holdout.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Lifecycle phase of the holdout. PENDING holdouts have not started including experiments. INCLUDING holdouts are adding experiments. WAITING_FOR_ANALYSIS holdouts have stopped including experiments but have not started analysis. ANALYSING holdouts are in their analysis window. COMPLETE applies only to EXTERNAL holdouts that have ended their analysis window but are not archived.
    #[serde(rename = "state")]
    pub state:
        Option<crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesState>,
    /// Share of subjects the holdout holds back, from 0 to 1.
    #[serde(rename = "traffic_allocation")]
    pub traffic_allocation: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExperimentHoldoutV2DTODataAttributes {
    pub fn new() -> ExperimentsExperimentHoldoutV2DTODataAttributes {
        ExperimentsExperimentHoldoutV2DTODataAttributes {
            evaluation_order: None,
            holdout_type: None,
            key: None,
            kind: None,
            name: None,
            state: None,
            traffic_allocation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn evaluation_order(mut self, value: Option<i64>) -> Self {
        self.evaluation_order = Some(value);
        self
    }

    pub fn holdout_type(
        mut self,
        value: Option<
            crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesHoldoutType,
        >,
    ) -> Self {
        self.holdout_type = Some(value);
        self
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn state(
        mut self,
        value: crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesState,
    ) -> Self {
        self.state = Some(value);
        self
    }

    pub fn traffic_allocation(mut self, value: f64) -> Self {
        self.traffic_allocation = Some(value);
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

impl Default for ExperimentsExperimentHoldoutV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsExperimentHoldoutV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentHoldoutV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentHoldoutV2DTODataAttributesVisitor {
            type Value = ExperimentsExperimentHoldoutV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut evaluation_order: Option<Option<i64>> = None;
                let mut holdout_type: Option<Option<crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesHoldoutType>> = None;
                let mut key: Option<String> = None;
                let mut kind: Option<
                    crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesKind,
                > = None;
                let mut name: Option<String> = None;
                let mut state: Option<
                    crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesState,
                > = None;
                let mut traffic_allocation: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "evaluation_order" => {
                            evaluation_order =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "holdout_type" => {
                            holdout_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _holdout_type) = holdout_type {
                                match _holdout_type {
                                    Some(crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesHoldoutType::UnparsedObject(_holdout_type)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            if v.is_null() {
                                continue;
                            }
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::ExperimentsExperimentHoldoutV2DTODataAttributesState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "traffic_allocation" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            traffic_allocation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsExperimentHoldoutV2DTODataAttributes {
                    evaluation_order,
                    holdout_type,
                    key,
                    kind,
                    name,
                    state,
                    traffic_allocation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExperimentHoldoutV2DTODataAttributesVisitor)
    }
}
