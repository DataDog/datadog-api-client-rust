// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of Fargate tasks run and hourly usage.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageFargateHour {
    /// The high-water mark of APM ECS Fargate tasks during the given hour.
    #[serde(rename = "apm_fargate_count", default, with = "::serde_with::rust::double_option")]
    pub apm_fargate_count: Option<Option<i64>>,
    /// The Application Security Monitoring ECS Fargate tasks during the given hour.
    #[serde(rename = "appsec_fargate_count", default, with = "::serde_with::rust::double_option")]
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
    #[serde(rename = "tasks_count", default, with = "::serde_with::rust::double_option")]
    pub tasks_count: Option<Option<i64>>,
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
        }
    }
}
