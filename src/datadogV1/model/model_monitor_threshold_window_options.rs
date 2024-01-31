// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Alerting time window options.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorThresholdWindowOptions {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    #[serde(
        rename = "recovery_window",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub recovery_window: Option<Option<String>>,
    /// Describes how long a metric must be anomalous before an alert triggers.
    #[serde(
        rename = "trigger_window",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub trigger_window: Option<Option<String>>,
}

impl MonitorThresholdWindowOptions {
    pub fn new() -> MonitorThresholdWindowOptions {
        MonitorThresholdWindowOptions {
            recovery_window: None,
            trigger_window: None,
        }
    }

    pub fn with_recovery_window(&mut self, value: Option<String>) -> &mut Self {
        self.recovery_window = Some(value);
        self
    }

    pub fn with_trigger_window(&mut self, value: Option<String>) -> &mut Self {
        self.trigger_window = Some(value);
        self
    }
}
impl Default for MonitorThresholdWindowOptions {
    fn default() -> Self {
        Self::new()
    }
}
