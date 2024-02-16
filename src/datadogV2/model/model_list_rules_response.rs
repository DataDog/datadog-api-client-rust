// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard rules response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRulesResponse {
    /// Array of rule details.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ListRulesResponseDataItem>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ListRulesResponseLinks>,
}

impl ListRulesResponse {
    pub fn new() -> ListRulesResponse {
        ListRulesResponse {
            data: None,
            links: None,
        }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::ListRulesResponseDataItem>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV2::model::ListRulesResponseLinks) -> &mut Self {
        self.links = Some(value);
        self
    }
}

impl Default for ListRulesResponse {
    fn default() -> Self {
        Self::new()
    }
}
