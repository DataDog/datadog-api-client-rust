// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of indexed logs for each hour and index for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByIndexResponse {
    /// An array of objects regarding hourly usage of logs by index response.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageLogsByIndexHour>>,
}

impl UsageLogsByIndexResponse {
    pub fn new() -> UsageLogsByIndexResponse {
        UsageLogsByIndexResponse { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageLogsByIndexHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for UsageLogsByIndexResponse {
    fn default() -> Self {
        Self::new()
    }
}
