// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsResponseLinks {
    /// Link for the next set of results. Note that the request can also be made using the
POST endpoint.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: String,
}

