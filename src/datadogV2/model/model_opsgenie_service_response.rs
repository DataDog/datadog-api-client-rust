// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceResponse {
    /// Opsgenie service data from a response.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::OpsgenieServiceResponseData>,
}

impl OpsgenieServiceResponse {
    /// Response of an Opsgenie service.
    pub fn new(data: crate::datadogV2::model::OpsgenieServiceResponseData) -> OpsgenieServiceResponse {
        OpsgenieServiceResponse { data: Box::new(data) }
    }
}
