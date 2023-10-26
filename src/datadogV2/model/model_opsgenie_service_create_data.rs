// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateData {
    /// The Opsgenie service attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::OpsgenieServiceCreateAttributes>,
    /// Opsgenie service resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OpsgenieServiceType,
}

impl OpsgenieServiceCreateData {
    /// Opsgenie service data for a create request.
    pub fn new(
        attributes: crate::datadogV2::model::OpsgenieServiceCreateAttributes,
        type_: crate::datadogV2::model::OpsgenieServiceType,
    ) -> OpsgenieServiceCreateData {
        OpsgenieServiceCreateData {
            attributes: Box::new(attributes),
            type_: type_,
        }
    }
}
