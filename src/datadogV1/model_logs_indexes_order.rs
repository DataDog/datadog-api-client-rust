// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexesOrder {
    /// Array of strings identifying by their name(s) the index(es) of your organization.
Logs are tested against the query filter of each index one by one, following the order of the array.
Logs are eventually stored in the first matching index.
    #[serde(rename = "index_names", skip_serializing_if = "Option::is_none")]
    pub index_names: Vec<String>,
}

