// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about a single test run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTriggerCITestRunResult {
    /// The device ID.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDeviceID>,
    /// The location ID of the test run.
    #[serde(rename = "location")]
    pub location: Option<i64>,
    /// The public ID of the Synthetic test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// ID of the result.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTriggerCITestRunResult {
    pub fn new() -> SyntheticsTriggerCITestRunResult {
        SyntheticsTriggerCITestRunResult {
            device: None,
            location: None,
            public_id: None,
            result_id: None,
            _unparsed: false,
        }
    }

    pub fn device(mut self, value: crate::datadogV1::model::SyntheticsDeviceID) -> Self {
        self.device = Some(value);
        self
    }

    pub fn location(mut self, value: i64) -> Self {
        self.location = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn result_id(mut self, value: String) -> Self {
        self.result_id = Some(value);
        self
    }
}

impl Default for SyntheticsTriggerCITestRunResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTriggerCITestRunResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTriggerCITestRunResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsTriggerCITestRunResultVisitor {
            type Value = SyntheticsTriggerCITestRunResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut device: Option<crate::datadogV1::model::SyntheticsDeviceID> = None;
                let mut location: Option<i64> = None;
                let mut public_id: Option<String> = None;
                let mut result_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _device) = device {
                                match _device {
                                    crate::datadogV1::model::SyntheticsDeviceID::UnparsedObject(
                                        _device,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_id" => {
                            if v.is_null() {
                                continue;
                            }
                            result_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTriggerCITestRunResult {
                    device,
                    location,
                    public_id,
                    result_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTriggerCITestRunResultVisitor)
    }
}
