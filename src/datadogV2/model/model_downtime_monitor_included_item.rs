// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about the monitor identified by the downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeMonitorIncludedItem {
    /// Attributes of the monitor identified by the downtime.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::DowntimeMonitorIncludedAttributes>,
    /// ID of the monitor identified by the downtime.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Monitor resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DowntimeIncludedMonitorType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeMonitorIncludedItem {
    pub fn new() -> DowntimeMonitorIncludedItem {
        DowntimeMonitorIncludedItem {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::DowntimeMonitorIncludedAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::DowntimeIncludedMonitorType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for DowntimeMonitorIncludedItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeMonitorIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeMonitorIncludedItemVisitor;
        impl<'a> Visitor<'a> for DowntimeMonitorIncludedItemVisitor {
            type Value = DowntimeMonitorIncludedItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::DowntimeMonitorIncludedAttributes,
                > = None;
                let mut id: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::DowntimeIncludedMonitorType> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DowntimeIncludedMonitorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeMonitorIncludedItem {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeMonitorIncludedItemVisitor)
    }
}
