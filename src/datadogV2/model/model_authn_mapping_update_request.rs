// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request to update an AuthN Mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingUpdateRequest {
    /// Data for updating an AuthN Mapping.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::AuthNMappingUpdateData,
}

impl AuthNMappingUpdateRequest {
    pub fn new(data: crate::datadogV2::model::AuthNMappingUpdateData) -> AuthNMappingUpdateRequest {
        AuthNMappingUpdateRequest { data }
    }
}