// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountCreateRequestData {
    /// Attributes associated with the account creation request.
    #[serde(rename = "attributes")]
    pub attributes: ConfluentAccountCreateRequestAttributes,
    /// The JSON:API type for this API. Should always be `confluent-cloud-accounts`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ConfluentAccountType,
}

