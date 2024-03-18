// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Projected Cost attributes data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectedCostAttributes {
    /// List of charges data reported for the requested month.
    #[serde(rename = "charges")]
    pub charges: Option<Vec<crate::datadogV2::model::ChargebackBreakdown>>,
    /// The month requested.
    #[serde(rename = "date")]
    pub date: Option<DateTime<Utc>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The total projected cost of products for the month.
    #[serde(rename = "projected_total_cost")]
    pub projected_total_cost: Option<f64>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectedCostAttributes {
    pub fn new() -> ProjectedCostAttributes {
        ProjectedCostAttributes {
            charges: None,
            date: None,
            org_name: None,
            projected_total_cost: None,
            public_id: None,
            region: None,
            _unparsed: false,
        }
    }

    pub fn charges(mut self, value: Vec<crate::datadogV2::model::ChargebackBreakdown>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn date(mut self, value: DateTime<Utc>) -> Self {
        self.date = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn projected_total_cost(mut self, value: f64) -> Self {
        self.projected_total_cost = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }
}

impl Default for ProjectedCostAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectedCostAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectedCostAttributesVisitor;
        impl<'a> Visitor<'a> for ProjectedCostAttributesVisitor {
            type Value = ProjectedCostAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut charges: Option<Vec<crate::datadogV2::model::ChargebackBreakdown>> = None;
                let mut date: Option<DateTime<Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut projected_total_cost: Option<f64> = None;
                let mut public_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "charges" => {
                            if v.is_null() {
                                continue;
                            }
                            charges = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date" => {
                            if v.is_null() {
                                continue;
                            }
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projected_total_cost" => {
                            if v.is_null() {
                                continue;
                            }
                            projected_total_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ProjectedCostAttributes {
                    charges,
                    date,
                    org_name,
                    projected_total_cost,
                    public_id,
                    region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectedCostAttributesVisitor)
    }
}
