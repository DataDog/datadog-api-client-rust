// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metric in an experiment metric group, with its name and primary metric designation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems {
    /// Whether this is the experiment primary metric.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    /// Identifier of the metric in the group.
    #[serde(rename = "metric_id")]
    pub metric_id: String,
    /// Display name of the metric in the group.
    #[serde(rename = "metric_name")]
    pub metric_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems {
    pub fn new(
        is_primary: bool,
        metric_id: String,
        metric_name: String,
    ) -> ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems {
        ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems {
            is_primary,
            metric_id,
            metric_name,
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

impl<'de> Deserialize<'de>
    for ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItemsVisitor
        {
            type Value = ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_primary: Option<bool> = None;
                let mut metric_id: Option<String> = None;
                let mut metric_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_primary" => {
                            is_primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_name" => {
                            metric_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_primary = is_primary.ok_or_else(|| M::Error::missing_field("is_primary"))?;
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;
                let metric_name =
                    metric_name.ok_or_else(|| M::Error::missing_field("metric_name"))?;

                let content =
                    ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItems {
                        is_primary,
                        metric_id,
                        metric_name,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsExperimentMetricGroupMutationV2DataAttributesMetricsItemsVisitor,
        )
    }
}
