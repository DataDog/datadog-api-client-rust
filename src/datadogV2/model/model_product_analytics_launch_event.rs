// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An event to instrument for the launch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchEvent {
    /// What the event records.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// IDs of the launch KPIs that the event measures.
    #[serde(rename = "kpis")]
    pub kpis: Option<Vec<String>>,
    /// Name of the event, unique within the tracking plan.
    #[serde(rename = "name")]
    pub name: String,
    /// Properties that the event carries.
    #[serde(rename = "properties")]
    pub properties: Option<Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventProperty>>,
    /// What causes the event to be sent.
    #[serde(rename = "trigger")]
    pub trigger: Option<String>,
    /// Views where the event is sent. Each value must be unique and not blank.
    #[serde(rename = "views")]
    pub views: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchEvent {
    pub fn new(name: String) -> ProductAnalyticsLaunchEvent {
        ProductAnalyticsLaunchEvent {
            description: None,
            kpis: None,
            name,
            properties: None,
            trigger: None,
            views: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn kpis(mut self, value: Vec<String>) -> Self {
        self.kpis = Some(value);
        self
    }

    pub fn properties(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventProperty>,
    ) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn trigger(mut self, value: String) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn views(mut self, value: Vec<String>) -> Self {
        self.views = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchEventVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchEventVisitor {
            type Value = ProductAnalyticsLaunchEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut kpis: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut properties: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsLaunchEventProperty>,
                > = None;
                let mut trigger: Option<String> = None;
                let mut views: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kpis" => {
                            if v.is_null() {
                                continue;
                            }
                            kpis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            if v.is_null() {
                                continue;
                            }
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger" => {
                            if v.is_null() {
                                continue;
                            }
                            trigger = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "views" => {
                            if v.is_null() {
                                continue;
                            }
                            views = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ProductAnalyticsLaunchEvent {
                    description,
                    kpis,
                    name,
                    properties,
                    trigger,
                    views,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchEventVisitor)
    }
}
