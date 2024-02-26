// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The new security filter properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterUpdateData {
    /// The security filters properties to be updated.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityFilterUpdateAttributes,
    /// The type of the resource. The value should always be `security_filters`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityFilterType,
}

impl SecurityFilterUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityFilterUpdateAttributes,
        type_: crate::datadogV2::model::SecurityFilterType,
    ) -> SecurityFilterUpdateData {
        SecurityFilterUpdateData { attributes, type_ }
    }
}
