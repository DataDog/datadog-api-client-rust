// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options on impossible travel rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringRuleImpossibleTravelOptions {
    /// If true, signals are suppressed for the first 24 hours. In that time, Datadog learns the user's regular
    /// access locations. This can be helpful to reduce noise and infer VPN usage or credentialed API access.
    #[serde(rename = "baselineUserLocations")]
    pub baseline_user_locations: Option<bool>,
}

impl SecurityMonitoringRuleImpossibleTravelOptions {
    pub fn new() -> SecurityMonitoringRuleImpossibleTravelOptions {
        SecurityMonitoringRuleImpossibleTravelOptions {
            baseline_user_locations: None,
        }
    }

    pub fn baseline_user_locations(mut self, value: bool) -> Self {
        self.baseline_user_locations = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleImpossibleTravelOptions {
    fn default() -> Self {
        Self::new()
    }
}
