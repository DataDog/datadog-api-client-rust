// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The definition of an archive.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveDefinition {
    /// The attributes associated with the archive.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::LogsArchiveAttributes>>,
    /// The archive ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the resource. The value should always be archives.
    #[serde(rename = "type")]
    pub type_: String,
}

impl LogsArchiveDefinition {
    pub fn new(type_: String) -> LogsArchiveDefinition {
        LogsArchiveDefinition {
            attributes: None,
            id: None,
            type_,
        }
    }
}
