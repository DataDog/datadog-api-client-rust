// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The RUM definition for a DEM journey.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemJourneyRum {
    /// An optional RUM query filter applied to the entire journey.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// List of RUM journey steps.
    #[serde(rename = "rum_steps")]
    pub rum_steps: Vec<crate::datadogV2::model::DemRumStep>,
    /// List of variants associated with a DEM journey.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<crate::datadogV2::model::DemVariant>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemJourneyRum {
    pub fn new(rum_steps: Vec<crate::datadogV2::model::DemRumStep>) -> DemJourneyRum {
        DemJourneyRum {
            filter: None,
            rum_steps,
            variants: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<crate::datadogV2::model::DemVariant>) -> Self {
        self.variants = Some(value);
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

impl<'de> Deserialize<'de> for DemJourneyRum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemJourneyRumVisitor;
        impl<'a> Visitor<'a> for DemJourneyRumVisitor {
            type Value = DemJourneyRum;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut rum_steps: Option<Vec<crate::datadogV2::model::DemRumStep>> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::DemVariant>> = None;
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
                        "rum_steps" => {
                            rum_steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            if v.is_null() {
                                continue;
                            }
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rum_steps = rum_steps.ok_or_else(|| M::Error::missing_field("rum_steps"))?;

                let content = DemJourneyRum {
                    filter,
                    rum_steps,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemJourneyRumVisitor)
    }
}
