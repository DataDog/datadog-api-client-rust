// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a notebook `toplist` cell.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookToplistCellAttributes {
    /// The top list visualization enables you to display a list of Tag value like hostname or service with the most or least of any metric value, such as highest consumers of CPU, hosts with the least disk space, etc.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::ToplistWidgetDefinition,
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

impl NotebookToplistCellAttributes {
    pub fn new(
        definition: crate::datadogV1::model::ToplistWidgetDefinition,
    ) -> NotebookToplistCellAttributes {
        NotebookToplistCellAttributes {
            definition,
            graph_size: None,
            split_by: None,
            time: None,
        }
    }

    pub fn graph_size(mut self, value: crate::datadogV1::model::NotebookGraphSize) -> Self {
        self.graph_size = Some(value);
        self
    }

    pub fn split_by(mut self, value: crate::datadogV1::model::NotebookSplitBy) -> Self {
        self.split_by = Some(value);
        self
    }

    pub fn time(mut self, value: Option<crate::datadogV1::model::NotebookCellTime>) -> Self {
        self.time = Some(value);
        self
    }
}
