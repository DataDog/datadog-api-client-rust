// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object of a list of SLO corrections.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrection {
    /// The attribute object associated with the SLO correction.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV1::model::SLOCorrectionResponseAttributes>>,
    /// The ID of the SLO correction.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// SLO correction resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOCorrectionType>,
}

impl SLOCorrection {
    pub fn new() -> SLOCorrection {
        SLOCorrection {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
