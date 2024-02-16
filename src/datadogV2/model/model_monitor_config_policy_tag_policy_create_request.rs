// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Tag attributes of a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyTagPolicyCreateRequest {
    /// The key of the tag.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// If a tag key is required for monitor creation.
    #[serde(rename = "tag_key_required")]
    pub tag_key_required: bool,
    /// Valid values for the tag.
    #[serde(rename = "valid_tag_values")]
    pub valid_tag_values: Vec<String>,
}

impl MonitorConfigPolicyTagPolicyCreateRequest {
    pub fn new(
        tag_key: String,
        tag_key_required: bool,
        valid_tag_values: Vec<String>,
    ) -> MonitorConfigPolicyTagPolicyCreateRequest {
        MonitorConfigPolicyTagPolicyCreateRequest {
            tag_key,
            tag_key_required,
            valid_tag_values,
        }
    }
}
