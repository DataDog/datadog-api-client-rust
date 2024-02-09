// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data returned by an incident search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseData {
    /// Attributes returned by an incident search.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentSearchResponseAttributes>,
    /// Incident search result type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::IncidentSearchResultsType>,
}

impl IncidentSearchResponseData {
    pub fn new() -> IncidentSearchResponseData {
        IncidentSearchResponseData {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::IncidentSearchResponseAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::IncidentSearchResultsType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for IncidentSearchResponseData {
    fn default() -> Self {
        Self::new()
    }
}
