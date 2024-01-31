// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object containing the ID of the request that was updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkMuteFindingsResponseData {
    /// UUID used to identify the request
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for findings.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::FindingType>,
}

impl BulkMuteFindingsResponseData {
    pub fn new() -> BulkMuteFindingsResponseData {
        BulkMuteFindingsResponseData {
            id: None,
            type_: None,
        }
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::FindingType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for BulkMuteFindingsResponseData {
    fn default() -> Self {
        Self::new()
    }
}
