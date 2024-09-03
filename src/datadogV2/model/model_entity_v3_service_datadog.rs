// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Datadog product integrations for the service entity
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3ServiceDatadog {
    /// Schema for mapping source code locations to an entity
    #[serde(rename = "codeLocations")]
    pub code_locations: Option<Vec<crate::datadogV2::model::EntityV3DatadogCodeLocationItem>>,
    /// Events associations
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV2::model::EntityV3DatadogEventItem>>,
    /// Logs association
    #[serde(rename = "logs")]
    pub logs: Option<Vec<crate::datadogV2::model::EntityV3DatadogLogItem>>,
    /// Performance stats association
    #[serde(rename = "performanceData")]
    pub performance_data: Option<crate::datadogV2::model::EntityV3DatadogPerformance>,
    /// CI Pipelines association
    #[serde(rename = "pipelines")]
    pub pipelines: Option<crate::datadogV2::model::EntityV3DatadogPipelines>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3ServiceDatadog {
    pub fn new() -> EntityV3ServiceDatadog {
        EntityV3ServiceDatadog {
            code_locations: None,
            events: None,
            logs: None,
            performance_data: None,
            pipelines: None,
            _unparsed: false,
        }
    }

    pub fn code_locations(
        mut self,
        value: Vec<crate::datadogV2::model::EntityV3DatadogCodeLocationItem>,
    ) -> Self {
        self.code_locations = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<crate::datadogV2::model::EntityV3DatadogEventItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn logs(mut self, value: Vec<crate::datadogV2::model::EntityV3DatadogLogItem>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn performance_data(
        mut self,
        value: crate::datadogV2::model::EntityV3DatadogPerformance,
    ) -> Self {
        self.performance_data = Some(value);
        self
    }

    pub fn pipelines(mut self, value: crate::datadogV2::model::EntityV3DatadogPipelines) -> Self {
        self.pipelines = Some(value);
        self
    }
}

impl Default for EntityV3ServiceDatadog {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3ServiceDatadog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3ServiceDatadogVisitor;
        impl<'a> Visitor<'a> for EntityV3ServiceDatadogVisitor {
            type Value = EntityV3ServiceDatadog;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_locations: Option<
                    Vec<crate::datadogV2::model::EntityV3DatadogCodeLocationItem>,
                > = None;
                let mut events: Option<Vec<crate::datadogV2::model::EntityV3DatadogEventItem>> =
                    None;
                let mut logs: Option<Vec<crate::datadogV2::model::EntityV3DatadogLogItem>> = None;
                let mut performance_data: Option<
                    crate::datadogV2::model::EntityV3DatadogPerformance,
                > = None;
                let mut pipelines: Option<crate::datadogV2::model::EntityV3DatadogPipelines> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "codeLocations" => {
                            if v.is_null() {
                                continue;
                            }
                            code_locations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs" => {
                            if v.is_null() {
                                continue;
                            }
                            logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "performanceData" => {
                            if v.is_null() {
                                continue;
                            }
                            performance_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipelines" => {
                            if v.is_null() {
                                continue;
                            }
                            pipelines = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3ServiceDatadog {
                    code_locations,
                    events,
                    logs,
                    performance_data,
                    pipelines,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3ServiceDatadogVisitor)
    }
}
