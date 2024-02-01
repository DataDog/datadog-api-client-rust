// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The definition of an archive.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveCreateRequestDefinition {
    /// The attributes associated with the archive.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::LogsArchiveCreateRequestAttributes>,
    /// The type of the resource. The value should always be archives.
    #[serde(rename = "type")]
    pub type_: String,
}

impl LogsArchiveCreateRequestDefinition {
    pub fn new(type_: String) -> LogsArchiveCreateRequestDefinition {
        LogsArchiveCreateRequestDefinition {
            attributes: None,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::LogsArchiveCreateRequestAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}
