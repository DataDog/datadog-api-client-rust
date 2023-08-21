// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListRequestPage {
    /// List following results with a cursor provided in the previous query.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: String,
    /// Maximum number of spans in the response.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i32,
}

