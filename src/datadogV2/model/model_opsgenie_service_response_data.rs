// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceResponseData {
    /// The attributes from an Opsgenie service response.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::OpsgenieServiceResponseAttributes>,
    /// The ID of the Opsgenie service.
    #[serde(rename = "id")]
    pub id: String,
    /// Opsgenie service resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OpsgenieServiceType,
}

impl OpsgenieServiceResponseData {
    /// Opsgenie service data from a response.
    pub fn new(
        attributes: crate::datadogV2::model::OpsgenieServiceResponseAttributes,
        id: String,
        type_: crate::datadogV2::model::OpsgenieServiceType,
    ) -> OpsgenieServiceResponseData {
        OpsgenieServiceResponseData {
            attributes: Box::new(attributes),
            id,
            type_,
        }
    }
}
