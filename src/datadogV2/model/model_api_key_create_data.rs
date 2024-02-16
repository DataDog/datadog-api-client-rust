// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object used to create an API key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeyCreateData {
    /// Attributes used to create an API Key.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::APIKeyCreateAttributes,
    /// API Keys resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::APIKeysType,
}

impl APIKeyCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::APIKeyCreateAttributes,
        type_: crate::datadogV2::model::APIKeysType,
    ) -> APIKeyCreateData {
        APIKeyCreateData { attributes, type_ }
    }
}
