// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a notebook `distribution` cell.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookDistributionCellAttributes {
    /// The Distribution visualization is another way of showing metrics
    /// aggregated across one or several tags, such as hosts.
    /// Unlike the heat map, a distribution graph’s x-axis is quantity rather than time.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::DistributionWidgetDefinition,
    /// The size of the graph.
    #[serde(rename = "graph_size")]
    pub graph_size: Option<crate::datadogV1::model::NotebookGraphSize>,
    /// Object describing how to split the graph to display multiple visualizations per request.
    #[serde(rename = "split_by")]
    pub split_by: Option<crate::datadogV1::model::NotebookSplitBy>,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option")]
    pub time: Option<Option<crate::datadogV1::model::NotebookCellTime>>,
}

impl NotebookDistributionCellAttributes {
    pub fn new(
        definition: crate::datadogV1::model::DistributionWidgetDefinition,
    ) -> NotebookDistributionCellAttributes {
        NotebookDistributionCellAttributes {
            definition,
            graph_size: None,
            split_by: None,
            time: None,
        }
    }

    pub fn with_graph_size(
        &mut self,
        value: crate::datadogV1::model::NotebookGraphSize,
    ) -> &mut Self {
        self.graph_size = Some(value);
        self
    }

    pub fn with_split_by(&mut self, value: crate::datadogV1::model::NotebookSplitBy) -> &mut Self {
        self.split_by = Some(value);
        self
    }

    pub fn with_time(
        &mut self,
        value: Option<crate::datadogV1::model::NotebookCellTime>,
    ) -> &mut Self {
        self.time = Some(value);
        self
    }
}
