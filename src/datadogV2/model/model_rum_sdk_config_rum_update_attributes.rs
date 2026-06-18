// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The RUM SDK settings to apply when updating a configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSdkConfigRumUpdateAttributes {
    /// A list of URL configurations for distributed tracing.
    #[serde(rename = "allowed_tracing_urls")]
    pub allowed_tracing_urls: Option<Vec<crate::datadogV2::model::RumSdkConfigTracingUrlConfig>>,
    /// A list of origin patterns allowed for cross-origin session tracking.
    #[serde(rename = "allowed_tracking_origins")]
    pub allowed_tracking_origins: Option<Vec<crate::datadogV2::model::RumSdkConfigMatchOption>>,
    /// A list of dynamic option key-value pairs.
    #[serde(rename = "context")]
    pub context: Option<Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>>,
    /// The default privacy masking level applied to all RUM data.
    #[serde(rename = "default_privacy_level")]
    pub default_privacy_level: String,
    /// Whether to mask user-interaction action names for privacy.
    #[serde(rename = "enable_privacy_for_action_name")]
    pub enable_privacy_for_action_name: bool,
    /// The environment tag for the RUM application.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// The service name tag for the RUM application.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The percentage of collected sessions for which a replay is captured (0–100).
    #[serde(rename = "session_replay_sample_rate")]
    pub session_replay_sample_rate: i64,
    /// The percentage of user sessions to collect (0–100).
    #[serde(rename = "session_sample_rate")]
    pub session_sample_rate: i64,
    /// The percentage of requests to forward as APM traces (0–100).
    #[serde(rename = "trace_sample_rate")]
    pub trace_sample_rate: Option<i64>,
    /// Whether to share a session across subdomains of the same site.
    #[serde(rename = "track_session_across_subdomains")]
    pub track_session_across_subdomains: Option<bool>,
    /// A list of dynamic option key-value pairs.
    #[serde(rename = "user")]
    pub user: Option<Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>>,
    /// A dynamic configuration option that extracts a value at runtime using a specified strategy.
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV2::model::RumSdkConfigDynamicOption>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSdkConfigRumUpdateAttributes {
    pub fn new(
        default_privacy_level: String,
        enable_privacy_for_action_name: bool,
        session_replay_sample_rate: i64,
        session_sample_rate: i64,
    ) -> RumSdkConfigRumUpdateAttributes {
        RumSdkConfigRumUpdateAttributes {
            allowed_tracing_urls: None,
            allowed_tracking_origins: None,
            context: None,
            default_privacy_level,
            enable_privacy_for_action_name,
            env: None,
            service: None,
            session_replay_sample_rate,
            session_sample_rate,
            trace_sample_rate: None,
            track_session_across_subdomains: None,
            user: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allowed_tracing_urls(
        mut self,
        value: Vec<crate::datadogV2::model::RumSdkConfigTracingUrlConfig>,
    ) -> Self {
        self.allowed_tracing_urls = Some(value);
        self
    }

    pub fn allowed_tracking_origins(
        mut self,
        value: Vec<crate::datadogV2::model::RumSdkConfigMatchOption>,
    ) -> Self {
        self.allowed_tracking_origins = Some(value);
        self
    }

    pub fn context(
        mut self,
        value: Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>,
    ) -> Self {
        self.context = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn trace_sample_rate(mut self, value: i64) -> Self {
        self.trace_sample_rate = Some(value);
        self
    }

    pub fn track_session_across_subdomains(mut self, value: bool) -> Self {
        self.track_session_across_subdomains = Some(value);
        self
    }

    pub fn user(
        mut self,
        value: Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>,
    ) -> Self {
        self.user = Some(value);
        self
    }

    pub fn version(mut self, value: crate::datadogV2::model::RumSdkConfigDynamicOption) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for RumSdkConfigRumUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSdkConfigRumUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RumSdkConfigRumUpdateAttributesVisitor {
            type Value = RumSdkConfigRumUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowed_tracing_urls: Option<
                    Vec<crate::datadogV2::model::RumSdkConfigTracingUrlConfig>,
                > = None;
                let mut allowed_tracking_origins: Option<
                    Vec<crate::datadogV2::model::RumSdkConfigMatchOption>,
                > = None;
                let mut context: Option<
                    Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>,
                > = None;
                let mut default_privacy_level: Option<String> = None;
                let mut enable_privacy_for_action_name: Option<bool> = None;
                let mut env: Option<String> = None;
                let mut service: Option<String> = None;
                let mut session_replay_sample_rate: Option<i64> = None;
                let mut session_sample_rate: Option<i64> = None;
                let mut trace_sample_rate: Option<i64> = None;
                let mut track_session_across_subdomains: Option<bool> = None;
                let mut user: Option<Vec<crate::datadogV2::model::RumSdkConfigDynamicOptionPair>> =
                    None;
                let mut version: Option<crate::datadogV2::model::RumSdkConfigDynamicOption> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowed_tracing_urls" => {
                            if v.is_null() {
                                continue;
                            }
                            allowed_tracing_urls =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allowed_tracking_origins" => {
                            if v.is_null() {
                                continue;
                            }
                            allowed_tracking_origins =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "context" => {
                            if v.is_null() {
                                continue;
                            }
                            context = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_privacy_level" => {
                            default_privacy_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enable_privacy_for_action_name" => {
                            enable_privacy_for_action_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_replay_sample_rate" => {
                            session_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_sample_rate" => {
                            session_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_sample_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "track_session_across_subdomains" => {
                            if v.is_null() {
                                continue;
                            }
                            track_session_across_subdomains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let default_privacy_level = default_privacy_level
                    .ok_or_else(|| M::Error::missing_field("default_privacy_level"))?;
                let enable_privacy_for_action_name = enable_privacy_for_action_name
                    .ok_or_else(|| M::Error::missing_field("enable_privacy_for_action_name"))?;
                let session_replay_sample_rate = session_replay_sample_rate
                    .ok_or_else(|| M::Error::missing_field("session_replay_sample_rate"))?;
                let session_sample_rate = session_sample_rate
                    .ok_or_else(|| M::Error::missing_field("session_sample_rate"))?;

                let content = RumSdkConfigRumUpdateAttributes {
                    allowed_tracing_urls,
                    allowed_tracking_origins,
                    context,
                    default_privacy_level,
                    enable_privacy_for_action_name,
                    env,
                    service,
                    session_replay_sample_rate,
                    session_sample_rate,
                    trace_sample_rate,
                    track_session_across_subdomains,
                    user,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSdkConfigRumUpdateAttributesVisitor)
    }
}
