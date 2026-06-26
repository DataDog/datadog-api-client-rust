// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceLimitAttributes {
    /// A description of what the limit measures.
    #[serde(rename = "description")]
    pub description: String,
    /// The human-readable name of the limit.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The type of limit, such as a rate limit or a usage limit.
    #[serde(rename = "limit_type")]
    pub limit_type: String,
    /// The Datadog product the limit belongs to.
    #[serde(rename = "product")]
    pub product: String,
    /// A metric query used to compute usage against a limit.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::GovernanceLimitQuery,
    /// The query execution context used to visualize a limit and its usage.
    #[serde(rename = "query_config")]
    pub query_config: crate::datadogV2::model::GovernanceLimitQueryConfig,
    /// The time range over which usage against the limit is measured.
    #[serde(rename = "time_range")]
    pub time_range: String,
    /// The number of times usage has reached the limit within the measured time range.
    #[serde(rename = "times_hit_limit")]
    pub times_hit_limit: f64,
    /// The unit in which the limit and its usage are measured.
    #[serde(rename = "unit_name")]
    pub unit_name: String,
    /// The current usage status of the limit relative to its threshold.
    #[serde(rename = "usage_status")]
    pub usage_status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceLimitAttributes {
    pub fn new(
        description: String,
        display_name: String,
        limit_type: String,
        product: String,
        query: crate::datadogV2::model::GovernanceLimitQuery,
        query_config: crate::datadogV2::model::GovernanceLimitQueryConfig,
        time_range: String,
        times_hit_limit: f64,
        unit_name: String,
        usage_status: String,
    ) -> GovernanceLimitAttributes {
        GovernanceLimitAttributes {
            description,
            display_name,
            limit_type,
            product,
            query,
            query_config,
            time_range,
            times_hit_limit,
            unit_name,
            usage_status,
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

impl<'de> Deserialize<'de> for GovernanceLimitAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceLimitAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceLimitAttributesVisitor {
            type Value = GovernanceLimitAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut limit_type: Option<String> = None;
                let mut product: Option<String> = None;
                let mut query: Option<crate::datadogV2::model::GovernanceLimitQuery> = None;
                let mut query_config: Option<crate::datadogV2::model::GovernanceLimitQueryConfig> =
                    None;
                let mut time_range: Option<String> = None;
                let mut times_hit_limit: Option<f64> = None;
                let mut unit_name: Option<String> = None;
                let mut usage_status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit_type" => {
                            limit_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_config" => {
                            query_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_range" => {
                            time_range = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times_hit_limit" => {
                            times_hit_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit_name" => {
                            unit_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_status" => {
                            usage_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let limit_type = limit_type.ok_or_else(|| M::Error::missing_field("limit_type"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let query_config =
                    query_config.ok_or_else(|| M::Error::missing_field("query_config"))?;
                let time_range = time_range.ok_or_else(|| M::Error::missing_field("time_range"))?;
                let times_hit_limit =
                    times_hit_limit.ok_or_else(|| M::Error::missing_field("times_hit_limit"))?;
                let unit_name = unit_name.ok_or_else(|| M::Error::missing_field("unit_name"))?;
                let usage_status =
                    usage_status.ok_or_else(|| M::Error::missing_field("usage_status"))?;

                let content = GovernanceLimitAttributes {
                    description,
                    display_name,
                    limit_type,
                    product,
                    query,
                    query_config,
                    time_range,
                    times_hit_limit,
                    unit_name,
                    usage_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceLimitAttributesVisitor)
    }
}
