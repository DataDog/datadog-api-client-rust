// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing details about your Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestDetails {
    /// Configuration object for a Synthetic test.
    #[serde(rename = "config")]
    pub config: Option<crate::datadogV1::model::SyntheticsTestConfig>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV1::model::Creator>,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<String>>,
    /// Notification message associated with the test.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the test.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Object describing the extra options for a Synthetic test.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV1::model::SyntheticsTestOptions>,
    /// The test public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// For browser test, the steps of the test.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>>,
    /// The subtype of the Synthetic API test, `http`, `ssl`, `tcp`,
    /// `dns`, `icmp`, `udp`, `websocket`, `grpc` or `multi`.
    #[serde(rename = "subtype")]
    pub subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsTestDetailsType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestDetails {
    pub fn new() -> SyntheticsTestDetails {
        SyntheticsTestDetails {
            config: None,
            creator: None,
            locations: None,
            message: None,
            monitor_id: None,
            name: None,
            options: None,
            public_id: None,
            status: None,
            steps: None,
            subtype: None,
            tags: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn config(mut self, value: crate::datadogV1::model::SyntheticsTestConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<String>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn monitor_id(mut self, value: i64) -> Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV1::model::SyntheticsTestOptions) -> Self {
        self.options = Some(value);
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

    pub fn subtype(mut self, value: crate::datadogV1::model::SyntheticsTestDetailsSubType) -> Self {
        self.subtype = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::SyntheticsTestDetailsType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SyntheticsTestDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestDetailsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestDetailsVisitor {
            type Value = SyntheticsTestDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<crate::datadogV1::model::SyntheticsTestConfig> = None;
                let mut creator: Option<crate::datadogV1::model::Creator> = None;
                let mut locations: Option<Vec<String>> = None;
                let mut message: Option<String> = None;
                let mut monitor_id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV1::model::SyntheticsTestOptions> = None;
                let mut public_id: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus> = None;
                let mut steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>> = None;
                let mut subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsTestDetailsType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            if v.is_null() {
                                continue;
                            }
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "subtype" => {
                            if v.is_null() {
                                continue;
                            }
                            subtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subtype) = subtype {
                                match _subtype {
                                    crate::datadogV1::model::SyntheticsTestDetailsSubType::UnparsedObject(_subtype) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsTestDetailsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestDetails {
                    config,
                    creator,
                    locations,
                    message,
                    monitor_id,
                    name,
                    options,
                    public_id,
                    status,
                    steps,
                    subtype,
                    tags,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestDetailsVisitor)
    }
}
