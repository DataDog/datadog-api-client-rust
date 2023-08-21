// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionUpdateData {
    /// The attribute object associated with the SLO correction to be updated.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SLOCorrectionUpdateRequestAttributes,
    /// SLO correction resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SLOCorrectionType,
}

