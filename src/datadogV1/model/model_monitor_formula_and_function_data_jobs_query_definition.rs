// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions data jobs query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataJobsQueryDefinition {
    /// The type of job being monitored. Valid values include:
    /// `databricks.job`, `spark.application`, `airflow.dag`,
    /// `dbt.job`, `dbt.model`, `dbt.test`, `glue.job`.
    /// Custom job types are supported with the `custom.ol.` prefix.
    #[serde(rename = "job_type")]
    pub job_type: String,
    /// Filter expression used to select the jobs to monitor.
    #[serde(rename = "jobs_query")]
    pub jobs_query: String,
    /// Name of the query for use in formulas. Must be `run_query`.
    #[serde(rename = "name")]
    pub name: String,
    /// Query dialect for data jobs queries. Currently only `metric` is supported.
    #[serde(rename = "query_dialect")]
    pub query_dialect: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataJobsQueryDefinition {
    pub fn new(
        job_type: String,
        jobs_query: String,
        name: String,
        query_dialect: String,
    ) -> MonitorFormulaAndFunctionDataJobsQueryDefinition {
        MonitorFormulaAndFunctionDataJobsQueryDefinition {
            job_type,
            jobs_query,
            name,
            query_dialect,
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

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataJobsQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataJobsQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataJobsQueryDefinitionVisitor {
            type Value = MonitorFormulaAndFunctionDataJobsQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut job_type: Option<String> = None;
                let mut jobs_query: Option<String> = None;
                let mut name: Option<String> = None;
                let mut query_dialect: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "job_type" => {
                            job_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobs_query" => {
                            jobs_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_dialect" => {
                            query_dialect =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let job_type = job_type.ok_or_else(|| M::Error::missing_field("job_type"))?;
                let jobs_query = jobs_query.ok_or_else(|| M::Error::missing_field("jobs_query"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query_dialect =
                    query_dialect.ok_or_else(|| M::Error::missing_field("query_dialect"))?;

                let content = MonitorFormulaAndFunctionDataJobsQueryDefinition {
                    job_type,
                    jobs_query,
                    name,
                    query_dialect,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataJobsQueryDefinitionVisitor)
    }
}
