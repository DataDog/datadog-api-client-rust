// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data from search SLO response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseData {
    /// Attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SearchSLOResponseDataAttributes>,
    /// Type of service level objective result.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl SearchSLOResponseData {
    pub fn new() -> SearchSLOResponseData {
        SearchSLOResponseData {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV1::model::SearchSLOResponseDataAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SearchSLOResponseData {
    fn default() -> Self {
        Self::new()
    }
}
