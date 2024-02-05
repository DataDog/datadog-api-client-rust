// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The split graph list contains a graph for each value of the split dimension.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitVectorEntryItem {
    /// The tag key.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// The tag values.
    #[serde(rename = "tag_values")]
    pub tag_values: Vec<String>,
}

impl SplitVectorEntryItem {
    pub fn new(tag_key: String, tag_values: Vec<String>) -> SplitVectorEntryItem {
        SplitVectorEntryItem {
            tag_key,
            tag_values,
        }
    }
}
