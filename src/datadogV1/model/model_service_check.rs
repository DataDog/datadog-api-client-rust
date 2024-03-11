// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object containing service check and status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceCheck {
    /// The check.
    #[serde(rename = "check")]
    pub check: String,
    /// The host name correlated with the check.
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// Message containing check status.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The status of a service check. Set to `0` for OK, `1` for warning, `2` for critical, and `3` for unknown.
    #[serde(rename = "status")]
    pub status: crate::datadogV1::model::ServiceCheckStatus,
    /// Tags related to a check.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Time of check.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceCheck {
    pub fn new(
        check: String,
        host_name: String,
        status: crate::datadogV1::model::ServiceCheckStatus,
        tags: Vec<String>,
    ) -> ServiceCheck {
        ServiceCheck {
            check,
            host_name,
            message: None,
            status,
            tags,
            timestamp: None,
            _unparsed: false,
        }
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn timestamp(&mut self, value: i64) -> &mut Self {
        self.timestamp = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceCheck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceCheckVisitor;
        impl<'a> Visitor<'a> for ServiceCheckVisitor {
            type Value = ServiceCheck;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut check: Option<String> = None;
                let mut host_name: Option<String> = None;
                let mut message: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::ServiceCheckStatus> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "check" => {
                            check = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_name" => {
                            host_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::ServiceCheckStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let check = check.ok_or_else(|| M::Error::missing_field("check"))?;
                let host_name = host_name.ok_or_else(|| M::Error::missing_field("host_name"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = ServiceCheck {
                    check,
                    host_name,
                    message,
                    status,
                    tags,
                    timestamp,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceCheckVisitor)
    }
}
