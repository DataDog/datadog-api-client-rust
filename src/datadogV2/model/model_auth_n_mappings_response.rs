// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Array of AuthN Mappings response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingsResponse {
    /// Array of returned AuthN Mappings.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::AuthNMapping>>,
    /// Included data in the AuthN Mapping response.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::AuthNMappingIncluded>>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ResponseMetaAttributes>,
}

impl AuthNMappingsResponse {
    pub fn new() -> AuthNMappingsResponse {
        AuthNMappingsResponse {
            data: None,
            included: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::AuthNMapping>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::AuthNMappingIncluded>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::ResponseMetaAttributes) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for AuthNMappingsResponse {
    fn default() -> Self {
        Self::new()
    }
}
