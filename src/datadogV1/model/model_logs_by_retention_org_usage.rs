// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Indexed logs usage by retention for a single organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetentionOrgUsage {
    /// Indexed logs usage for each active retention for the organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionSumUsage>>,
}

impl LogsByRetentionOrgUsage {
    pub fn new() -> LogsByRetentionOrgUsage {
        LogsByRetentionOrgUsage { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsRetentionSumUsage>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for LogsByRetentionOrgUsage {
    fn default() -> Self {
        Self::new()
    }
}
