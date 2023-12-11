// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response containing attributes for specified custom reports.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsAttributes {
    /// The date the specified custom report was computed.
    #[serde(rename = "computed_on")]
    pub computed_on: Option<String>,
    /// The ending date of specified custom report.
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// A downloadable file for the specified custom reporting file.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// size
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// The starting date of specified custom report.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// A list of tags to apply to specified custom reports.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl UsageSpecifiedCustomReportsAttributes {
    pub fn new() -> UsageSpecifiedCustomReportsAttributes {
        UsageSpecifiedCustomReportsAttributes {
            computed_on: None,
            end_date: None,
            location: None,
            size: None,
            start_date: None,
            tags: None,
        }
    }
}
