// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS metric name filter preview request attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricNameFilterPreviewRequestAttributes {
    /// The metric name filters to preview.
    #[serde(rename = "metric_name_filters")]
    pub metric_name_filters: Vec<crate::datadogV2::model::AWSMetricNameFilters>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricNameFilterPreviewRequestAttributes {
    pub fn new(
        metric_name_filters: Vec<crate::datadogV2::model::AWSMetricNameFilters>,
    ) -> AWSMetricNameFilterPreviewRequestAttributes {
        AWSMetricNameFilterPreviewRequestAttributes {
            metric_name_filters,
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

impl<'de> Deserialize<'de> for AWSMetricNameFilterPreviewRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricNameFilterPreviewRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AWSMetricNameFilterPreviewRequestAttributesVisitor {
            type Value = AWSMetricNameFilterPreviewRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metric_name_filters: Option<
                    Vec<crate::datadogV2::model::AWSMetricNameFilters>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metric_name_filters" => {
                            metric_name_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metric_name_filters = metric_name_filters
                    .ok_or_else(|| M::Error::missing_field("metric_name_filters"))?;

                let content = AWSMetricNameFilterPreviewRequestAttributes {
                    metric_name_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricNameFilterPreviewRequestAttributesVisitor)
    }
}
