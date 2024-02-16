// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The data for a notebook in get all response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponseData {
    /// The attributes of a notebook in get all response.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV1::model::NotebooksResponseDataAttributes,
    /// Unique notebook ID, assigned when you create the notebook.
    #[serde(rename = "id")]
    pub id: i64,
    /// Type of the Notebook resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookResourceType,
}

impl NotebooksResponseData {
    pub fn new(
        attributes: crate::datadogV1::model::NotebooksResponseDataAttributes,
        id: i64,
        type_: crate::datadogV1::model::NotebookResourceType,
    ) -> NotebooksResponseData {
        NotebooksResponseData {
            attributes,
            id,
            type_,
        }
    }
}
