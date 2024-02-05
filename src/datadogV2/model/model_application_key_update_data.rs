// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object used to update an application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyUpdateData {
    /// Attributes used to update an application Key.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ApplicationKeyUpdateAttributes,
    /// ID of the application key.
    #[serde(rename = "id")]
    pub id: String,
    /// Application Keys resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ApplicationKeysType,
}

impl ApplicationKeyUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::ApplicationKeyUpdateAttributes,
        id: String,
        type_: crate::datadogV2::model::ApplicationKeysType,
    ) -> ApplicationKeyUpdateData {
        ApplicationKeyUpdateData {
            attributes,
            id,
            type_,
        }
    }
}
