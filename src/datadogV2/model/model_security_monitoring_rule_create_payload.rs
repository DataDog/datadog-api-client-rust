// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Create a new rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecurityMonitoringRuleCreatePayload {
    SecurityMonitoringStandardRuleCreatePayload(
        crate::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload,
    ),
    SecurityMonitoringSignalRuleCreatePayload(
        crate::datadogV2::model::SecurityMonitoringSignalRuleCreatePayload,
    ),
    CloudConfigurationRuleCreatePayload(
        crate::datadogV2::model::CloudConfigurationRuleCreatePayload,
    ),
}
