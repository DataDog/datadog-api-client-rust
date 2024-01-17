// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the index exclusion filter object from configuration API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsExclusion {
    /// Exclusion filter is defined by a query, a sampling rule, and a active/inactive toggle.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV1::model::LogsExclusionFilter>>,
    /// Whether or not the exclusion filter is active.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the index exclusion filter.
    #[serde(rename = "name")]
    pub name: String,
}

impl LogsExclusion {
    pub fn new(name: String) -> LogsExclusion {
        LogsExclusion {
            filter: None,
            is_enabled: None,
            name,
        }
    }
}
