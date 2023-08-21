// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookCellUpdateRequest {
    /// The attributes of a notebook cell in update cell request. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
`log_stream`. [More information on each graph visualization type.](https://docs.datadoghq.com/dashboards/widgets/)
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: NotebookCellUpdateRequestAttributes,
    /// Notebook cell ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Type of the Notebook Cell resource.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: NotebookCellResourceType,
}

