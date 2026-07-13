// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metric name filter preview for a single namespace.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricNameFilterPreviewNamespace {
    /// The metric name filter patterns evaluated for this namespace and how many metrics they matched.
    #[serde(rename = "filters")]
    pub filters: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewFilterMatch>,
    /// The CloudWatch metrics collected for this namespace and whether each resulting
    /// Datadog metric is filtered.
    #[serde(rename = "metrics")]
    pub metrics: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewMetric>,
    /// The AWS CloudWatch namespace.
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricNameFilterPreviewNamespace {
    pub fn new(
        filters: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewFilterMatch>,
        metrics: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewMetric>,
        namespace: String,
    ) -> AWSMetricNameFilterPreviewNamespace {
        AWSMetricNameFilterPreviewNamespace {
            filters,
            metrics,
            namespace,
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

impl<'de> Deserialize<'de> for AWSMetricNameFilterPreviewNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricNameFilterPreviewNamespaceVisitor;
        impl<'a> Visitor<'a> for AWSMetricNameFilterPreviewNamespaceVisitor {
            type Value = AWSMetricNameFilterPreviewNamespace;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filters: Option<
                    Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewFilterMatch>,
                > = None;
                let mut metrics: Option<
                    Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewMetric>,
                > = None;
                let mut namespace: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;
                let metrics = metrics.ok_or_else(|| M::Error::missing_field("metrics"))?;
                let namespace = namespace.ok_or_else(|| M::Error::missing_field("namespace"))?;

                let content = AWSMetricNameFilterPreviewNamespace {
                    filters,
                    metrics,
                    namespace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricNameFilterPreviewNamespaceVisitor)
    }
}
