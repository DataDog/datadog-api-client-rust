// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a notebook `log_stream` cell.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookLogStreamCellAttributes {
    /// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
    #[serde(rename = "definition")]
    pub definition: Box<crate::datadogV1::model::LogStreamWidgetDefinition>,
    /// The size of the graph.
    #[serde(rename = "graph_size")]
    pub graph_size: Option<crate::datadogV1::model::NotebookGraphSize>,
    /// Timeframe for the notebook cell. When 'null', the notebook global time is used.
    #[serde(rename = "time", default, with = "::serde_with::rust::double_option")]
    pub time: Option<Option<Box<crate::datadogV1::model::NotebookCellTime>>>,
}

impl NotebookLogStreamCellAttributes {
    pub fn new(
        definition: Box<crate::datadogV1::model::LogStreamWidgetDefinition>,
    ) -> NotebookLogStreamCellAttributes {
        NotebookLogStreamCellAttributes {
            definition,
            graph_size: None,
            time: None,
        }
    }
}
