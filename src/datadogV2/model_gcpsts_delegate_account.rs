// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccount {
    /// Your delegate account attributes.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: GCPSTSDelegateAccountAttributes,
    /// The ID of the delegate service account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The type of account.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: GCPSTSDelegateAccountType,
}

