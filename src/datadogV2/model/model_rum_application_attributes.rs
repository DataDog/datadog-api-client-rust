// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RUM application attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMApplicationAttributes {
    /// ID of the API key associated with the application.
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<i32>,
    /// Indicates if flat sampling of Session Replay for APM-correlated RUM sessions is enabled.
    #[serde(rename = "apm_rum_flat_sampling_replay_enabled")]
    pub apm_rum_flat_sampling_replay_enabled: Option<bool>,
    /// Sample rate (0-100) of Session Replay for APM-correlated RUM sessions when flat sampling is enabled.
    #[serde(rename = "apm_rum_flat_sampling_replay_sample_rate")]
    pub apm_rum_flat_sampling_replay_sample_rate: Option<f64>,
    /// ID of the RUM application.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Client token of the RUM application.
    #[serde(rename = "client_token")]
    pub client_token: String,
    /// Timestamp in ms of the creation date.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Handle of the creator user.
    #[serde(rename = "created_by_handle")]
    pub created_by_handle: String,
    /// Timestamp in ms of the date the RUM application was disabled. Only present if the application was disabled.
    #[serde(rename = "disabled_at")]
    pub disabled_at: Option<i64>,
    /// Handle of the user who disabled the RUM application. Only present if the application was disabled.
    #[serde(rename = "disabled_by_handle")]
    pub disabled_by_handle: Option<String>,
    /// Indicates if Error Tracking exclusion filters are enabled for the application.
    #[serde(rename = "error_tracking_exclusion_filter_enabled")]
    pub error_tracking_exclusion_filter_enabled: Option<bool>,
    /// Non-native ingestion paths the RUM application sends its data through. `otel` means the application sends RUM data through OpenTelemetry instead of the Datadog RUM SDK.
    #[serde(rename = "external_feeds")]
    pub external_feeds: Option<Vec<String>>,
    /// Hash of the RUM application. Optional.
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    /// Indicates if the RUM application is active.
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates if the out-of-the-box RUM metrics are installed for the application.
    #[serde(rename = "ootb_metrics_installed")]
    pub ootb_metrics_installed: Option<bool>,
    /// Org ID of the RUM application.
    #[serde(rename = "org_id")]
    pub org_id: i32,
    /// Indicates that the Product Analytics preview is disabled for the application. Only present (and `true`) when the Product Analytics retention state is not preview.
    #[serde(rename = "product_analytics_preview_disabled")]
    pub product_analytics_preview_disabled: Option<bool>,
    /// Sample rate (0-100) of Session Replay for Product Analytics sessions.
    #[serde(rename = "product_analytics_replay_sample_rate")]
    pub product_analytics_replay_sample_rate: Option<i32>,
    /// Product Scales configuration for the RUM application.
    #[serde(rename = "product_scales")]
    pub product_scales: Option<crate::datadogV2::model::RUMProductScales>,
    /// ID of the RUM SDK remote configuration for the application, if one exists.
    #[serde(rename = "remote_config_id")]
    pub remote_config_id: Option<String>,
    /// Short name of the RUM application.
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    /// List of tags associated with the RUM application.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the RUM application. Possible values are `browser`, `ios`, `android`, `react-native`, `flutter`, `roku`, `electron`, `unity`, `kotlin-multiplatform`, `react`, `nextjs`, `angular`, `vuejs`, `nuxt`, `tanstack`, `maui`, `salesforce-lwc`, `shopify`, `cpp`, `rum-cpp`, `unreal`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Timestamp in ms of the last update date.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    /// Handle of the updater user.
    #[serde(rename = "updated_by_handle")]
    pub updated_by_handle: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMApplicationAttributes {
    pub fn new(
        application_id: String,
        client_token: String,
        created_at: i64,
        created_by_handle: String,
        name: String,
        org_id: i32,
        type_: String,
        updated_at: i64,
        updated_by_handle: String,
    ) -> RUMApplicationAttributes {
        RUMApplicationAttributes {
            api_key_id: None,
            apm_rum_flat_sampling_replay_enabled: None,
            apm_rum_flat_sampling_replay_sample_rate: None,
            application_id,
            client_token,
            created_at,
            created_by_handle,
            disabled_at: None,
            disabled_by_handle: None,
            error_tracking_exclusion_filter_enabled: None,
            external_feeds: None,
            hash: None,
            is_active: None,
            name,
            ootb_metrics_installed: None,
            org_id,
            product_analytics_preview_disabled: None,
            product_analytics_replay_sample_rate: None,
            product_scales: None,
            remote_config_id: None,
            short_name: None,
            tags: None,
            type_,
            updated_at,
            updated_by_handle,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_key_id(mut self, value: i32) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn apm_rum_flat_sampling_replay_enabled(mut self, value: bool) -> Self {
        self.apm_rum_flat_sampling_replay_enabled = Some(value);
        self
    }

    pub fn apm_rum_flat_sampling_replay_sample_rate(mut self, value: f64) -> Self {
        self.apm_rum_flat_sampling_replay_sample_rate = Some(value);
        self
    }

    pub fn disabled_at(mut self, value: i64) -> Self {
        self.disabled_at = Some(value);
        self
    }

    pub fn disabled_by_handle(mut self, value: String) -> Self {
        self.disabled_by_handle = Some(value);
        self
    }

    pub fn error_tracking_exclusion_filter_enabled(mut self, value: bool) -> Self {
        self.error_tracking_exclusion_filter_enabled = Some(value);
        self
    }

    pub fn external_feeds(mut self, value: Vec<String>) -> Self {
        self.external_feeds = Some(value);
        self
    }

    pub fn hash(mut self, value: String) -> Self {
        self.hash = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn ootb_metrics_installed(mut self, value: bool) -> Self {
        self.ootb_metrics_installed = Some(value);
        self
    }

    pub fn product_analytics_preview_disabled(mut self, value: bool) -> Self {
        self.product_analytics_preview_disabled = Some(value);
        self
    }

    pub fn product_analytics_replay_sample_rate(mut self, value: i32) -> Self {
        self.product_analytics_replay_sample_rate = Some(value);
        self
    }

    pub fn product_scales(mut self, value: crate::datadogV2::model::RUMProductScales) -> Self {
        self.product_scales = Some(value);
        self
    }

    pub fn remote_config_id(mut self, value: String) -> Self {
        self.remote_config_id = Some(value);
        self
    }

    pub fn short_name(mut self, value: String) -> Self {
        self.short_name = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for RUMApplicationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMApplicationAttributesVisitor;
        impl<'a> Visitor<'a> for RUMApplicationAttributesVisitor {
            type Value = RUMApplicationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key_id: Option<i32> = None;
                let mut apm_rum_flat_sampling_replay_enabled: Option<bool> = None;
                let mut apm_rum_flat_sampling_replay_sample_rate: Option<f64> = None;
                let mut application_id: Option<String> = None;
                let mut client_token: Option<String> = None;
                let mut created_at: Option<i64> = None;
                let mut created_by_handle: Option<String> = None;
                let mut disabled_at: Option<i64> = None;
                let mut disabled_by_handle: Option<String> = None;
                let mut error_tracking_exclusion_filter_enabled: Option<bool> = None;
                let mut external_feeds: Option<Vec<String>> = None;
                let mut hash: Option<String> = None;
                let mut is_active: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut ootb_metrics_installed: Option<bool> = None;
                let mut org_id: Option<i32> = None;
                let mut product_analytics_preview_disabled: Option<bool> = None;
                let mut product_analytics_replay_sample_rate: Option<i32> = None;
                let mut product_scales: Option<crate::datadogV2::model::RUMProductScales> = None;
                let mut remote_config_id: Option<String> = None;
                let mut short_name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<String> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by_handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_token" => {
                            client_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_handle" => {
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_at" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_tracking_exclusion_filter_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            error_tracking_exclusion_filter_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_feeds" => {
                            if v.is_null() {
                                continue;
                            }
                            external_feeds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hash" => {
                            if v.is_null() {
                                continue;
                            }
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            if v.is_null() {
                                continue;
                            }
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ootb_metrics_installed" => {
                            if v.is_null() {
                                continue;
                            }
                            ootb_metrics_installed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_preview_disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_preview_disabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_replay_sample_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_scales" => {
                            if v.is_null() {
                                continue;
                            }
                            product_scales =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_config_id" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_config_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            if v.is_null() {
                                continue;
                            }
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by_handle" => {
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let client_token =
                    client_token.ok_or_else(|| M::Error::missing_field("client_token"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by_handle = created_by_handle
                    .ok_or_else(|| M::Error::missing_field("created_by_handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by_handle = updated_by_handle
                    .ok_or_else(|| M::Error::missing_field("updated_by_handle"))?;

                let content = RUMApplicationAttributes {
                    api_key_id,
                    apm_rum_flat_sampling_replay_enabled,
                    apm_rum_flat_sampling_replay_sample_rate,
                    application_id,
                    client_token,
                    created_at,
                    created_by_handle,
                    disabled_at,
                    disabled_by_handle,
                    error_tracking_exclusion_filter_enabled,
                    external_feeds,
                    hash,
                    is_active,
                    name,
                    ootb_metrics_installed,
                    org_id,
                    product_analytics_preview_disabled,
                    product_analytics_replay_sample_rate,
                    product_scales,
                    remote_config_id,
                    short_name,
                    tags,
                    type_,
                    updated_at,
                    updated_by_handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMApplicationAttributesVisitor)
    }
}
