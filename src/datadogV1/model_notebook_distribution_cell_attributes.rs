// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookDistributionCellAttributes {
    /// The Distribution visualization is another way of showing metrics
aggregated across one or several tags, such as hosts.
Unlike the heat map, a distribution graph’s x-axis is quantity rather than time.
    #[serde(rename = "definition")]
    pub definition: DistributionWidgetDefinition,
    /// The size of the graph.
    #[serde(rename = "graph_size", skip_serializing_if = "Option::is_none")]
    pub graph_size: NotebookGraphSize,
    /// Object describing how to split the graph to display multiple visualizations per request.
    #[serde(rename = "split_by")]
    pub split_by: NotebookSplitBy,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub time: NullableNotebookCellTime,
}

