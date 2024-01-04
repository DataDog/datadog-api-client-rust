// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object for Fastly service requests.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceData {
    /// Attributes object for Fastly service requests.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::FastlyServiceAttributes>>,
    /// The ID of the Fastly service.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for this API. Should always be `fastly-services`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::FastlyServiceType,
}

impl FastlyServiceData {
    pub fn new(id: String, type_: crate::datadogV2::model::FastlyServiceType) -> FastlyServiceData {
        FastlyServiceData {
            attributes: None,
            id,
            type_,
        }
    }
}
