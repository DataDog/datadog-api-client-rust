// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A variant (sub-funnel) of a DEM journey with its own steps.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemVariant {
    /// An optional RUM query filter to scope this variant.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// The unique identifier of the variant.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the variant.
    #[serde(rename = "name")]
    pub name: String,
    /// List of RUM journey steps.
    #[serde(rename = "rum_steps")]
    pub rum_steps: Vec<crate::datadogV2::model::DemRumStep>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemVariant {
    pub fn new(name: String, rum_steps: Vec<crate::datadogV2::model::DemRumStep>) -> DemVariant {
        DemVariant {
            filter: None,
            id: None,
            name,
            rum_steps,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for DemVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemVariantVisitor;
        impl<'a> Visitor<'a> for DemVariantVisitor {
            type Value = DemVariant;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut rum_steps: Option<Vec<crate::datadogV2::model::DemRumStep>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_steps" => {
                            rum_steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rum_steps = rum_steps.ok_or_else(|| M::Error::missing_field("rum_steps"))?;

                let content = DemVariant {
                    filter,
                    id,
                    name,
                    rum_steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemVariantVisitor)
    }
}
