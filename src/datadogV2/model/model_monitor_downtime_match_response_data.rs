// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A downtime match.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorDowntimeMatchResponseData {
    /// Downtime match details.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MonitorDowntimeMatchResponseAttributes>,
    /// The downtime ID.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option")]
    pub id: Option<Option<String>>,
    /// Monitor Downtime Match resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MonitorDowntimeMatchResourceType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorDowntimeMatchResponseData {
    pub fn new() -> MonitorDowntimeMatchResponseData {
        MonitorDowntimeMatchResponseData {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::MonitorDowntimeMatchResponseAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: Option<String>) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::MonitorDowntimeMatchResourceType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorDowntimeMatchResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorDowntimeMatchResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorDowntimeMatchResponseDataVisitor;
        impl<'a> Visitor<'a> for MonitorDowntimeMatchResponseDataVisitor {
            type Value = MonitorDowntimeMatchResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::MonitorDowntimeMatchResponseAttributes,
                > = None;
                let mut id: Option<Option<String>> = None;
                let mut type_: Option<crate::datadogV2::model::MonitorDowntimeMatchResourceType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MonitorDowntimeMatchResourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = MonitorDowntimeMatchResponseData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorDowntimeMatchResponseDataVisitor)
    }
}
