// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Text in a notebook is formatted with [Markdown](https://daringfireball.net/projects/markdown/), which enables the use of headings, subheadings, links, images, lists, and code blocks.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookMarkdownCellDefinition {
    /// The markdown content.
    #[serde(rename = "text")]
    pub text: String,
    /// Type of the markdown cell.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookMarkdownCellDefinitionType,
}

impl NotebookMarkdownCellDefinition {
    pub fn new(
        text: String,
        type_: crate::datadogV1::model::NotebookMarkdownCellDefinitionType,
    ) -> NotebookMarkdownCellDefinition {
        NotebookMarkdownCellDefinition { text, type_ }
    }
}
