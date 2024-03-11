// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of Fargate tasks run and hourly usage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageFargateHour {
    /// The high-water mark of APM ECS Fargate tasks during the given hour.
    #[serde(
        rename = "apm_fargate_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub apm_fargate_count: Option<Option<i64>>,
    /// The Application Security Monitoring ECS Fargate tasks during the given hour.
    #[serde(
        rename = "appsec_fargate_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub appsec_fargate_count: Option<Option<i64>>,
    /// The average profiled task count for Fargate Profiling.
    #[serde(
        rename = "avg_profiled_fargate_tasks",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub avg_profiled_fargate_tasks: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The number of Fargate tasks run.
    #[serde(
        rename = "tasks_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub tasks_count: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageFargateHour {
    pub fn new() -> UsageFargateHour {
        UsageFargateHour {
            apm_fargate_count: None,
            appsec_fargate_count: None,
            avg_profiled_fargate_tasks: None,
            hour: None,
            org_name: None,
            public_id: None,
            tasks_count: None,
            _unparsed: false,
        }
    }

    pub fn apm_fargate_count(&mut self, value: Option<i64>) -> &mut Self {
        self.apm_fargate_count = Some(value);
        self
    }

    pub fn appsec_fargate_count(&mut self, value: Option<i64>) -> &mut Self {
        self.appsec_fargate_count = Some(value);
        self
    }

    pub fn avg_profiled_fargate_tasks(&mut self, value: Option<i64>) -> &mut Self {
        self.avg_profiled_fargate_tasks = Some(value);
        self
    }

    pub fn hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn tasks_count(&mut self, value: Option<i64>) -> &mut Self {
        self.tasks_count = Some(value);
        self
    }
}

impl Default for UsageFargateHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageFargateHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageFargateHourVisitor;
        impl<'a> Visitor<'a> for UsageFargateHourVisitor {
            type Value = UsageFargateHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_fargate_count: Option<Option<i64>> = None;
                let mut appsec_fargate_count: Option<Option<i64>> = None;
                let mut avg_profiled_fargate_tasks: Option<Option<i64>> = None;
                let mut hour: Option<String> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut tasks_count: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_fargate_count" => {
                            apm_fargate_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "appsec_fargate_count" => {
                            appsec_fargate_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_profiled_fargate_tasks" => {
                            avg_profiled_fargate_tasks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tasks_count" => {
                            tasks_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageFargateHour {
                    apm_fargate_count,
                    appsec_fargate_count,
                    avg_profiled_fargate_tasks,
                    hour,
                    org_name,
                    public_id,
                    tasks_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageFargateHourVisitor)
    }
}
