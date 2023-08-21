// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteSLOResponseData {
    /// An array of of SLO IDs that can be safely deleted.
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Vec<String>,
}

