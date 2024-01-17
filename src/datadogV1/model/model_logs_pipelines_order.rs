// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the ordered list of pipeline IDs.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipelinesOrder {
    /// Ordered Array of `<PIPELINE_ID>` strings, the order of pipeline IDs in the array
    /// define the overall Pipelines order for Datadog.
    #[serde(rename = "pipeline_ids")]
    pub pipeline_ids: Vec<String>,
}

impl LogsPipelinesOrder {
    pub fn new(pipeline_ids: Vec<String>) -> LogsPipelinesOrder {
        LogsPipelinesOrder { pipeline_ids }
    }
}
