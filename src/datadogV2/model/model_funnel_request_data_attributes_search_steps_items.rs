// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelRequestDataAttributesSearchStepsItems {
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    #[serde(rename = "step_filter")]
    pub step_filter: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelRequestDataAttributesSearchStepsItems {
    pub fn new() -> FunnelRequestDataAttributesSearchStepsItems {
        FunnelRequestDataAttributesSearchStepsItems {
            facet: None,
            step_filter: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }

    pub fn step_filter(mut self, value: String) -> Self {
        self.step_filter = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
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

impl Default for FunnelRequestDataAttributesSearchStepsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FunnelRequestDataAttributesSearchStepsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelRequestDataAttributesSearchStepsItemsVisitor;
        impl<'a> Visitor<'a> for FunnelRequestDataAttributesSearchStepsItemsVisitor {
            type Value = FunnelRequestDataAttributesSearchStepsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut step_filter: Option<String> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            if v.is_null() {
                                continue;
                            }
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "step_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            step_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FunnelRequestDataAttributesSearchStepsItems {
                    facet,
                    step_filter,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelRequestDataAttributesSearchStepsItemsVisitor)
    }
}
