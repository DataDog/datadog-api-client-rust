// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingRequestData {
    /// The mute properties to be updated.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::MuteFindingRequestAttributes>,
    /// The unique ID for this finding.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for findings.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::FindingType,
}

impl MuteFindingRequestData {
    /// Data object containing the new mute properties of the finding.
    pub fn new(
        attributes: crate::datadogV2::model::MuteFindingRequestAttributes,
        id: String,
        type_: crate::datadogV2::model::FindingType,
    ) -> MuteFindingRequestData {
        MuteFindingRequestData {
            attributes: Box::new(attributes),
            id: id,
            type_: type_,
        }
    }
}
