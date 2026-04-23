// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Device information for the test result (browser and mobile tests).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultDevice {
    /// Browser information for the device used to run the test.
    #[serde(rename = "browser")]
    pub browser: Option<crate::datadogV2::model::SyntheticsTestResultDeviceBrowser>,
    /// Device identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Device name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Platform information for the device used to run the test.
    #[serde(rename = "platform")]
    pub platform: Option<crate::datadogV2::model::SyntheticsTestResultDevicePlatform>,
    /// Screen resolution of the device used to run the test.
    #[serde(rename = "resolution")]
    pub resolution: Option<crate::datadogV2::model::SyntheticsTestResultDeviceResolution>,
    /// Device type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultDevice {
    pub fn new() -> SyntheticsTestResultDevice {
        SyntheticsTestResultDevice {
            browser: None,
            id: None,
            name: None,
            platform: None,
            resolution: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn browser(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultDeviceBrowser,
    ) -> Self {
        self.browser = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn platform(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultDevicePlatform,
    ) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn resolution(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultDeviceResolution,
    ) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestResultDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultDeviceVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultDeviceVisitor {
            type Value = SyntheticsTestResultDevice;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut browser: Option<
                    crate::datadogV2::model::SyntheticsTestResultDeviceBrowser,
                > = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut platform: Option<
                    crate::datadogV2::model::SyntheticsTestResultDevicePlatform,
                > = None;
                let mut resolution: Option<
                    crate::datadogV2::model::SyntheticsTestResultDeviceResolution,
                > = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "browser" => {
                            if v.is_null() {
                                continue;
                            }
                            browser = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "platform" => {
                            if v.is_null() {
                                continue;
                            }
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolution" => {
                            if v.is_null() {
                                continue;
                            }
                            resolution = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultDevice {
                    browser,
                    id,
                    name,
                    platform,
                    resolution,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultDeviceVisitor)
    }
}
