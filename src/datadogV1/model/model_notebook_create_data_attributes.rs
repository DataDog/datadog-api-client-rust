// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The data attributes of a notebook.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookCreateDataAttributes {
    /// List of cells to display in the notebook.
    #[serde(rename = "cells")]
    pub cells: Vec<crate::datadogV1::model::NotebookCellCreateRequest>,
    /// Metadata associated with the notebook.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::NotebookMetadata>,
    /// The name of the notebook.
    #[serde(rename = "name")]
    pub name: String,
    /// Publication status of the notebook. For now, always "published".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::NotebookStatus>,
    /// Notebook global timeframe.
    #[serde(rename = "time")]
    pub time: crate::datadogV1::model::NotebookGlobalTime,
}

impl NotebookCreateDataAttributes {
    pub fn new(
        cells: Vec<crate::datadogV1::model::NotebookCellCreateRequest>,
        name: String,
        time: crate::datadogV1::model::NotebookGlobalTime,
    ) -> NotebookCreateDataAttributes {
        NotebookCreateDataAttributes {
            cells,
            metadata: None,
            name,
            status: None,
            time,
        }
    }

    pub fn with_metadata(&mut self, value: crate::datadogV1::model::NotebookMetadata) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn with_status(&mut self, value: crate::datadogV1::model::NotebookStatus) -> &mut Self {
        self.status = Some(value);
        self
    }
}
