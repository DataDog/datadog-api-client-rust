// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Confluent Cloud resource data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceResponseData {
    /// Model representation of a Confluent Cloud resource.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ConfluentResourceResponseAttributes>,
    /// The ID associated with the Confluent resource.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for this request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfluentResourceType,
}

impl ConfluentResourceResponseData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::ConfluentResourceResponseAttributes>,
        id: String,
        type_: crate::datadogV2::model::ConfluentResourceType,
    ) -> ConfluentResourceResponseData {
        ConfluentResourceResponseData {
            attributes,
            id,
            type_,
        }
    }
}
