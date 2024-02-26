// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object of a list of SLO corrections.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrection {
    /// The attribute object associated with the SLO correction.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SLOCorrectionResponseAttributes>,
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

    pub fn attributes(
        &mut self,
        value: crate::datadogV1::model::SLOCorrectionResponseAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV1::model::SLOCorrectionType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SLOCorrection {
    fn default() -> Self {
        Self::new()
    }
}
