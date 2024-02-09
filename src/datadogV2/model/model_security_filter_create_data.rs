// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single security filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterCreateData {
    /// Object containing the attributes of the security filter to be created.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityFilterCreateAttributes,
    /// The type of the resource. The value should always be `security_filters`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityFilterType,
}

impl SecurityFilterCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityFilterCreateAttributes,
        type_: crate::datadogV2::model::SecurityFilterType,
    ) -> SecurityFilterCreateData {
        SecurityFilterCreateData { attributes, type_ }
    }
}
