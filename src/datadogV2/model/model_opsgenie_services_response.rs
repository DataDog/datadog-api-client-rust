// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServicesResponse {
    /// An array of Opsgenie services.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::OpsgenieServiceResponseData>,
}

impl OpsgenieServicesResponse {
    /// Response with a list of Opsgenie services.
    pub fn new(data: Vec<crate::datadogV2::model::OpsgenieServiceResponseData>) -> OpsgenieServicesResponse {
        OpsgenieServicesResponse { data: data }
    }
}
