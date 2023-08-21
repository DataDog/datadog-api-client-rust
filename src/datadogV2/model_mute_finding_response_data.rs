// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingResponseData {
    /// The JSON:API attributes of the finding.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: MuteFindingResponseAttributes,
    /// The unique ID for this finding.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The JSON:API type for findings.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: FindingType,
}

