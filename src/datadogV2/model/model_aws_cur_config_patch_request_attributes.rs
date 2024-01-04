// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for AWS CUR config Patch Request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigPatchRequestAttributes {
    /// Whether or not the Cloud Cost Management account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
}

impl AwsCURConfigPatchRequestAttributes {
    pub fn new(is_enabled: bool) -> AwsCURConfigPatchRequestAttributes {
        AwsCURConfigPatchRequestAttributes { is_enabled }
    }
}
