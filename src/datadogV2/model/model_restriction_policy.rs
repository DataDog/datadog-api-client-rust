// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Restriction policy object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictionPolicy {
    /// Restriction policy attributes.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::RestrictionPolicyAttributes,
    /// The identifier, always equivalent to the value specified in the `resource_id` path parameter.
    #[serde(rename = "id")]
    pub id: String,
    /// Restriction policy type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RestrictionPolicyType,
}

impl RestrictionPolicy {
    pub fn new(
        attributes: crate::datadogV2::model::RestrictionPolicyAttributes,
        id: String,
        type_: crate::datadogV2::model::RestrictionPolicyType,
    ) -> RestrictionPolicy {
        RestrictionPolicy {
            attributes,
            id,
            type_,
        }
    }
}
