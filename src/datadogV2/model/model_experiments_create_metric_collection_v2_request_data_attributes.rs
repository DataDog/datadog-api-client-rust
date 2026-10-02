// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Name, description, and metric selection for the new collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateMetricCollectionV2RequestDataAttributes {
    /// Description of the metric collection.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Whether the collection is designated as a guardrail collection.
    #[serde(rename = "is_guardrail")]
    pub is_guardrail: Option<bool>,
    /// Metrics to include in the collection.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>>,
    /// Display name of the metric collection.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsCreateMetricCollectionV2RequestDataAttributes {
    pub fn new(name: String) -> ExperimentsCreateMetricCollectionV2RequestDataAttributes {
        ExperimentsCreateMetricCollectionV2RequestDataAttributes {
            description: None,
            is_guardrail: None,
            metrics: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn is_guardrail(mut self, value: bool) -> Self {
        self.is_guardrail = Some(value);
        self
    }

    pub fn metrics(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>,
    ) -> Self {
        self.metrics = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsCreateMetricCollectionV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateMetricCollectionV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsCreateMetricCollectionV2RequestDataAttributesVisitor {
            type Value = ExperimentsCreateMetricCollectionV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut is_guardrail: Option<bool> = None;
                let mut metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems>> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_guardrail" => {
                            if v.is_null() {
                                continue;
                            }
                            is_guardrail =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ExperimentsCreateMetricCollectionV2RequestDataAttributes {
                    description,
                    is_guardrail,
                    metrics,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsCreateMetricCollectionV2RequestDataAttributesVisitor)
    }
}
