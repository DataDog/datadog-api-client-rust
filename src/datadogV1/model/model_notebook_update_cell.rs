// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Updating a notebook can either insert new cell(s) or update existing cell(s) by including the cell `id`.
/// To delete existing cell(s), simply omit it from the list of cells.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookUpdateCell {
    NotebookCellCreateRequest(Box<crate::datadogV1::model::NotebookCellCreateRequest>),
    NotebookCellUpdateRequest(Box<crate::datadogV1::model::NotebookCellUpdateRequest>),
}
