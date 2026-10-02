// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the metric collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricCollectionV2DTODataAttributes {
    /// Time when this resource was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Text that explains the metric collection.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Whether the collection is used for guardrail metrics.
    #[serde(rename = "is_guardrail")]
    pub is_guardrail: Option<bool>,
    /// Number of metrics in this collection.
    #[serde(rename = "metric_count")]
    pub metric_count: Option<i64>,
    /// Metrics included in this collection.
    #[serde(rename = "metrics")]
    pub metrics: Option<
        Vec<crate::datadogV2::model::ExperimentsMetricCollectionV2DTODataAttributesMetricsItems>,
    >,
    /// Display name of the metric collection.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Time when this resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricCollectionV2DTODataAttributes {
    pub fn new() -> ExperimentsMetricCollectionV2DTODataAttributes {
        ExperimentsMetricCollectionV2DTODataAttributes {
            created_at: None,
            description: None,
            is_guardrail: None,
            metric_count: None,
            metrics: None,
            name: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn is_guardrail(mut self, value: bool) -> Self {
        self.is_guardrail = Some(value);
        self
    }

    pub fn metric_count(mut self, value: i64) -> Self {
        self.metric_count = Some(value);
        self
    }

    pub fn metrics(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsMetricCollectionV2DTODataAttributesMetricsItems,
        >,
    ) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl Default for ExperimentsMetricCollectionV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricCollectionV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricCollectionV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricCollectionV2DTODataAttributesVisitor {
            type Value = ExperimentsMetricCollectionV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut is_guardrail: Option<bool> = None;
                let mut metric_count: Option<i64> = None;
                let mut metrics: Option<Vec<crate::datadogV2::model::ExperimentsMetricCollectionV2DTODataAttributesMetricsItems>> = None;
                let mut name: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "metric_count" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_count =
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
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricCollectionV2DTODataAttributes {
                    created_at,
                    description,
                    is_guardrail,
                    metric_count,
                    metrics,
                    name,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsMetricCollectionV2DTODataAttributesVisitor)
    }
}
