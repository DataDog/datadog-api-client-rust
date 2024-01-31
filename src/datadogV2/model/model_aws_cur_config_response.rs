// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response of AWS CUR config.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigResponse {
    /// AWS CUR config.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::AwsCURConfig>,
}

impl AwsCURConfigResponse {
    pub fn new() -> AwsCURConfigResponse {
        AwsCURConfigResponse { data: None }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::AwsCURConfig) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for AwsCURConfigResponse {
    fn default() -> Self {
        Self::new()
    }
}
