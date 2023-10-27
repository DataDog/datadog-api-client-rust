// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountCreateRequestData {
    /// Attributes associated with the account creation request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::ConfluentAccountCreateRequestAttributes>,
    /// The JSON:API type for this API. Should always be `confluent-cloud-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfluentAccountType,
}

impl ConfluentAccountCreateRequestData {
    /// The data body for adding a Confluent account.
    pub fn new(
        attributes: crate::datadogV2::model::ConfluentAccountCreateRequestAttributes,
        type_: crate::datadogV2::model::ConfluentAccountType,
    ) -> ConfluentAccountCreateRequestData {
        ConfluentAccountCreateRequestData {
            attributes: Box::new(attributes),
            type_,
        }
    }
}
