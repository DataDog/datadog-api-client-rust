// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a notebook in get all response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponseDataAttributes {
    /// Attributes of user object returned by the API.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::NotebookAuthor>,
    /// List of cells to display in the notebook.
    #[serde(rename = "cells")]
    pub cells: Option<Vec<crate::datadogV1::model::NotebookCellResponse>>,
    /// UTC time stamp for when the notebook was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Metadata associated with the notebook.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::NotebookMetadata>,
    /// UTC time stamp for when the notebook was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// The name of the notebook.
    #[serde(rename = "name")]
    pub name: String,
    /// Publication status of the notebook. For now, always "published".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::NotebookStatus>,
    /// Notebook global timeframe.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::NotebookGlobalTime>,
}

impl NotebooksResponseDataAttributes {
    pub fn new(name: String) -> NotebooksResponseDataAttributes {
        NotebooksResponseDataAttributes {
            author: None,
            cells: None,
            created: None,
            metadata: None,
            modified: None,
            name,
            status: None,
            time: None,
        }
    }

    pub fn author(&mut self, value: crate::datadogV1::model::NotebookAuthor) -> &mut Self {
        self.author = Some(value);
        self
    }

    pub fn cells(
        &mut self,
        value: Vec<crate::datadogV1::model::NotebookCellResponse>,
    ) -> &mut Self {
        self.cells = Some(value);
        self
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn metadata(&mut self, value: crate::datadogV1::model::NotebookMetadata) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV1::model::NotebookStatus) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn time(&mut self, value: crate::datadogV1::model::NotebookGlobalTime) -> &mut Self {
        self.time = Some(value);
        self
    }
}
