// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response containing attributes for custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCustomReportsAttributes {
    /// The date the specified custom report was computed.
    #[serde(rename = "computed_on")]
    pub computed_on: Option<String>,
    /// The ending date of custom report.
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// size
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// The starting date of custom report.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// A list of tags to apply to custom reports.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl UsageCustomReportsAttributes {
    pub fn new() -> UsageCustomReportsAttributes {
        UsageCustomReportsAttributes {
            computed_on: None,
            end_date: None,
            size: None,
            start_date: None,
            tags: None,
        }
    }

    pub fn computed_on(mut self, value: String) -> Self {
        self.computed_on = Some(value);
        self
    }

    pub fn end_date(mut self, value: String) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for UsageCustomReportsAttributes {
    fn default() -> Self {
        Self::new()
    }
}
