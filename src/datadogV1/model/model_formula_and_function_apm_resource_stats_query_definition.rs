// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// APM resource stats query using formulas and functions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionApmResourceStatsQueryDefinition {
    /// Data source for APM resource stats queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Array of fields to group results by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Name of this query to use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name")]
    pub operation_name: Option<String>,
    /// Name of the second primary tag used within APM. Required when `primary_tag_value` is specified. See <https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog>
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Value of the second primary tag by which to filter APM data. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// APM resource name.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// APM service name.
    #[serde(rename = "service")]
    pub service: String,
    /// APM resource stat name.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1::model::FormulaAndFunctionApmResourceStatName,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionApmResourceStatsQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource,
        env: String,
        name: String,
        service: String,
        stat: crate::datadogV1::model::FormulaAndFunctionApmResourceStatName,
    ) -> FormulaAndFunctionApmResourceStatsQueryDefinition {
        FormulaAndFunctionApmResourceStatsQueryDefinition {
            data_source,
            env,
            group_by: None,
            name,
            operation_name: None,
            primary_tag_name: None,
            primary_tag_value: None,
            resource_name: None,
            service,
            stat,
            _unparsed: false,
        }
    }

    pub fn group_by(&mut self, value: Vec<String>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn operation_name(&mut self, value: String) -> &mut Self {
        self.operation_name = Some(value);
        self
    }

    pub fn primary_tag_name(&mut self, value: String) -> &mut Self {
        self.primary_tag_name = Some(value);
        self
    }

    pub fn primary_tag_value(&mut self, value: String) -> &mut Self {
        self.primary_tag_value = Some(value);
        self
    }

    pub fn resource_name(&mut self, value: String) -> &mut Self {
        self.resource_name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionApmResourceStatsQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionApmResourceStatsQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionApmResourceStatsQueryDefinitionVisitor {
            type Value = FormulaAndFunctionApmResourceStatsQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource,
                > = None;
                let mut env: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut operation_name: Option<String> = None;
                let mut primary_tag_name: Option<String> = None;
                let mut primary_tag_value: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut stat: Option<
                    crate::datadogV1::model::FormulaAndFunctionApmResourceStatName,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation_name" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                                    crate::datadogV1::model::FormulaAndFunctionApmResourceStatName::UnparsedObject(_stat) => {
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
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let stat = stat.ok_or_else(|| M::Error::missing_field("stat"))?;

                let content = FormulaAndFunctionApmResourceStatsQueryDefinition {
                    data_source,
                    env,
                    group_by,
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

        deserializer.deserialize_any(FormulaAndFunctionApmResourceStatsQueryDefinitionVisitor)
    }
}
