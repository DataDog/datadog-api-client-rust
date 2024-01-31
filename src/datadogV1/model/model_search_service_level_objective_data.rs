// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective ID and attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchServiceLevelObjectiveData {
    /// A service level objective object includes a service level indicator, thresholds
    /// for one or more timeframes, and metadata (`name`, `description`, and `tags`).
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SearchServiceLevelObjectiveAttributes>,
    /// A unique identifier for the service level objective object.
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the object, must be `slo`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl SearchServiceLevelObjectiveData {
    pub fn new() -> SearchServiceLevelObjectiveData {
        SearchServiceLevelObjectiveData {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV1::model::SearchServiceLevelObjectiveAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for SearchServiceLevelObjectiveData {
    fn default() -> Self {
        Self::new()
    }
}
