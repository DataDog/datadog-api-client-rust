// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request used to update an application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyUpdateRequest {
    /// Object used to update an application key.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::ApplicationKeyUpdateData,
}

impl ApplicationKeyUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::ApplicationKeyUpdateData,
    ) -> ApplicationKeyUpdateRequest {
        ApplicationKeyUpdateRequest { data }
    }
}
