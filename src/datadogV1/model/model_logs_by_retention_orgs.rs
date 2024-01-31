// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Indexed logs usage summary for each organization for each retention period with usage.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetentionOrgs {
    /// Indexed logs usage summary for each organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsByRetentionOrgUsage>>,
}

impl LogsByRetentionOrgs {
    pub fn new() -> LogsByRetentionOrgs {
        LogsByRetentionOrgs { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsByRetentionOrgUsage>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for LogsByRetentionOrgs {
    fn default() -> Self {
        Self::new()
    }
}
