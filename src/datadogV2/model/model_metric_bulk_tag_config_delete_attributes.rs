// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Optional parameters for bulk deleting metric tag configurations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigDeleteAttributes {
    /// A list of account emails to notify when the configuration is applied.
    #[serde(rename = "emails")]
    pub emails: Option<Vec<String>>,
}

impl MetricBulkTagConfigDeleteAttributes {
    pub fn new() -> MetricBulkTagConfigDeleteAttributes {
        MetricBulkTagConfigDeleteAttributes { emails: None }
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }
}

impl Default for MetricBulkTagConfigDeleteAttributes {
    fn default() -> Self {
        Self::new()
    }
}
