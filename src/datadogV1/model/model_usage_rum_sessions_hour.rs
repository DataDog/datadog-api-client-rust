// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of RUM Sessions recorded for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageRumSessionsHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
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
        }
    }

    pub fn hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn replay_session_count(&mut self, value: i64) -> &mut Self {
        self.replay_session_count = Some(value);
        self
    }

    pub fn session_count(&mut self, value: Option<i64>) -> &mut Self {
        self.session_count = Some(value);
        self
    }

    pub fn session_count_android(&mut self, value: Option<i64>) -> &mut Self {
        self.session_count_android = Some(value);
        self
    }

    pub fn session_count_flutter(&mut self, value: Option<i64>) -> &mut Self {
        self.session_count_flutter = Some(value);
        self
    }

    pub fn session_count_ios(&mut self, value: Option<i64>) -> &mut Self {
        self.session_count_ios = Some(value);
        self
    }

    pub fn session_count_reactnative(&mut self, value: Option<i64>) -> &mut Self {
        self.session_count_reactnative = Some(value);
        self
    }
}

impl Default for UsageRumSessionsHour {
    fn default() -> Self {
        Self::new()
    }
}
