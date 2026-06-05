// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The configuration used when `mode` is `custom`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRateLimitCustomConfig {
    /// The time of day when the daily quota resets, in `HH:MM` 24-hour format.
    #[serde(rename = "daily_reset_time")]
    pub daily_reset_time: String,
    /// The timezone offset used for the daily reset time, in `±HH:MM` format.
    #[serde(rename = "daily_reset_timezone")]
    pub daily_reset_timezone: String,
    /// The action to take when the session quota is reached.
    #[serde(rename = "quota_reached_action")]
    pub quota_reached_action: crate::datadogV2::model::RumRateLimitQuotaReachedAction,
    /// The maximum number of sessions allowed within the window.
    #[serde(rename = "session_limit")]
    pub session_limit: i64,
    /// The window type over which the session limit is enforced.
    #[serde(rename = "window_type")]
    pub window_type: crate::datadogV2::model::RumRateLimitWindowType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRateLimitCustomConfig {
    pub fn new(
        daily_reset_time: String,
        daily_reset_timezone: String,
        quota_reached_action: crate::datadogV2::model::RumRateLimitQuotaReachedAction,
        session_limit: i64,
        window_type: crate::datadogV2::model::RumRateLimitWindowType,
    ) -> RumRateLimitCustomConfig {
        RumRateLimitCustomConfig {
            daily_reset_time,
            daily_reset_timezone,
            quota_reached_action,
            session_limit,
            window_type,
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

impl<'de> Deserialize<'de> for RumRateLimitCustomConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRateLimitCustomConfigVisitor;
        impl<'a> Visitor<'a> for RumRateLimitCustomConfigVisitor {
            type Value = RumRateLimitCustomConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut daily_reset_time: Option<String> = None;
                let mut daily_reset_timezone: Option<String> = None;
                let mut quota_reached_action: Option<
                    crate::datadogV2::model::RumRateLimitQuotaReachedAction,
                > = None;
                let mut session_limit: Option<i64> = None;
                let mut window_type: Option<crate::datadogV2::model::RumRateLimitWindowType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "daily_reset_time" => {
                            daily_reset_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "daily_reset_timezone" => {
                            daily_reset_timezone =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quota_reached_action" => {
                            quota_reached_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _quota_reached_action) = quota_reached_action {
                                match _quota_reached_action {
                                    crate::datadogV2::model::RumRateLimitQuotaReachedAction::UnparsedObject(_quota_reached_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "session_limit" => {
                            session_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window_type" => {
                            window_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _window_type) = window_type {
                                match _window_type {
                                    crate::datadogV2::model::RumRateLimitWindowType::UnparsedObject(_window_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let daily_reset_time =
                    daily_reset_time.ok_or_else(|| M::Error::missing_field("daily_reset_time"))?;
                let daily_reset_timezone = daily_reset_timezone
                    .ok_or_else(|| M::Error::missing_field("daily_reset_timezone"))?;
                let quota_reached_action = quota_reached_action
                    .ok_or_else(|| M::Error::missing_field("quota_reached_action"))?;
                let session_limit =
                    session_limit.ok_or_else(|| M::Error::missing_field("session_limit"))?;
                let window_type =
                    window_type.ok_or_else(|| M::Error::missing_field("window_type"))?;

                let content = RumRateLimitCustomConfig {
                    daily_reset_time,
                    daily_reset_timezone,
                    quota_reached_action,
                    session_limit,
                    window_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRateLimitCustomConfigVisitor)
    }
}
