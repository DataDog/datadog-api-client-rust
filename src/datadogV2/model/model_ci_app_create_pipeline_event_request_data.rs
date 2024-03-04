// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data of the pipeline event to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCreatePipelineEventRequestData {
    /// Attributes of the pipeline event to create.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributes>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CIAppCreatePipelineEventRequestDataType>,
}

impl CIAppCreatePipelineEventRequestData {
    pub fn new() -> CIAppCreatePipelineEventRequestData {
        CIAppCreatePipelineEventRequestData {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::CIAppCreatePipelineEventRequestAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::CIAppCreatePipelineEventRequestDataType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for CIAppCreatePipelineEventRequestData {
    fn default() -> Self {
        Self::new()
    }
}
