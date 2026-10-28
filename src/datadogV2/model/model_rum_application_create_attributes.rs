// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RUM application creation attributes.
/// When `rum_event_processing_state` is omitted, it defaults to `ALL`. When `product_analytics_retention_state` is omitted, Product Analytics is not enabled. Setting it to `MAX` requires the `product_analytics_apps_write` permission.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMApplicationCreateAttributes {
    /// Indicates if flat sampling of Session Replay for APM-correlated RUM sessions is enabled. Defaults to `true`.
    #[serde(rename = "apm_rum_flat_sampling_replay_enabled")]
    pub apm_rum_flat_sampling_replay_enabled: Option<bool>,
    /// Sample rate (0-100) of Session Replay for APM-correlated RUM sessions when flat sampling is enabled. Defaults to `1`.
    #[serde(rename = "apm_rum_flat_sampling_replay_sample_rate")]
    pub apm_rum_flat_sampling_replay_sample_rate: Option<f64>,
    /// Whether to create the default retention filters when the application is created: `Sessions with errors` and `Sessions`, each with a 100% retention rate.
    /// Set to `false` to create the application without them, for example when you add your own retention filters right after creating it. Until you add retention filters, the application only retains sessions matched by its [permanent retention filters](<https://docs.datadoghq.com/real_user_monitoring/rum_without_limits/retention_filters/#permanent-retention-filters>).
    /// This setting doesn't affect permanent retention filters (sessions with forced replays, Synthetics sessions and RUM-APM flat sampling).
    #[serde(rename = "create_default_retention_filters")]
    pub create_default_retention_filters: Option<bool>,
    /// Indicates if Error Tracking exclusion filters are enabled for the application. Defaults to `false`.
    #[serde(rename = "error_tracking_exclusion_filter_enabled")]
    pub error_tracking_exclusion_filter_enabled: Option<bool>,
    /// Name of the RUM application. Between 1 and 255 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// Sample rate (0-100) of Session Replay for Product Analytics sessions. Defaults to `100`.
    #[serde(rename = "product_analytics_replay_sample_rate")]
    pub product_analytics_replay_sample_rate: Option<i32>,
    /// Controls the retention policy for Product Analytics data derived from RUM events.
    /// Requests also accept `PREVIEW` (the initial state of a new application, reported as `NONE` in responses). Setting `MAX` requires the `product_analytics_apps_write` permission.
    #[serde(rename = "product_analytics_retention_state")]
    pub product_analytics_retention_state:
        Option<crate::datadogV2::model::RUMProductAnalyticsRetentionState>,
    /// Configures which RUM events are processed and stored for the application. Defaults to `ALL` when creating an application.
    #[serde(rename = "rum_event_processing_state")]
    pub rum_event_processing_state: Option<crate::datadogV2::model::RUMEventProcessingState>,
    /// List of tags to associate with the RUM application, in `key:value` format.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the RUM application. Defaults to `browser`. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`, `roku`, `electron`, `unity`, `kotlin-multiplatform`, `react`, `nextjs`, `angular`, `vuejs`, `nuxt`, `tanstack`, `maui`, `salesforce-lwc`, `shopify`, `cpp`, `rum-cpp`, `unreal`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMApplicationCreateAttributes {
    pub fn new(name: String) -> RUMApplicationCreateAttributes {
        RUMApplicationCreateAttributes {
            apm_rum_flat_sampling_replay_enabled: None,
            apm_rum_flat_sampling_replay_sample_rate: None,
            create_default_retention_filters: None,
            error_tracking_exclusion_filter_enabled: None,
            name,
            product_analytics_replay_sample_rate: None,
            product_analytics_retention_state: None,
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

    pub fn create_default_retention_filters(mut self, value: bool) -> Self {
        self.create_default_retention_filters = Some(value);
        self
    }

    pub fn error_tracking_exclusion_filter_enabled(mut self, value: bool) -> Self {
        self.error_tracking_exclusion_filter_enabled = Some(value);
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

impl<'de> Deserialize<'de> for RUMApplicationCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMApplicationCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RUMApplicationCreateAttributesVisitor {
            type Value = RUMApplicationCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_rum_flat_sampling_replay_enabled: Option<bool> = None;
                let mut apm_rum_flat_sampling_replay_sample_rate: Option<f64> = None;
                let mut create_default_retention_filters: Option<bool> = None;
                let mut error_tracking_exclusion_filter_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut product_analytics_replay_sample_rate: Option<i32> = None;
                let mut product_analytics_retention_state: Option<
                    crate::datadogV2::model::RUMProductAnalyticsRetentionState,
                > = None;
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
                        "create_default_retention_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            create_default_retention_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_exclusion_filter_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_exclusion_filter_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = RUMApplicationCreateAttributes {
                    apm_rum_flat_sampling_replay_enabled,
                    apm_rum_flat_sampling_replay_sample_rate,
                    create_default_retention_filters,
                    error_tracking_exclusion_filter_enabled,
                    name,
                    product_analytics_replay_sample_rate,
                    product_analytics_retention_state,
                    rum_event_processing_state,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMApplicationCreateAttributesVisitor)
    }
}
