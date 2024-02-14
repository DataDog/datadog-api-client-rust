// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of Fargate tasks run and hourly usage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageFargateResponse {
    /// Array with the number of hourly Fargate tasks recorded for a given organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageFargateHour>>,
}

impl UsageFargateResponse {
    pub fn new() -> UsageFargateResponse {
        UsageFargateResponse { usage: None }
    }

    pub fn usage(&mut self, value: Vec<crate::datadogV1::model::UsageFargateHour>) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageFargateResponse {
    fn default() -> Self {
        Self::new()
    }
}
