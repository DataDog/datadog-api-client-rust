// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the device used to perform the Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn is_mobile(mut self, value: bool) -> Self {
        self.is_mobile = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDeviceVisitor;
        impl<'a> Visitor<'a> for SyntheticsDeviceVisitor {
            type Value = SyntheticsDevice;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<i64> = None;
                let mut id: Option<crate::datadogV1::model::SyntheticsDeviceID> = None;
                let mut is_mobile: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut width: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "height" => {
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _id) = id {
                                match _id {
                                    crate::datadogV1::model::SyntheticsDeviceID::UnparsedObject(
                                        _id,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "isMobile" => {
                            if v.is_null() {
                                continue;
                            }
                            is_mobile = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let height = height.ok_or_else(|| M::Error::missing_field("height"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let width = width.ok_or_else(|| M::Error::missing_field("width"))?;

                let content = SyntheticsDevice {
                    height,
                    id,
                    is_mobile,
                    name,
                    width,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDeviceVisitor)
    }
}
