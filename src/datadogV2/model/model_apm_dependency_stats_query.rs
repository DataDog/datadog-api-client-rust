// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A query for APM dependency statistics between services, such as call latency and error rates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApmDependencyStatsQuery {
    /// A data source for APM dependency statistics queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::ApmDependencyStatsDataSource,
    /// The environment to query.
    #[serde(rename = "env")]
    pub env: String,
    /// Determines whether stats for upstream or downstream dependencies should be queried.
    #[serde(rename = "is_upstream")]
    pub is_upstream: Option<bool>,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// The APM operation name.
    #[serde(rename = "operation_name")]
    pub operation_name: String,
    /// The name of the second primary tag used within APM; required when `primary_tag_value` is specified. See <https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog.>
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Filter APM data by the second primary tag. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// The resource name to filter by.
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    /// The service name to filter by.
    #[serde(rename = "service")]
    pub service: String,
    /// The APM dependency statistic to query.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV2::model::ApmDependencyStatName,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApmDependencyStatsQuery {
    pub fn new(
        data_source: crate::datadogV2::model::ApmDependencyStatsDataSource,
        env: String,
        name: String,
        operation_name: String,
        resource_name: String,
        service: String,
        stat: crate::datadogV2::model::ApmDependencyStatName,
    ) -> ApmDependencyStatsQuery {
        ApmDependencyStatsQuery {
            data_source,
            env,
            is_upstream: None,
            name,
            operation_name,
            primary_tag_name: None,
            primary_tag_value: None,
            resource_name,
            service,
            stat,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_upstream(mut self, value: bool) -> Self {
        self.is_upstream = Some(value);
        self
    }

    pub fn primary_tag_name(mut self, value: String) -> Self {
        self.primary_tag_name = Some(value);
        self
    }

    pub fn primary_tag_value(mut self, value: String) -> Self {
        self.primary_tag_value = Some(value);
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

impl<'de> Deserialize<'de> for ApmDependencyStatsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApmDependencyStatsQueryVisitor;
        impl<'a> Visitor<'a> for ApmDependencyStatsQueryVisitor {
            type Value = ApmDependencyStatsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV2::model::ApmDependencyStatsDataSource> =
                    None;
                let mut env: Option<String> = None;
                let mut is_upstream: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut primary_tag_name: Option<String> = None;
                let mut primary_tag_value: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut stat: Option<crate::datadogV2::model::ApmDependencyStatName> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::ApmDependencyStatsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_upstream" => {
                            if v.is_null() {
                                continue;
                            }
                            is_upstream =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_name" => {
                            operation_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_tag_name" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_tag_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_tag_value" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_tag_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stat" => {
                            stat = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stat) = stat {
                                match _stat {
                                    crate::datadogV2::model::ApmDependencyStatName::UnparsedObject(_stat) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let operation_name =
                    operation_name.ok_or_else(|| M::Error::missing_field("operation_name"))?;
                let resource_name =
                    resource_name.ok_or_else(|| M::Error::missing_field("resource_name"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = ApmDependencyStatsQuery {
                    data_source,
                    env,
                    is_upstream,
                    name,
                    operation_name,
                    primary_tag_name,
                    primary_tag_value,
                    resource_name,
                    service,
                    stat,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApmDependencyStatsQueryVisitor)
    }
}
