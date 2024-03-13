// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Updating a notebook can either insert new cell(s) or update existing cell(s) by including the cell `id`.
/// To delete existing cell(s), simply omit it from the list of cells.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NotebookUpdateCell {
    NotebookCellCreateRequest(Box<crate::datadogV1::model::NotebookCellCreateRequest>),
    NotebookCellUpdateRequest(Box<crate::datadogV1::model::NotebookCellUpdateRequest>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for NotebookUpdateCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookCellCreateRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookUpdateCell::NotebookCellCreateRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookCellUpdateRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookUpdateCell::NotebookCellUpdateRequest(_v));
            }
        }

        return Ok(NotebookUpdateCell::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
