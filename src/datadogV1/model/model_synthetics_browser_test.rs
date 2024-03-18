// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing details about a Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTest {
    /// Configuration object for a Synthetic browser test.
    #[serde(rename = "config")]
    pub config: crate::datadogV1::model::SyntheticsBrowserTestConfig,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Notification message associated with the test. Message can either be text or an empty string.
    #[serde(rename = "message")]
    pub message: String,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the test.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the extra options for a Synthetic test.
    #[serde(rename = "options")]
    pub options: crate::datadogV1::model::SyntheticsTestOptions,
    /// The public ID of the test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// Array of steps for the test.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, `browser`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserTestType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTest {
    pub fn new(
        config: crate::datadogV1::model::SyntheticsBrowserTestConfig,
        locations: Vec<String>,
        message: String,
        name: String,
        options: crate::datadogV1::model::SyntheticsTestOptions,
        type_: crate::datadogV1::model::SyntheticsBrowserTestType,
    ) -> SyntheticsBrowserTest {
        SyntheticsBrowserTest {
            config,
            locations,
            message,
            monitor_id: None,
            name,
            options,
            public_id: None,
            status: None,
            steps: None,
            tags: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn monitor_id(mut self, value: i64) -> Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::SyntheticsTestPauseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<crate::datadogV1::model::SyntheticsStep>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestVisitor {
            type Value = SyntheticsBrowserTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<crate::datadogV1::model::SyntheticsBrowserTestConfig> = None;
                let mut locations: Option<Vec<String>> = None;
                let mut message: Option<String> = None;
                let mut monitor_id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV1::model::SyntheticsTestOptions> = None;
                let mut public_id: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus> = None;
                let mut steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBrowserTestType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::SyntheticsTestPauseStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBrowserTestType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;
                let locations = locations.ok_or_else(|| M::Error::missing_field("locations"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBrowserTest {
                    config,
                    locations,
                    message,
                    monitor_id,
                    name,
                    options,
                    public_id,
                    status,
                    steps,
                    tags,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestVisitor)
    }
}
