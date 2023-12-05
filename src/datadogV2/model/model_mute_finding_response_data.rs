// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object containing the updated finding.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingResponseData {
    /// The JSON:API attributes of the finding.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MuteFindingResponseAttributes>>,
    /// The unique ID for this finding.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for findings.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::FindingType>,
}

impl MuteFindingResponseData {
    pub fn new() -> MuteFindingResponseData {
        MuteFindingResponseData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}