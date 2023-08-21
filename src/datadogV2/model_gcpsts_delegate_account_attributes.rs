// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccountAttributes {
    /// Your organization's Datadog principal email address.
    #[serde(rename = "delegate_account_email", skip_serializing_if = "Option::is_none")]
    pub delegate_account_email: String,
}

