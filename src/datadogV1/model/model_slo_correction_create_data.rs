// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The data object associated with the SLO correction to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionCreateData {
    /// The attribute object associated with the SLO correction to be created.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SLOCorrectionCreateRequestAttributes>,
    /// SLO correction resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOCorrectionType,
}

impl SLOCorrectionCreateData {
    pub fn new(type_: crate::datadogV1::model::SLOCorrectionType) -> SLOCorrectionCreateData {
        SLOCorrectionCreateData {
            attributes: None,
            type_,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SLOCorrectionCreateRequestAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }
}
