// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes of a notebook `markdown` cell.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookMarkdownCellAttributes {
    /// Text in a notebook is formatted with [Markdown](<https://daringfireball.net/projects/markdown/>), which enables the use of headings, subheadings, links, images, lists, and code blocks.
    #[serde(rename = "definition")]
    pub definition: Box<crate::datadogV1::model::NotebookMarkdownCellDefinition>,
}

impl NotebookMarkdownCellAttributes {
    pub fn new(
        definition: Box<crate::datadogV1::model::NotebookMarkdownCellDefinition>,
    ) -> NotebookMarkdownCellAttributes {
        NotebookMarkdownCellAttributes { definition }
    }
}
