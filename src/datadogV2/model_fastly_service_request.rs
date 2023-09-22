// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceRequest {
    /// Data object for Fastly service requests.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::FastlyServiceData>,
}

impl FastlyServiceRequest {
    /// Payload schema for Fastly service requests.
    pub fn new(data: crate::datadogV2::FastlyServiceData) -> FastlyServiceRequest {
        FastlyServiceRequest {
            data: Box::new(data),
        }
    }
}
