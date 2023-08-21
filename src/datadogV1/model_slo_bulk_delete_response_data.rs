// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOBulkDeleteResponseData {
    /// An array of service level objective object IDs that indicates
which objects that were completely deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Vec<String>,
    /// An array of service level objective object IDs that indicates
which objects that were modified (objects for which at least one
threshold was deleted, but that were not completely deleted).
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Vec<String>,
}

