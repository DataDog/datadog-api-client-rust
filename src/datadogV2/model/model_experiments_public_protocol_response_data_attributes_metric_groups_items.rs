// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A group of metrics supplied by the protocol.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
    /// Whether the group contains decision metrics.
    #[serde(rename = "is_decision")]
    pub is_decision: Option<bool>,
    /// Metrics included in this group.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>>,
    /// Display name of the metric group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
    pub fn new() -> ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
        ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
            is_decision: None,
            metrics: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_decision(mut self, value: bool) -> Self {
        self.is_decision = Some(value);
        self
    }

    pub fn metrics(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>,
    ) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItemsVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_decision: Option<bool> = None;
                let mut metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_decision" => {
                            if v.is_null() {
                                continue;
                            }
                            is_decision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems {
                    is_decision,
                    metrics,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItemsVisitor,
        )
    }
}
