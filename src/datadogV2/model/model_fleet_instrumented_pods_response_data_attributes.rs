// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the instrumented pods response containing the list of pod groups.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetInstrumentedPodsResponseDataAttributes {
    /// Array of instrumented pod groups in the cluster.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV2::model::FleetInstrumentedPodGroupAttributes>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetInstrumentedPodsResponseDataAttributes {
    pub fn new() -> FleetInstrumentedPodsResponseDataAttributes {
        FleetInstrumentedPodsResponseDataAttributes {
            groups: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn groups(
        mut self,
        value: Vec<crate::datadogV2::model::FleetInstrumentedPodGroupAttributes>,
    ) -> Self {
        self.groups = Some(value);
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

impl Default for FleetInstrumentedPodsResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetInstrumentedPodsResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetInstrumentedPodsResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for FleetInstrumentedPodsResponseDataAttributesVisitor {
            type Value = FleetInstrumentedPodsResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut groups: Option<
                    Vec<crate::datadogV2::model::FleetInstrumentedPodGroupAttributes>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetInstrumentedPodsResponseDataAttributes {
                    groups,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetInstrumentedPodsResponseDataAttributesVisitor)
    }
}
