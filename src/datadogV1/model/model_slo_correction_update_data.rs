// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The data object associated with the SLO correction to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionUpdateData {
    /// The attribute object associated with the SLO correction to be updated.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SLOCorrectionUpdateRequestAttributes>,
    /// SLO correction resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOCorrectionType>,
}

impl SLOCorrectionUpdateData {
    pub fn new() -> SLOCorrectionUpdateData {
        SLOCorrectionUpdateData {
            attributes: None,
            type_: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV1::model::SLOCorrectionUpdateRequestAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV1::model::SLOCorrectionType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for SLOCorrectionUpdateData {
    fn default() -> Self {
        Self::new()
    }
}
