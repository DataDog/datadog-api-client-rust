// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSTagFilterDeleteRequest {
    /// The unique identifier of your AWS account.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: String,
    /// The namespace associated with the tag filter entry.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: AWSNamespace,
}

