// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the logs filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsCategoryProcessorCategory {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV1::model::LogsFilter>>,
    /// Value to assign to the target attribute.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl LogsCategoryProcessorCategory {
    pub fn new() -> LogsCategoryProcessorCategory {
        LogsCategoryProcessorCategory {
            filter: None,
            name: None,
        }
    }
}
