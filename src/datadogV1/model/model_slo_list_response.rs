// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A response with one or more service level objective.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::ServiceLevelObjective>>,
    /// An array of error messages. Each endpoint documents how/whether this field is
    /// used.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
    /// The metadata object containing additional information about the list of SLOs.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SLOListResponseMetadata>,
}

impl SLOListResponse {
    pub fn new() -> SLOListResponse {
        SLOListResponse {
            data: None,
            errors: None,
            metadata: None,
        }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV1::model::ServiceLevelObjective>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn errors(&mut self, value: Vec<String>) -> &mut Self {
        self.errors = Some(value);
        self
    }

    pub fn metadata(
        &mut self,
        value: crate::datadogV1::model::SLOListResponseMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }
}

impl Default for SLOListResponse {
    fn default() -> Self {
        Self::new()
    }
}
