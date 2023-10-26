// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateRequest {
    /// Opsgenie service data for a create request.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::OpsgenieServiceCreateData>,
}

impl OpsgenieServiceCreateRequest {
    /// Create request for an Opsgenie service.
    pub fn new(data: crate::datadogV2::model::OpsgenieServiceCreateData) -> OpsgenieServiceCreateRequest {
        OpsgenieServiceCreateRequest { data: Box::new(data) }
    }
}
