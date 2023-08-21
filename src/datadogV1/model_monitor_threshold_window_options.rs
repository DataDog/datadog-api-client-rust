// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorThresholdWindowOptions {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    #[serde(rename = "recovery_window", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub recovery_window: Option<String>,
    /// Describes how long a metric must be anomalous before an alert triggers.
    #[serde(rename = "trigger_window", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub trigger_window: Option<String>,
}

