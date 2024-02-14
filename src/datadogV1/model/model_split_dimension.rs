// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The property by which the graph splits
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitDimension {
    /// The system interprets this attribute differently depending on the data source of the query being split. For metrics, it's a tag. For the events platform, it's an attribute or tag.
    #[serde(rename = "one_graph_per")]
    pub one_graph_per: String,
}

impl SplitDimension {
    pub fn new(one_graph_per: String) -> SplitDimension {
        SplitDimension { one_graph_per }
    }
}
