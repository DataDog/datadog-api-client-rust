// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestrictionPolicy {
    /// Restriction policy attributes.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::RestrictionPolicyAttributes>,
    /// The identifier, always equivalent to the value specified in the `resource_id` path parameter.
    #[serde(rename = "id")]
    pub id: String,
    /// Restriction policy type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RestrictionPolicyType,
}

impl RestrictionPolicy {
    /// Restriction policy object.
    pub fn new(
        attributes: crate::datadogV2::model::RestrictionPolicyAttributes,
        id: String,
        type_: crate::datadogV2::model::RestrictionPolicyType,
    ) -> RestrictionPolicy {
        RestrictionPolicy {
            attributes: Box::new(attributes),
            id: id,
            type_: type_,
        }
    }
}
