// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountResponseData {
    /// The attributes of a Confluent account.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ConfluentAccountResponseAttributes>,
    /// A randomly generated ID associated with a Confluent account.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for this API. Should always be `confluent-cloud-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfluentAccountType,
}

impl ConfluentAccountResponseData {
    /// An API key and API secret pair that represents a Confluent account.
    pub fn new(
        attributes: crate::datadogV2::model::ConfluentAccountResponseAttributes,
        id: String,
        type_: crate::datadogV2::model::ConfluentAccountType,
    ) -> ConfluentAccountResponseData {
        ConfluentAccountResponseData {
            attributes: Box::new(attributes),
            id: id,
            type_: type_,
        }
    }
}
