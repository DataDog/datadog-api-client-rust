// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The data for a notebook create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookCreateData {
    /// The data attributes of a notebook.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV1::model::NotebookCreateDataAttributes,
    /// Type of the Notebook resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookResourceType,
}

impl NotebookCreateData {
    pub fn new(
        attributes: crate::datadogV1::model::NotebookCreateDataAttributes,
        type_: crate::datadogV1::model::NotebookResourceType,
    ) -> NotebookCreateData {
        NotebookCreateData { attributes, type_ }
    }
}
