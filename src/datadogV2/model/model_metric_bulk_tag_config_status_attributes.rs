// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Optional attributes for the status of a bulk tag configuration request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigStatusAttributes {
    /// A list of account emails to notify when the configuration is applied.
    #[serde(rename = "emails")]
    pub emails: Option<Vec<String>>,
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// The status of the request.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// A list of tag names to apply to the configuration.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl MetricBulkTagConfigStatusAttributes {
    pub fn new() -> MetricBulkTagConfigStatusAttributes {
        MetricBulkTagConfigStatusAttributes {
            emails: None,
            exclude_tags_mode: None,
            status: None,
            tags: None,
        }
    }
}
