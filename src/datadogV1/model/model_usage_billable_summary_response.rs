// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with monthly summary of data billed by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryResponse {
    /// An array of objects regarding usage of billable summary.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageBillableSummaryHour>>,
}

impl UsageBillableSummaryResponse {
    pub fn new() -> UsageBillableSummaryResponse {
        UsageBillableSummaryResponse { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageBillableSummaryHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for UsageBillableSummaryResponse {
    fn default() -> Self {
        Self::new()
    }
}
