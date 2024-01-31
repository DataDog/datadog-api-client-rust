// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// The attributes of a notebook cell response. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
/// `log_stream`. [More information on each graph visualization type.](<https://docs.datadoghq.com/dashboards/widgets/>)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookCellResponseAttributes {
    NotebookMarkdownCellAttributes(crate::datadogV1::model::NotebookMarkdownCellAttributes),
    NotebookTimeseriesCellAttributes(crate::datadogV1::model::NotebookTimeseriesCellAttributes),
    NotebookToplistCellAttributes(crate::datadogV1::model::NotebookToplistCellAttributes),
    NotebookHeatMapCellAttributes(crate::datadogV1::model::NotebookHeatMapCellAttributes),
    NotebookDistributionCellAttributes(crate::datadogV1::model::NotebookDistributionCellAttributes),
    NotebookLogStreamCellAttributes(crate::datadogV1::model::NotebookLogStreamCellAttributes),
}
