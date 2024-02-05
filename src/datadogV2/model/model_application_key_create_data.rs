// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object used to create an application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyCreateData {
    /// Attributes used to create an application Key.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ApplicationKeyCreateAttributes,
    /// Application Keys resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ApplicationKeysType,
}

impl ApplicationKeyCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::ApplicationKeyCreateAttributes,
        type_: crate::datadogV2::model::ApplicationKeysType,
    ) -> ApplicationKeyCreateData {
        ApplicationKeyCreateData { attributes, type_ }
    }
}
