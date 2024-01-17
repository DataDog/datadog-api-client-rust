// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The description of a notebook cell response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookCellResponse {
    /// The attributes of a notebook cell response. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
    /// `log_stream`. [More information on each graph visualization type.](https://docs.datadoghq.com/dashboards/widgets/)
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV1::model::NotebookCellResponseAttributes>,
    /// Notebook cell ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Type of the Notebook Cell resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookCellResourceType,
}

impl NotebookCellResponse {
    pub fn new(
        attributes: Box<crate::datadogV1::model::NotebookCellResponseAttributes>,
        id: String,
        type_: crate::datadogV1::model::NotebookCellResourceType,
    ) -> NotebookCellResponse {
        NotebookCellResponse {
            attributes,
            id,
            type_,
        }
    }
}
