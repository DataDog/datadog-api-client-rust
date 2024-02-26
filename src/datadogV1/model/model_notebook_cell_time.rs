// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Timeframe for the notebook cell. When 'null', the notebook global time is used.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookCellTime {
    NotebookRelativeTime(Box<crate::datadogV1::model::NotebookRelativeTime>),
    NotebookAbsoluteTime(Box<crate::datadogV1::model::NotebookAbsoluteTime>),
}
