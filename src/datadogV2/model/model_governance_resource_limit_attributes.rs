// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance resource limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceResourceLimitAttributes {
    /// The current limit configured for the resource.
    #[serde(rename = "current_limit")]
    pub current_limit: f64,
    /// The current value of the resource.
    #[serde(rename = "current_value")]
    pub current_value: f64,
    /// A link to the Datadog page where the resource and its limit can be managed.
    #[serde(rename = "deep_link")]
    pub deep_link: String,
    /// The default current value used when the resource value cannot be computed from the query.
    #[serde(rename = "default_current_value")]
    pub default_current_value: f64,
    /// The default limit value used when the limit cannot be computed from the query.
    #[serde(rename = "default_limit_value")]
    pub default_limit_value: f64,
    /// A description of what the resource limit measures.
    #[serde(rename = "description")]
    pub description: String,
    /// The human-readable name of the resource limit.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A metric query used to compute usage against a limit.
    #[serde(rename = "limit_query")]
    pub limit_query: crate::datadogV2::model::GovernanceLimitQuery,
    /// The Datadog product the resource limit belongs to.
    #[serde(rename = "product")]
    pub product: String,
    /// A metric query used to compute usage against a limit.
    #[serde(rename = "query")]
    pub query: crate::datadogV2::model::GovernanceLimitQuery,
    /// The query execution context used to visualize a limit and its usage.
    #[serde(rename = "query_config")]
    pub query_config: crate::datadogV2::model::GovernanceLimitQueryConfig,
    /// The time range over which the resource value is measured.
    #[serde(rename = "time_range")]
    pub time_range: String,
    /// The unit in which the resource value and limit are measured.
    #[serde(rename = "unit_name")]
    pub unit_name: String,
    /// The current usage status of the resource relative to its limit.
    #[serde(rename = "usage_status")]
    pub usage_status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceResourceLimitAttributes {
    pub fn new(
        current_limit: f64,
        current_value: f64,
        deep_link: String,
        default_current_value: f64,
        default_limit_value: f64,
        description: String,
        display_name: String,
        limit_query: crate::datadogV2::model::GovernanceLimitQuery,
        product: String,
        query: crate::datadogV2::model::GovernanceLimitQuery,
        query_config: crate::datadogV2::model::GovernanceLimitQueryConfig,
        time_range: String,
        unit_name: String,
        usage_status: String,
    ) -> GovernanceResourceLimitAttributes {
        GovernanceResourceLimitAttributes {
            current_limit,
            current_value,
            deep_link,
            default_current_value,
            default_limit_value,
            description,
            display_name,
            limit_query,
            product,
            query,
            query_config,
            time_range,
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

impl<'de> Deserialize<'de> for GovernanceResourceLimitAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceResourceLimitAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceResourceLimitAttributesVisitor {
            type Value = GovernanceResourceLimitAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_limit: Option<f64> = None;
                let mut current_value: Option<f64> = None;
                let mut deep_link: Option<String> = None;
                let mut default_current_value: Option<f64> = None;
                let mut default_limit_value: Option<f64> = None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut limit_query: Option<crate::datadogV2::model::GovernanceLimitQuery> = None;
                let mut product: Option<String> = None;
                let mut query: Option<crate::datadogV2::model::GovernanceLimitQuery> = None;
                let mut query_config: Option<crate::datadogV2::model::GovernanceLimitQueryConfig> =
                    None;
                let mut time_range: Option<String> = None;
                let mut unit_name: Option<String> = None;
                let mut usage_status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current_limit" => {
                            current_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "current_value" => {
                            current_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deep_link" => {
                            deep_link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_current_value" => {
                            default_current_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_limit_value" => {
                            default_limit_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit_query" => {
                            limit_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let current_limit =
                    current_limit.ok_or_else(|| M::Error::missing_field("current_limit"))?;
                let current_value =
                    current_value.ok_or_else(|| M::Error::missing_field("current_value"))?;
                let deep_link = deep_link.ok_or_else(|| M::Error::missing_field("deep_link"))?;
                let default_current_value = default_current_value
                    .ok_or_else(|| M::Error::missing_field("default_current_value"))?;
                let default_limit_value = default_limit_value
                    .ok_or_else(|| M::Error::missing_field("default_limit_value"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let limit_query =
                    limit_query.ok_or_else(|| M::Error::missing_field("limit_query"))?;
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let query_config =
                    query_config.ok_or_else(|| M::Error::missing_field("query_config"))?;
                let time_range = time_range.ok_or_else(|| M::Error::missing_field("time_range"))?;
                let unit_name = unit_name.ok_or_else(|| M::Error::missing_field("unit_name"))?;
                let usage_status =
                    usage_status.ok_or_else(|| M::Error::missing_field("usage_status"))?;

                let content = GovernanceResourceLimitAttributes {
                    current_limit,
                    current_value,
                    deep_link,
                    default_current_value,
                    default_limit_value,
                    description,
                    display_name,
                    limit_query,
                    product,
                    query,
                    query_config,
                    time_range,
                    unit_name,
                    usage_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceResourceLimitAttributesVisitor)
    }
}
