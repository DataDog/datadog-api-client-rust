// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Tag attributes of a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyTagPolicy {
    /// The key of the tag.
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    /// If a tag key is required for monitor creation.
    #[serde(rename = "tag_key_required")]
    pub tag_key_required: Option<bool>,
    /// Valid values for the tag.
    #[serde(rename = "valid_tag_values")]
    pub valid_tag_values: Option<Vec<String>>,
}

impl MonitorConfigPolicyTagPolicy {
    pub fn new() -> MonitorConfigPolicyTagPolicy {
        MonitorConfigPolicyTagPolicy {
            tag_key: None,
            tag_key_required: None,
            valid_tag_values: None,
        }
    }

    pub fn tag_key(mut self, value: String) -> Self {
        self.tag_key = Some(value);
        self
    }

    pub fn tag_key_required(mut self, value: bool) -> Self {
        self.tag_key_required = Some(value);
        self
    }

    pub fn valid_tag_values(mut self, value: Vec<String>) -> Self {
        self.valid_tag_values = Some(value);
        self
    }
}

impl Default for MonitorConfigPolicyTagPolicy {
    fn default() -> Self {
        Self::new()
    }
}
