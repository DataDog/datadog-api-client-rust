// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the device used to perform the Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDevice {
    /// Screen height of the device.
    #[serde(rename = "height")]
    pub height: i64,
    /// The device ID.
    #[serde(rename = "id")]
    pub id: crate::datadogV1::model::SyntheticsDeviceID,
    /// Whether or not the device is a mobile.
    #[serde(rename = "isMobile")]
    pub is_mobile: Option<bool>,
    /// The device name.
    #[serde(rename = "name")]
    pub name: String,
    /// Screen width of the device.
    #[serde(rename = "width")]
    pub width: i64,
}

impl SyntheticsDevice {
    pub fn new(
        height: i64,
        id: crate::datadogV1::model::SyntheticsDeviceID,
        name: String,
        width: i64,
    ) -> SyntheticsDevice {
        SyntheticsDevice {
            height,
            id,
            is_mobile: None,
            name,
            width,
        }
    }

    pub fn is_mobile(mut self, value: bool) -> Self {
        self.is_mobile = Some(value);
        self
    }
}
