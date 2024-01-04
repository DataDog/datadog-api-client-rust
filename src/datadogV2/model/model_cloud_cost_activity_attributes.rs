// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for Cloud Cost activity.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudCostActivityAttributes {
    /// Whether or not the cloud account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
}

impl CloudCostActivityAttributes {
    pub fn new(is_enabled: bool) -> CloudCostActivityAttributes {
        CloudCostActivityAttributes { is_enabled }
    }
}
