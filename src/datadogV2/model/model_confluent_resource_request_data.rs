// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceRequestData {
    /// Attributes object for updating a Confluent resource.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ConfluentResourceRequestAttributes>,
    /// The ID associated with a Confluent resource.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for this request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfluentResourceType,
}

impl ConfluentResourceRequestData {
    /// JSON:API request for updating a Confluent resource.
    pub fn new(
        attributes: crate::datadogV2::model::ConfluentResourceRequestAttributes,
        id: String,
        type_: crate::datadogV2::model::ConfluentResourceType,
    ) -> ConfluentResourceRequestData {
        ConfluentResourceRequestData {
            attributes: Box::new(attributes),
            id: id,
            type_: type_,
        }
    }
}
