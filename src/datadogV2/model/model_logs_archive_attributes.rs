// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes associated with the archive.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveAttributes {
    /// An archive's destination.
    #[serde(rename = "destination")]
    pub destination: Option<crate::datadogV2::model::LogsArchiveDestination>,
    /// To store the tags in the archive, set the value "true".
    /// If it is set to "false", the tags will be deleted when the logs are sent to the archive.
    #[serde(rename = "include_tags")]
    pub include_tags: Option<bool>,
    /// The archive name.
    #[serde(rename = "name")]
    pub name: String,
    /// The archive query/filter. Logs matching this query are included in the archive.
    #[serde(rename = "query")]
    pub query: String,
    /// Maximum scan size for rehydration from this archive.
    #[serde(
        rename = "rehydration_max_scan_size_in_gb",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rehydration_max_scan_size_in_gb: Option<Option<i64>>,
    /// An array of tags to add to rehydrated logs from an archive.
    #[serde(rename = "rehydration_tags")]
    pub rehydration_tags: Option<Vec<String>>,
    /// The state of the archive.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::LogsArchiveState>,
}

impl LogsArchiveAttributes {
    pub fn new(
        destination: Option<crate::datadogV2::model::LogsArchiveDestination>,
        name: String,
        query: String,
    ) -> LogsArchiveAttributes {
        LogsArchiveAttributes {
            destination,
            include_tags: None,
            name,
            query,
            rehydration_max_scan_size_in_gb: None,
            rehydration_tags: None,
            state: None,
        }
    }

    pub fn include_tags(&mut self, value: bool) -> &mut Self {
        self.include_tags = Some(value);
        self
    }

    pub fn rehydration_max_scan_size_in_gb(&mut self, value: Option<i64>) -> &mut Self {
        self.rehydration_max_scan_size_in_gb = Some(value);
        self
    }

    pub fn rehydration_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.rehydration_tags = Some(value);
        self
    }

    pub fn state(&mut self, value: crate::datadogV2::model::LogsArchiveState) -> &mut Self {
        self.state = Some(value);
        self
    }
}
