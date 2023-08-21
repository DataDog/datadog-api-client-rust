// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookLogStreamCellAttributes {
    /// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
    #[serde(rename = "definition")]
    pub definition: LogStreamWidgetDefinition,
    /// The size of the graph.
    #[serde(rename = "graph_size", skip_serializing_if = "Option::is_none")]
    pub graph_size: NotebookGraphSize,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub time: NullableNotebookCellTime,
}

