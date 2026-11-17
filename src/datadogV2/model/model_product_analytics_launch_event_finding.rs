// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Code analysis result for an event of the tracking plan.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchEventFinding {
    /// Name of the event in the tracking plan.
    #[serde(rename = "name")]
    pub name: String,
    /// Results for the properties of the event. Only allowed when the event is `found`.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchPropertyFinding>>,
    /// Whether the inspected code sends the event, or attaches the property to the event.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ProductAnalyticsLaunchCodeStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchEventFinding {
    pub fn new(
        name: String,
        status: crate::datadogV2::model::ProductAnalyticsLaunchCodeStatus,
    ) -> ProductAnalyticsLaunchEventFinding {
        ProductAnalyticsLaunchEventFinding {
            name,
            properties: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn properties(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsLaunchPropertyFinding>,
    ) -> Self {
        self.properties = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchEventFinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchEventFindingVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchEventFindingVisitor {
            type Value = ProductAnalyticsLaunchEventFinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut properties: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsLaunchPropertyFinding>,
                > = None;
                let mut status: Option<crate::datadogV2::model::ProductAnalyticsLaunchCodeStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ProductAnalyticsLaunchCodeStatus::UnparsedObject(_status) => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = ProductAnalyticsLaunchEventFinding {
                    name,
                    properties,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchEventFindingVisitor)
    }
}
