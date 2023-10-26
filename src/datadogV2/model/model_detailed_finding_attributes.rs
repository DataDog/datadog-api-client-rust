// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DetailedFindingAttributes {
    /// The evaluation of the finding.
    #[serde(rename = "evaluation", skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// The date on which the evaluation for this finding changed (Unix ms).
    #[serde(rename = "evaluation_changed_at", skip_serializing_if = "Option::is_none")]
    pub evaluation_changed_at: Option<i64>,
    /// The remediation message for this finding.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Information about the mute status of this finding.
    #[serde(rename = "mute", skip_serializing_if = "Option::is_none")]
    pub mute: Option<Box<crate::datadogV2::model::FindingMute>>,
    /// The resource name of this finding.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The resource configuration for this finding.
    #[serde(rename = "resource_configuration", skip_serializing_if = "Option::is_none")]
    pub resource_configuration: Option<Box<crate::datadogV2::model::None>>,
    /// The date on which the resource was discovered (Unix ms).
    #[serde(rename = "resource_discovery_date", skip_serializing_if = "Option::is_none")]
    pub resource_discovery_date: Option<i64>,
    /// The resource type of this finding.
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The rule that triggered this finding.
    #[serde(rename = "rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<Box<crate::datadogV2::model::FindingRule>>,
    /// The status of the finding.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::datadogV2::model::FindingStatus>,
    /// The tags associated with this finding.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl DetailedFindingAttributes {
    /// The JSON:API attributes of the detailed finding.
    pub fn new() -> DetailedFindingAttributes {
        DetailedFindingAttributes {
            evaluation: None,
            evaluation_changed_at: None,
            message: None,
            mute: None,
            resource: None,
            resource_configuration: None,
            resource_discovery_date: None,
            resource_type: None,
            rule: None,
            status: None,
            tags: None,
        }
    }
}
