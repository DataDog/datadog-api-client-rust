// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Unit metadata for a numeric metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsUnit {
    /// The unit family (for example, percentage or money).
    #[serde(rename = "family")]
    pub family: String,
    /// The unit identifier.
    #[serde(rename = "id")]
    pub id: i64,
    /// The unit name (for example, percent or dollar).
    #[serde(rename = "name")]
    pub name: String,
    /// The plural form of the unit name.
    #[serde(rename = "plural")]
    pub plural: String,
    /// The scale factor for the unit.
    #[serde(rename = "scale_factor")]
    pub scale_factor: f64,
    /// The abbreviated unit name (for example, % or $).
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsUnit {
    pub fn new(
        family: String,
        id: i64,
        name: String,
        plural: String,
        scale_factor: f64,
        short_name: String,
    ) -> CommitmentsUnit {
        CommitmentsUnit {
            family,
            id,
            name,
            plural,
            scale_factor,
            short_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CommitmentsUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsUnitVisitor;
        impl<'a> Visitor<'a> for CommitmentsUnitVisitor {
            type Value = CommitmentsUnit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut family: Option<String> = None;
                let mut id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut plural: Option<String> = None;
                let mut scale_factor: Option<f64> = None;
                let mut short_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "family" => {
                            family = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "plural" => {
                            plural = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scale_factor" => {
                            scale_factor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let family = family.ok_or_else(|| M::Error::missing_field("family"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let plural = plural.ok_or_else(|| M::Error::missing_field("plural"))?;
                let scale_factor =
                    scale_factor.ok_or_else(|| M::Error::missing_field("scale_factor"))?;
                let short_name = short_name.ok_or_else(|| M::Error::missing_field("short_name"))?;

                let content = CommitmentsUnit {
                    family,
                    id,
                    name,
                    plural,
                    scale_factor,
                    short_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsUnitVisitor)
    }
}
