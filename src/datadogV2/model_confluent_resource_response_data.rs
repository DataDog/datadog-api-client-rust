// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceResponseData {
    /// Model representation of a Confluent Cloud resource.
    #[serde(rename = "attributes")]
    pub attributes: ConfluentResourceResponseAttributes,
    /// The ID associated with the Confluent resource.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The JSON:API type for this request.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ConfluentResourceType,
}

