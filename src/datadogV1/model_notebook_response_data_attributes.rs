// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookResponseDataAttributes {
    /// Attributes of user object returned by the API.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: NotebookAuthor,
    /// List of cells to display in the notebook.
    #[serde(rename = "cells", skip_serializing_if = "Option::is_none")]
    pub cells: Vec<NotebookCellResponse>,
    /// UTC time stamp for when the notebook was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// Metadata associated with the notebook.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: NotebookMetadata,
    /// UTC time stamp for when the notebook was last modified.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// The name of the notebook.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Publication status of the notebook. For now, always "published".
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: NotebookStatus,
    /// Notebook global timeframe.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: NotebookGlobalTime,
}

