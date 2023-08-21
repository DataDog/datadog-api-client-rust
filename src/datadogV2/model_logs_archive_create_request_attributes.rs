// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveCreateRequestAttributes {
    /// An archive's destination.
    #[serde(rename = "destination")]
    pub destination: LogsArchiveCreateRequestDestination,
    /// To store the tags in the archive, set the value "true".
If it is set to "false", the tags will be deleted when the logs are sent to the archive.
    #[serde(rename = "include_tags", skip_serializing_if = "Option::is_none")]
    pub include_tags: bool,
    /// The archive name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The archive query/filter. Logs matching this query are included in the archive.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Maximum scan size for rehydration from this archive.
    #[serde(rename = "rehydration_max_scan_size_in_gb", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub rehydration_max_scan_size_in_gb: Option<Int64>,
    /// An array of tags to add to rehydrated logs from an archive.
    #[serde(rename = "rehydration_tags", skip_serializing_if = "Option::is_none")]
    pub rehydration_tags: Vec<String>,
}

