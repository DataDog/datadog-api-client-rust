// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Metrics Collection config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricsConfig {
    /// Enable EC2 automute for AWS metrics. Defaults to `true`.
    #[serde(rename = "automute_enabled")]
    pub automute_enabled: Option<bool>,
    /// Enable CloudWatch alarms collection. Defaults to `false`.
    #[serde(rename = "collect_cloudwatch_alarms")]
    pub collect_cloudwatch_alarms: Option<bool>,
    /// Enable custom metrics collection. Defaults to `false`.
    #[serde(rename = "collect_custom_metrics")]
    pub collect_custom_metrics: Option<bool>,
    /// Enable Service Quotas collection. Defaults to `false`.
    #[serde(rename = "collect_service_quotas")]
    pub collect_service_quotas: Option<bool>,
    /// Enable AWS metrics collection. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// AWS CloudWatch metric name filters. Each filter applies to a single namespace.
    /// Exactly one of `include_only` or `exclude_only` must be set on each filter.
    #[serde(rename = "metric_name_filters")]
    pub metric_name_filters: Option<Vec<crate::datadogV2::model::AWSMetricNameFilters>>,
    /// AWS Metrics namespace filters. Defaults to `exclude_only`.
    #[serde(rename = "namespace_filters")]
    pub namespace_filters: Option<crate::datadogV2::model::AWSNamespaceFilters>,
    /// AWS Metrics collection resource type tag filters list. Defaults to `[]`.
    #[serde(rename = "resource_type_tag_filters")]
    pub resource_type_tag_filters: Option<Vec<crate::datadogV2::model::AWSResourceTypeTagFilter>>,
    /// AWS Metrics collection tag filters list. Defaults to `[]`.
    /// Deprecated in favor of `resource_type_tag_filters`. The two fields are mutually exclusive on an account.
    #[deprecated]
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<Vec<crate::datadogV2::model::AWSNamespaceTagFilter>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricsConfig {
    pub fn new() -> AWSMetricsConfig {
        #[allow(deprecated)]
        AWSMetricsConfig {
            automute_enabled: None,
            collect_cloudwatch_alarms: None,
            collect_custom_metrics: None,
            collect_service_quotas: None,
            enabled: None,
            metric_name_filters: None,
            namespace_filters: None,
            resource_type_tag_filters: None,
            tag_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn automute_enabled(mut self, value: bool) -> Self {
        self.automute_enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn collect_cloudwatch_alarms(mut self, value: bool) -> Self {
        self.collect_cloudwatch_alarms = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn collect_custom_metrics(mut self, value: bool) -> Self {
        self.collect_custom_metrics = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn collect_service_quotas(mut self, value: bool) -> Self {
        self.collect_service_quotas = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn metric_name_filters(
        mut self,
        value: Vec<crate::datadogV2::model::AWSMetricNameFilters>,
    ) -> Self {
        self.metric_name_filters = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn namespace_filters(
        mut self,
        value: crate::datadogV2::model::AWSNamespaceFilters,
    ) -> Self {
        self.namespace_filters = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn resource_type_tag_filters(
        mut self,
        value: Vec<crate::datadogV2::model::AWSResourceTypeTagFilter>,
    ) -> Self {
        self.resource_type_tag_filters = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn tag_filters(
        mut self,
        value: Vec<crate::datadogV2::model::AWSNamespaceTagFilter>,
    ) -> Self {
        self.tag_filters = Some(value);
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

impl Default for AWSMetricsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSMetricsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricsConfigVisitor;
        impl<'a> Visitor<'a> for AWSMetricsConfigVisitor {
            type Value = AWSMetricsConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut automute_enabled: Option<bool> = None;
                let mut collect_cloudwatch_alarms: Option<bool> = None;
                let mut collect_custom_metrics: Option<bool> = None;
                let mut collect_service_quotas: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut metric_name_filters: Option<
                    Vec<crate::datadogV2::model::AWSMetricNameFilters>,
                > = None;
                let mut namespace_filters: Option<crate::datadogV2::model::AWSNamespaceFilters> =
                    None;
                let mut resource_type_tag_filters: Option<
                    Vec<crate::datadogV2::model::AWSResourceTypeTagFilter>,
                > = None;
                let mut tag_filters: Option<Vec<crate::datadogV2::model::AWSNamespaceTagFilter>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "automute_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            automute_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "collect_cloudwatch_alarms" => {
                            if v.is_null() {
                                continue;
                            }
                            collect_cloudwatch_alarms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "collect_custom_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            collect_custom_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "collect_service_quotas" => {
                            if v.is_null() {
                                continue;
                            }
                            collect_service_quotas =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_name_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_name_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _namespace_filters) = namespace_filters {
                                match _namespace_filters {
                                    crate::datadogV2::model::AWSNamespaceFilters::UnparsedObject(_namespace_filters) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "resource_type_tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type_tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = AWSMetricsConfig {
                    automute_enabled,
                    collect_cloudwatch_alarms,
                    collect_custom_metrics,
                    collect_service_quotas,
                    enabled,
                    metric_name_filters,
                    namespace_filters,
                    resource_type_tag_filters,
                    tag_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricsConfigVisitor)
    }
}
