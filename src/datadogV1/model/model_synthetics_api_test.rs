// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing details about a Synthetic API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPITest {
    /// Configuration object for a Synthetic API test.
    #[serde(rename = "config")]
    pub config: crate::datadogV1::model::SyntheticsAPITestConfig,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Notification message associated with the test.
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
    /// The public ID for the test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// The subtype of the Synthetic API test, `http`, `ssl`, `tcp`,
    /// `dns`, `icmp`, `udp`, `websocket`, `grpc` or `multi`.
    #[serde(rename = "subtype")]
    pub subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, `api`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAPITestType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPITest {
    pub fn new(
        config: crate::datadogV1::model::SyntheticsAPITestConfig,
        locations: Vec<String>,
        message: String,
        name: String,
        options: crate::datadogV1::model::SyntheticsTestOptions,
        type_: crate::datadogV1::model::SyntheticsAPITestType,
    ) -> SyntheticsAPITest {
        SyntheticsAPITest {
            config,
            locations,
            message,
            monitor_id: None,
            name,
            options,
            public_id: None,
            status: None,
            subtype: None,
            tags: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn subtype(mut self, value: crate::datadogV1::model::SyntheticsTestDetailsSubType) -> Self {
        self.subtype = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsAPITest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestVisitor {
            type Value = SyntheticsAPITest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<crate::datadogV1::model::SyntheticsAPITestConfig> = None;
                let mut locations: Option<Vec<String>> = None;
                let mut message: Option<String> = None;
                let mut monitor_id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV1::model::SyntheticsTestOptions> = None;
                let mut public_id: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus> = None;
                let mut subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsAPITestType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsAPITestType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;
                let locations = locations.ok_or_else(|| M::Error::missing_field("locations"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsAPITest {
                    config,
                    locations,
                    message,
                    monitor_id,
                    name,
                    options,
                    public_id,
                    status,
                    subtype,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestVisitor)
    }
}
