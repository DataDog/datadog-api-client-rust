// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata associated with the notebook.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookMetadata {
    /// Whether or not the notebook is a template.
    #[serde(rename = "is_template")]
    pub is_template: Option<bool>,
    /// Whether or not the notebook takes snapshot image backups of the notebook's fixed-time graphs.
    #[serde(rename = "take_snapshots")]
    pub take_snapshots: Option<bool>,
    /// Metadata type of the notebook.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option")]
    pub type_: Option<Option<crate::datadogV1::model::NotebookMetadataType>>,
}

impl NotebookMetadata {
    pub fn new() -> NotebookMetadata {
        NotebookMetadata {
            is_template: None,
            take_snapshots: None,
            type_: None,
        }
    }
}