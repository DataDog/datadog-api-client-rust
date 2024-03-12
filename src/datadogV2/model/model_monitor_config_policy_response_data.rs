// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A monitor configuration policy data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyResponseData {
    /// Policy and policy type for a monitor configuration policy.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MonitorConfigPolicyAttributeResponse>,
    /// ID of this monitor configuration policy.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Monitor configuration policy resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MonitorConfigPolicyResourceType>,
}

impl MonitorConfigPolicyResponseData {
    pub fn new() -> MonitorConfigPolicyResponseData {
        MonitorConfigPolicyResponseData {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::MonitorConfigPolicyAttributeResponse,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::MonitorConfigPolicyResourceType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorConfigPolicyResponseData {
    fn default() -> Self {
        Self::new()
    }
}
