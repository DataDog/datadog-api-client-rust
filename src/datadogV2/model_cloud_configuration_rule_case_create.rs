// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRuleCaseCreate {
    /// Notification targets for each rule case.
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Vec<String>,
    /// Severity of the Security Signal.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SecurityMonitoringRuleSeverity,
}

