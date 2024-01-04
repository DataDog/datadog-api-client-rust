// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single finding with with message and resource configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailedFinding {
    /// The JSON:API attributes of the detailed finding.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::DetailedFindingAttributes>>,
    /// The unique ID for this finding.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for findings that have the message and resource configuration.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DetailedFindingType>,
}

impl DetailedFinding {
    pub fn new() -> DetailedFinding {
        DetailedFinding {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
