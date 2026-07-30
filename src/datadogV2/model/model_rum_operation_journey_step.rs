// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single step of a RUM operation's journey. Matches RUM events either through a list of `nodes`
/// or through a `composite` rule; the two are mutually exclusive.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMOperationJourneyStep {
    /// A composite rule combining several predicates. Used as an alternative to `nodes` on a journey
    /// step when several conditions must be matched together, in any order or in a specific order.
    #[serde(rename = "composite")]
    pub composite: Option<crate::datadogV2::model::RUMOperationJourneyCompositeRule>,
    /// The list of nodes that can match this step. Mutually exclusive with `composite`.
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<crate::datadogV2::model::RUMOperationJourneyNode>>,
    /// The type of a step within a RUM operation's journey.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RUMOperationJourneyStepType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMOperationJourneyStep {
    pub fn new(
        type_: crate::datadogV2::model::RUMOperationJourneyStepType,
    ) -> RUMOperationJourneyStep {
        RUMOperationJourneyStep {
            composite: None,
            nodes: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn composite(
        mut self,
        value: crate::datadogV2::model::RUMOperationJourneyCompositeRule,
    ) -> Self {
        self.composite = Some(value);
        self
    }

    pub fn nodes(mut self, value: Vec<crate::datadogV2::model::RUMOperationJourneyNode>) -> Self {
        self.nodes = Some(value);
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

impl<'de> Deserialize<'de> for RUMOperationJourneyStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMOperationJourneyStepVisitor;
        impl<'a> Visitor<'a> for RUMOperationJourneyStepVisitor {
            type Value = RUMOperationJourneyStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut composite: Option<
                    crate::datadogV2::model::RUMOperationJourneyCompositeRule,
                > = None;
                let mut nodes: Option<Vec<crate::datadogV2::model::RUMOperationJourneyNode>> = None;
                let mut type_: Option<crate::datadogV2::model::RUMOperationJourneyStepType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "composite" => {
                            if v.is_null() {
                                continue;
                            }
                            composite = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nodes" => {
                            if v.is_null() {
                                continue;
                            }
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RUMOperationJourneyStepType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RUMOperationJourneyStep {
                    composite,
                    nodes,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMOperationJourneyStepVisitor)
    }
}
