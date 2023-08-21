// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteMonitorResponseData {
    /// An array of of Monitor IDs that can be safely deleted.
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Vec<i64>,
}

