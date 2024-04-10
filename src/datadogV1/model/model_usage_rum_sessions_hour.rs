// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of RUM Sessions recorded for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageRumSessionsHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Contains the number of RUM Replay Sessions (data available beginning November 1, 2021).
    #[serde(rename = "replay_session_count")]
    pub replay_session_count: Option<i64>,
    /// Contains the number of browser RUM Lite Sessions.
    #[serde(
        rename = "session_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_count: Option<Option<i64>>,
    /// Contains the number of mobile RUM Sessions on Android (data available beginning December 1, 2020).
    #[serde(
        rename = "session_count_android",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_count_android: Option<Option<i64>>,
    /// Contains the number of mobile RUM Sessions on Flutter (data available beginning March 1, 2023).
    #[serde(
        rename = "session_count_flutter",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_count_flutter: Option<Option<i64>>,
    /// Contains the number of mobile RUM Sessions on iOS (data available beginning December 1, 2020).
    #[serde(
        rename = "session_count_ios",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_count_ios: Option<Option<i64>>,
    /// Contains the number of mobile RUM Sessions on React Native (data available beginning May 1, 2022).
    #[serde(
        rename = "session_count_reactnative",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_count_reactnative: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageRumSessionsHour {
    pub fn new() -> UsageRumSessionsHour {
        UsageRumSessionsHour {
            hour: None,
            org_name: None,
            public_id: None,
            replay_session_count: None,
            session_count: None,
            session_count_android: None,
            session_count_flutter: None,
            session_count_ios: None,
            session_count_reactnative: None,
            _unparsed: false,
        }
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn replay_session_count(mut self, value: i64) -> Self {
        self.replay_session_count = Some(value);
        self
    }

    pub fn session_count(mut self, value: Option<i64>) -> Self {
        self.session_count = Some(value);
        self
    }

    pub fn session_count_android(mut self, value: Option<i64>) -> Self {
        self.session_count_android = Some(value);
        self
    }

    pub fn session_count_flutter(mut self, value: Option<i64>) -> Self {
        self.session_count_flutter = Some(value);
        self
    }

    pub fn session_count_ios(mut self, value: Option<i64>) -> Self {
        self.session_count_ios = Some(value);
        self
    }

    pub fn session_count_reactnative(mut self, value: Option<i64>) -> Self {
        self.session_count_reactnative = Some(value);
        self
    }
}

impl Default for UsageRumSessionsHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageRumSessionsHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageRumSessionsHourVisitor;
        impl<'a> Visitor<'a> for UsageRumSessionsHourVisitor {
            type Value = UsageRumSessionsHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut replay_session_count: Option<i64> = None;
                let mut session_count: Option<Option<i64>> = None;
                let mut session_count_android: Option<Option<i64>> = None;
                let mut session_count_flutter: Option<Option<i64>> = None;
                let mut session_count_ios: Option<Option<i64>> = None;
                let mut session_count_reactnative: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "replay_session_count" => {
                            if v.is_null() {
                                continue;
                            }
                            replay_session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count" => {
                            session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count_android" => {
                            session_count_android =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count_flutter" => {
                            session_count_flutter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count_ios" => {
                            session_count_ios =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count_reactnative" => {
                            session_count_reactnative =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageRumSessionsHour {
                    hour,
                    org_name,
                    public_id,
                    replay_session_count,
                    session_count,
                    session_count_android,
                    session_count_flutter,
                    session_count_ios,
                    session_count_reactnative,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageRumSessionsHourVisitor)
    }
}
