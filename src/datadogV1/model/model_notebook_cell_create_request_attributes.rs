// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// The attributes of a notebook cell in create cell request. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
/// `log_stream`. [More information on each graph visualization type.](https://docs.datadoghq.com/dashboards/widgets/)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookCellCreateRequestAttributes {
    NotebookMarkdownCellAttributes(Box<crate::datadogV1::model::NotebookMarkdownCellAttributes>),
    NotebookTimeseriesCellAttributes(
        Box<crate::datadogV1::model::NotebookTimeseriesCellAttributes>,
    ),
    NotebookToplistCellAttributes(Box<crate::datadogV1::model::NotebookToplistCellAttributes>),
    NotebookHeatMapCellAttributes(Box<crate::datadogV1::model::NotebookHeatMapCellAttributes>),
    NotebookDistributionCellAttributes(
        Box<crate::datadogV1::model::NotebookDistributionCellAttributes>,
    ),
    NotebookLogStreamCellAttributes(Box<crate::datadogV1::model::NotebookLogStreamCellAttributes>),
}
