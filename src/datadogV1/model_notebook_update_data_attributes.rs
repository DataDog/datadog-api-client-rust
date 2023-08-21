// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookUpdateDataAttributes {
    /// List of cells to display in the notebook.
    #[serde(rename = "cells", skip_serializing_if = "Option::is_none")]
    pub cells: Vec<NotebookUpdateCell>,
    /// Metadata associated with the notebook.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: NotebookMetadata,
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

