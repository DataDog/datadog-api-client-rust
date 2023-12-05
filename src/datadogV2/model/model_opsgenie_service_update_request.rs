// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update request for an Opsgenie service.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateRequest {
    /// Opsgenie service for an update request.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::OpsgenieServiceUpdateData>,
}

impl OpsgenieServiceUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::OpsgenieServiceUpdateData,
    ) -> OpsgenieServiceUpdateRequest {
        OpsgenieServiceUpdateRequest {
            data: Box::new(data),
        }
    }
}