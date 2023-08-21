// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponseData {
    /// The attributes of a notebook in get all response.
    #[serde(rename = "attributes")]
    pub attributes: NotebooksResponseDataAttributes,
    /// Unique notebook ID, assigned when you create the notebook.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// Type of the Notebook resource.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: NotebookResourceType,
}

