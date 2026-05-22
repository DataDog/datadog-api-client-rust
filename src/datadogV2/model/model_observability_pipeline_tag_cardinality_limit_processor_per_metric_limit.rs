// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A cardinality override applied to a specific metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
    /// The action to take when the cardinality limit is exceeded.
    #[serde(rename = "limit_exceeded_action")]
    pub limit_exceeded_action:
        Option<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorAction>,
    /// The name of the metric this override applies to.
    #[serde(rename = "metric_name")]
    pub metric_name: String,
    /// How the per-metric override is applied. `tracked` enforces a custom limit; `excluded` skips the metric entirely.
    #[serde(rename = "mode")]
    pub mode:
        crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerMetricMode,
    /// A list of per-tag cardinality overrides that apply within this metric. Must be omitted when `mode` is `excluded`.
    #[serde(rename = "per_tag_limits")]
    pub per_tag_limits: Option<
        Vec<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit>,
    >,
    /// The maximum number of distinct tag value combinations allowed for this metric. Required when `mode` is `tracked`. Must be omitted when `mode` is `excluded`.
    #[serde(rename = "value_limit")]
    pub value_limit: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
    pub fn new(
        metric_name: String,
        mode: crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerMetricMode,
    ) -> ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
        ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
            limit_exceeded_action: None,
            metric_name,
            mode,
            per_tag_limits: None,
            value_limit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit_exceeded_action(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorAction,
    ) -> Self {
        self.limit_exceeded_action = Some(value);
        self
    }

    pub fn per_tag_limits(
        mut self,
        value: Vec<
            crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit,
        >,
    ) -> Self {
        self.per_tag_limits = Some(value);
        self
    }

    pub fn value_limit(mut self, value: i64) -> Self {
        self.value_limit = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimitVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimitVisitor {
            type Value = ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit_exceeded_action: Option<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorAction> = None;
                let mut metric_name: Option<String> = None;
                let mut mode: Option<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerMetricMode> = None;
                let mut per_tag_limits: Option<Vec<crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerTagLimit>> = None;
                let mut value_limit: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit_exceeded_action" => {
                            if v.is_null() {
                                continue;
                            }
                            limit_exceeded_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _limit_exceeded_action) = limit_exceeded_action {
                                match _limit_exceeded_action {
                                    crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorAction::UnparsedObject(_limit_exceeded_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metric_name" => {
                            metric_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineTagCardinalityLimitProcessorPerMetricMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "per_tag_limits" => {
                            if v.is_null() {
                                continue;
                            }
                            per_tag_limits =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_limit" => {
                            if v.is_null() {
                                continue;
                            }
                            value_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metric_name =
                    metric_name.ok_or_else(|| M::Error::missing_field("metric_name"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimit {
                    limit_exceeded_action,
                    metric_name,
                    mode,
                    per_tag_limits,
                    value_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineTagCardinalityLimitProcessorPerMetricLimitVisitor)
    }
}
