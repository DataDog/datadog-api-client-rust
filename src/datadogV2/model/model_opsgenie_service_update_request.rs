// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateRequest {
    /// Opsgenie service for an update request.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::OpsgenieServiceUpdateData>,
}

impl OpsgenieServiceUpdateRequest {
    /// Update request for an Opsgenie service.
    pub fn new(data: crate::datadogV2::model::OpsgenieServiceUpdateData) -> OpsgenieServiceUpdateRequest {
        OpsgenieServiceUpdateRequest { data: Box::new(data) }
    }
}
