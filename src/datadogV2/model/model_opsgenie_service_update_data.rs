// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Opsgenie service for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateData {
    /// The Opsgenie service attributes for an update request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::OpsgenieServiceUpdateAttributes,
    /// The ID of the Opsgenie service.
    #[serde(rename = "id")]
    pub id: String,
    /// Opsgenie service resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OpsgenieServiceType,
}

impl OpsgenieServiceUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::OpsgenieServiceUpdateAttributes,
        id: String,
        type_: crate::datadogV2::model::OpsgenieServiceType,
    ) -> OpsgenieServiceUpdateData {
        OpsgenieServiceUpdateData {
            attributes,
            id,
            type_,
        }
    }
}
