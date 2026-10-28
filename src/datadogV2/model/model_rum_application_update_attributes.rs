// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RUM application update attributes. Only the attributes you provide are changed, omitted attributes keep their current value.
/// Changing `product_analytics_retention_state` or `product_analytics_replay_sample_rate` requires the `product_analytics_apps_write` permission.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMApplicationUpdateAttributes {
    /// Indicates if flat sampling of Session Replay for APM-correlated RUM sessions is enabled.
    #[serde(rename = "apm_rum_flat_sampling_replay_enabled")]
    pub apm_rum_flat_sampling_replay_enabled: Option<bool>,
    /// Sample rate (0-100) of Session Replay for APM-correlated RUM sessions when flat sampling is enabled.
    #[serde(rename = "apm_rum_flat_sampling_replay_sample_rate")]
    pub apm_rum_flat_sampling_replay_sample_rate: Option<f64>,
    /// Indicates if Error Tracking exclusion filters are enabled for the application.
    #[serde(rename = "error_tracking_exclusion_filter_enabled")]
    pub error_tracking_exclusion_filter_enabled: Option<bool>,
    /// Instrumentation type of the RUM application.
    #[serde(rename = "instrumentation_type")]
    pub instrumentation_type: Option<String>,
    /// New name of the RUM application.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Sample rate (0-100) of Session Replay for Product Analytics sessions.
    #[serde(rename = "product_analytics_replay_sample_rate")]
    pub product_analytics_replay_sample_rate: Option<i32>,
    /// Controls the retention policy for Product Analytics data derived from RUM events.
    /// Requests also accept `PREVIEW` (the initial state of a new application, reported as `NONE` in responses). Setting `MAX` requires the `product_analytics_apps_write` permission.
    #[serde(rename = "product_analytics_retention_state")]
    pub product_analytics_retention_state:
        Option<crate::datadogV2::model::RUMProductAnalyticsRetentionState>,
    /// ID of the RUM SDK remote configuration to associate with the application.
    #[serde(rename = "remote_config_id")]
    pub remote_config_id: Option<String>,
    /// Configures which RUM events are processed and stored for the application. Defaults to `ALL` when creating an application.
    #[serde(rename = "rum_event_processing_state")]
    pub rum_event_processing_state: Option<crate::datadogV2::model::RUMEventProcessingState>,
    /// New list of tags of the RUM application, in `key:value` format. Replaces the existing tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// New type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`, `roku`, `electron`, `unity`, `kotlin-multiplatform`, `react`, `nextjs`, `angular`, `vuejs`, `nuxt`, `tanstack`, `maui`, `salesforce-lwc`, `shopify`, `cpp`, `rum-cpp`, `unreal`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMApplicationUpdateAttributes {
    pub fn new() -> RUMApplicationUpdateAttributes {
        RUMApplicationUpdateAttributes {
            apm_rum_flat_sampling_replay_enabled: None,
            apm_rum_flat_sampling_replay_sample_rate: None,
            error_tracking_exclusion_filter_enabled: None,
            instrumentation_type: None,
            name: None,
            product_analytics_replay_sample_rate: None,
            product_analytics_retention_state: None,
            remote_config_id: None,
            rum_event_processing_state: None,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn apm_rum_flat_sampling_replay_enabled(mut self, value: bool) -> Self {
        self.apm_rum_flat_sampling_replay_enabled = Some(value);
        self
    }

    pub fn apm_rum_flat_sampling_replay_sample_rate(mut self, value: f64) -> Self {
        self.apm_rum_flat_sampling_replay_sample_rate = Some(value);
        self
    }

    pub fn error_tracking_exclusion_filter_enabled(mut self, value: bool) -> Self {
        self.error_tracking_exclusion_filter_enabled = Some(value);
        self
    }

    pub fn instrumentation_type(mut self, value: String) -> Self {
        self.instrumentation_type = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn product_analytics_replay_sample_rate(mut self, value: i32) -> Self {
        self.product_analytics_replay_sample_rate = Some(value);
        self
    }

    pub fn product_analytics_retention_state(
        mut self,
        value: crate::datadogV2::model::RUMProductAnalyticsRetentionState,
    ) -> Self {
        self.product_analytics_retention_state = Some(value);
        self
    }

    pub fn remote_config_id(mut self, value: String) -> Self {
        self.remote_config_id = Some(value);
        self
    }

    pub fn rum_event_processing_state(
        mut self,
        value: crate::datadogV2::model::RUMEventProcessingState,
    ) -> Self {
        self.rum_event_processing_state = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for RUMApplicationUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMApplicationUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMApplicationUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RUMApplicationUpdateAttributesVisitor {
            type Value = RUMApplicationUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_rum_flat_sampling_replay_enabled: Option<bool> = None;
                let mut apm_rum_flat_sampling_replay_sample_rate: Option<f64> = None;
                let mut error_tracking_exclusion_filter_enabled: Option<bool> = None;
                let mut instrumentation_type: Option<String> = None;
                let mut name: Option<String> = None;
                let mut product_analytics_replay_sample_rate: Option<i32> = None;
                let mut product_analytics_retention_state: Option<
                    crate::datadogV2::model::RUMProductAnalyticsRetentionState,
                > = None;
                let mut remote_config_id: Option<String> = None;
                let mut rum_event_processing_state: Option<
                    crate::datadogV2::model::RUMEventProcessingState,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_rum_flat_sampling_replay_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_rum_flat_sampling_replay_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_rum_flat_sampling_replay_sample_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            apm_rum_flat_sampling_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_exclusion_filter_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_exclusion_filter_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instrumentation_type" => {
                            if v.is_null() {
                                continue;
                            }
                            instrumentation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_replay_sample_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_retention_state" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_retention_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _product_analytics_retention_state) =
                                product_analytics_retention_state
                            {
                                match _product_analytics_retention_state {
                                    crate::datadogV2::model::RUMProductAnalyticsRetentionState::UnparsedObject(_product_analytics_retention_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "remote_config_id" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_config_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_event_processing_state" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_event_processing_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rum_event_processing_state) =
                                rum_event_processing_state
                            {
                                match _rum_event_processing_state {
                                    crate::datadogV2::model::RUMEventProcessingState::UnparsedObject(_rum_event_processing_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMApplicationUpdateAttributes {
                    apm_rum_flat_sampling_replay_enabled,
                    apm_rum_flat_sampling_replay_sample_rate,
                    error_tracking_exclusion_filter_enabled,
                    instrumentation_type,
                    name,
                    product_analytics_replay_sample_rate,
                    product_analytics_retention_state,
                    remote_config_id,
                    rum_event_processing_state,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMApplicationUpdateAttributesVisitor)
    }
}
