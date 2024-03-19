// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions APM dependency stats query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionApmDependencyStatsQueryDefinition {
    /// Data source for APM dependency stats queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Determines whether stats for upstream or downstream dependencies should be queried.
    #[serde(rename = "is_upstream")]
    pub is_upstream: Option<bool>,
    /// Name of query to use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name")]
    pub operation_name: String,
    /// The name of the second primary tag used within APM; required when `primary_tag_value` is specified. See <https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog.>
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Filter APM data by the second primary tag. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// APM resource.
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    /// APM service.
    #[serde(rename = "service")]
    pub service: String,
    /// APM statistic.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionApmDependencyStatsQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource,
        env: String,
        name: String,
        operation_name: String,
        resource_name: String,
        service: String,
        stat: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName,
    ) -> FormulaAndFunctionApmDependencyStatsQueryDefinition {
        FormulaAndFunctionApmDependencyStatsQueryDefinition {
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
}

impl<'de> Deserialize<'de> for FormulaAndFunctionApmDependencyStatsQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionApmDependencyStatsQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionApmDependencyStatsQueryDefinitionVisitor {
            type Value = FormulaAndFunctionApmDependencyStatsQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource,
                > = None;
                let mut env: Option<String> = None;
                let mut is_upstream: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut primary_tag_name: Option<String> = None;
                let mut primary_tag_value: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut stat: Option<
                    crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource::UnparsedObject(_data_source) => {
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
                                    crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName::UnparsedObject(_stat) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
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

                let content = FormulaAndFunctionApmDependencyStatsQueryDefinition {
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
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionApmDependencyStatsQueryDefinitionVisitor)
    }
}
