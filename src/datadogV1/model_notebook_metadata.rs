// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookMetadata {
    /// Whether or not the notebook is a template.
    #[serde(rename = "is_template", skip_serializing_if = "Option::is_none")]
    pub is_template: bool,
    /// Whether or not the notebook takes snapshot image backups of the notebook's fixed-time graphs.
    #[serde(rename = "take_snapshots", skip_serializing_if = "Option::is_none")]
    pub take_snapshots: bool,
    /// Metadata type of the notebook.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub type_: NullableNotebookMetadataType,
}

