// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DetailedFinding {
    /// The JSON:API attributes of the detailed finding.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::DetailedFindingAttributes>>,
    /// The unique ID for this finding.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The JSON:API type for findings that have the message and resource configuration.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<crate::datadogV2::model::DetailedFindingType>,
}

impl DetailedFinding {
    /// A single finding with with message and resource configuration.
    pub fn new() -> DetailedFinding {
        DetailedFinding {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
