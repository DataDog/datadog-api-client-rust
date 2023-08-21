// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageRumSessionsHour {
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Contains the number of RUM Replay Sessions (data available beginning November 1, 2021).
    #[serde(rename = "replay_session_count", skip_serializing_if = "Option::is_none")]
    pub replay_session_count: i64,
    /// Contains the number of browser RUM Lite Sessions.
    #[serde(rename = "session_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_count: Option<Int64>,
    /// Contains the number of mobile RUM Sessions on Android (data available beginning December 1, 2020).
    #[serde(rename = "session_count_android", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_count_android: Option<Int64>,
    /// Contains the number of mobile RUM Sessions on Flutter (data available beginning March 1, 2023).
    #[serde(rename = "session_count_flutter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_count_flutter: Option<Int64>,
    /// Contains the number of mobile RUM Sessions on iOS (data available beginning December 1, 2020).
    #[serde(rename = "session_count_ios", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_count_ios: Option<Int64>,
    /// Contains the number of mobile RUM Sessions on React Native (data available beginning May 1, 2022).
    #[serde(rename = "session_count_reactnative", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_count_reactnative: Option<Int64>,
}

