// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Encapsulates all user choices about how to split a graph.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Maximum number of graphs to display in the widget.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Controls the order in which graphs appear in the split.
    #[serde(rename = "sort")]
    pub sort: crate::datadogV1::model::SplitSort,
    /// The dimension(s) on which to split the graph
    #[serde(rename = "split_dimensions")]
    pub split_dimensions: Vec<crate::datadogV1::model::SplitDimension>,
    /// Manual selection of tags making split graph widget static
    #[serde(rename = "static_splits")]
    pub static_splits: Option<Vec<Vec<crate::datadogV1::model::SplitVectorEntryItem>>>,
}

impl SplitConfig {
    pub fn new(
        limit: i64,
        sort: crate::datadogV1::model::SplitSort,
        split_dimensions: Vec<crate::datadogV1::model::SplitDimension>,
    ) -> SplitConfig {
        SplitConfig {
            limit,
            sort,
            split_dimensions,
            static_splits: None,
        }
    }

    pub fn static_splits(
        &mut self,
        value: Vec<Vec<crate::datadogV1::model::SplitVectorEntryItem>>,
    ) -> &mut Self {
        self.static_splits = Some(value);
        self
    }
}
