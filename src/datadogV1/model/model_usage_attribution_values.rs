// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Fields in Usage Summary by tag(s).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionValues {
    /// The percentage of synthetic API test usage by tag(s).
    #[serde(rename = "api_percentage")]
    pub api_percentage: Option<f64>,
    /// The synthetic API test usage by tag(s).
    #[serde(rename = "api_usage")]
    pub api_usage: Option<f64>,
    /// The percentage of APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_percentage")]
    pub apm_fargate_percentage: Option<f64>,
    /// The APM ECS Fargate task usage by tag(s).
    #[serde(rename = "apm_fargate_usage")]
    pub apm_fargate_usage: Option<f64>,
    /// The percentage of APM host usage by tag(s).
    #[serde(rename = "apm_host_percentage")]
    pub apm_host_percentage: Option<f64>,
    /// The APM host usage by tag(s).
    #[serde(rename = "apm_host_usage")]
    pub apm_host_usage: Option<f64>,
    /// The percentage of Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_percentage")]
    pub appsec_fargate_percentage: Option<f64>,
    /// The Application Security Monitoring ECS Fargate task usage by tag(s).
    #[serde(rename = "appsec_fargate_usage")]
    pub appsec_fargate_usage: Option<f64>,
    /// The percentage of Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_percentage")]
    pub appsec_percentage: Option<f64>,
    /// The Application Security Monitoring host usage by tag(s).
    #[serde(rename = "appsec_usage")]
    pub appsec_usage: Option<f64>,
    /// The percentage of synthetic browser test usage by tag(s).
    #[serde(rename = "browser_percentage")]
    pub browser_percentage: Option<f64>,
    /// The synthetic browser test usage by tag(s).
    #[serde(rename = "browser_usage")]
    pub browser_usage: Option<f64>,
    /// The percentage of container usage by tag(s).
    #[serde(rename = "container_percentage")]
    pub container_percentage: Option<f64>,
    /// The container usage by tag(s).
    #[serde(rename = "container_usage")]
    pub container_usage: Option<f64>,
    /// The percentage of Cloud Security Management Pro container usage by tag(s)
    #[serde(rename = "cspm_container_percentage")]
    pub cspm_container_percentage: Option<f64>,
    /// The Cloud Security Management Pro container usage by tag(s)
    #[serde(rename = "cspm_container_usage")]
    pub cspm_container_usage: Option<f64>,
    /// The percentage of Cloud Security Management Pro host usage by tag(s)
    #[serde(rename = "cspm_host_percentage")]
    pub cspm_host_percentage: Option<f64>,
    /// The Cloud Security Management Pro host usage by tag(s)
    #[serde(rename = "cspm_host_usage")]
    pub cspm_host_usage: Option<f64>,
    /// The percentage of custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_percentage")]
    pub custom_timeseries_percentage: Option<f64>,
    /// The custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_usage")]
    pub custom_timeseries_usage: Option<f64>,
    /// The percentage of Cloud Workload Security container usage by tag(s)
    #[serde(rename = "cws_container_percentage")]
    pub cws_container_percentage: Option<f64>,
    /// The Cloud Workload Security container usage by tag(s)
    #[serde(rename = "cws_container_usage")]
    pub cws_container_usage: Option<f64>,
    /// The percentage of Cloud Workload Security host usage by tag(s)
    #[serde(rename = "cws_host_percentage")]
    pub cws_host_percentage: Option<f64>,
    /// The Cloud Workload Security host usage by tag(s)
    #[serde(rename = "cws_host_usage")]
    pub cws_host_usage: Option<f64>,
    /// The percentage of Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_percentage")]
    pub dbm_hosts_percentage: Option<f64>,
    /// The Database Monitoring host usage by tag(s).
    #[serde(rename = "dbm_hosts_usage")]
    pub dbm_hosts_usage: Option<f64>,
    /// The percentage of Database Monitoring normalized queries usage by tag(s).
    #[serde(rename = "dbm_queries_percentage")]
    pub dbm_queries_percentage: Option<f64>,
    /// The Database Monitoring normalized queries usage by tag(s).
    #[serde(rename = "dbm_queries_usage")]
    pub dbm_queries_usage: Option<f64>,
    /// The percentage of estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_percentage")]
    pub estimated_indexed_logs_percentage: Option<f64>,
    /// The estimated live indexed logs usage by tag(s).
    #[serde(rename = "estimated_indexed_logs_usage")]
    pub estimated_indexed_logs_usage: Option<f64>,
    /// The percentage of estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_percentage")]
    pub estimated_indexed_spans_percentage: Option<f64>,
    /// The estimated indexed spans usage by tag(s).
    #[serde(rename = "estimated_indexed_spans_usage")]
    pub estimated_indexed_spans_usage: Option<f64>,
    /// The percentage of estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_percentage")]
    pub estimated_ingested_logs_percentage: Option<f64>,
    /// The estimated live ingested logs usage by tag(s).
    #[serde(rename = "estimated_ingested_logs_usage")]
    pub estimated_ingested_logs_usage: Option<f64>,
    /// The percentage of estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_percentage")]
    pub estimated_ingested_spans_percentage: Option<f64>,
    /// The estimated ingested spans usage by tag(s).
    #[serde(rename = "estimated_ingested_spans_usage")]
    pub estimated_ingested_spans_usage: Option<f64>,
    /// The percentage of estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_percentage")]
    pub estimated_rum_sessions_percentage: Option<f64>,
    /// The estimated rum sessions usage by tag(s).
    #[serde(rename = "estimated_rum_sessions_usage")]
    pub estimated_rum_sessions_usage: Option<f64>,
    /// The percentage of infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_percentage")]
    pub infra_host_percentage: Option<f64>,
    /// The infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_usage")]
    pub infra_host_usage: Option<f64>,
    /// The percentage of Lambda function usage by tag(s).
    #[serde(rename = "lambda_functions_percentage")]
    pub lambda_functions_percentage: Option<f64>,
    /// The Lambda function usage by tag(s).
    #[serde(rename = "lambda_functions_usage")]
    pub lambda_functions_usage: Option<f64>,
    /// The percentage of Lambda invocation usage by tag(s).
    #[serde(rename = "lambda_invocations_percentage")]
    pub lambda_invocations_percentage: Option<f64>,
    /// The Lambda invocation usage by tag(s).
    #[serde(rename = "lambda_invocations_usage")]
    pub lambda_invocations_usage: Option<f64>,
    /// The percentage of network host usage by tag(s).
    #[serde(rename = "npm_host_percentage")]
    pub npm_host_percentage: Option<f64>,
    /// The network host usage by tag(s).
    #[serde(rename = "npm_host_usage")]
    pub npm_host_usage: Option<f64>,
    /// The percentage of profiled containers usage by tag(s).
    #[serde(rename = "profiled_container_percentage")]
    pub profiled_container_percentage: Option<f64>,
    /// The profiled container usage by tag(s).
    #[serde(rename = "profiled_container_usage")]
    pub profiled_container_usage: Option<f64>,
    /// The percentage of profiled hosts usage by tag(s).
    #[serde(rename = "profiled_hosts_percentage")]
    pub profiled_hosts_percentage: Option<f64>,
    /// The profiled host usage by tag(s).
    #[serde(rename = "profiled_hosts_usage")]
    pub profiled_hosts_usage: Option<f64>,
    /// The percentage of network device usage by tag(s).
    #[serde(rename = "snmp_percentage")]
    pub snmp_percentage: Option<f64>,
    /// The network device usage by tag(s).
    #[serde(rename = "snmp_usage")]
    pub snmp_usage: Option<f64>,
}

impl UsageAttributionValues {
    pub fn new() -> UsageAttributionValues {
        UsageAttributionValues {
            api_percentage: None,
            api_usage: None,
            apm_fargate_percentage: None,
            apm_fargate_usage: None,
            apm_host_percentage: None,
            apm_host_usage: None,
            appsec_fargate_percentage: None,
            appsec_fargate_usage: None,
            appsec_percentage: None,
            appsec_usage: None,
            browser_percentage: None,
            browser_usage: None,
            container_percentage: None,
            container_usage: None,
            cspm_container_percentage: None,
            cspm_container_usage: None,
            cspm_host_percentage: None,
            cspm_host_usage: None,
            custom_timeseries_percentage: None,
            custom_timeseries_usage: None,
            cws_container_percentage: None,
            cws_container_usage: None,
            cws_host_percentage: None,
            cws_host_usage: None,
            dbm_hosts_percentage: None,
            dbm_hosts_usage: None,
            dbm_queries_percentage: None,
            dbm_queries_usage: None,
            estimated_indexed_logs_percentage: None,
            estimated_indexed_logs_usage: None,
            estimated_indexed_spans_percentage: None,
            estimated_indexed_spans_usage: None,
            estimated_ingested_logs_percentage: None,
            estimated_ingested_logs_usage: None,
            estimated_ingested_spans_percentage: None,
            estimated_ingested_spans_usage: None,
            estimated_rum_sessions_percentage: None,
            estimated_rum_sessions_usage: None,
            infra_host_percentage: None,
            infra_host_usage: None,
            lambda_functions_percentage: None,
            lambda_functions_usage: None,
            lambda_invocations_percentage: None,
            lambda_invocations_usage: None,
            npm_host_percentage: None,
            npm_host_usage: None,
            profiled_container_percentage: None,
            profiled_container_usage: None,
            profiled_hosts_percentage: None,
            profiled_hosts_usage: None,
            snmp_percentage: None,
            snmp_usage: None,
        }
    }

    pub fn api_percentage(&mut self, value: f64) -> &mut Self {
        self.api_percentage = Some(value);
        self
    }

    pub fn api_usage(&mut self, value: f64) -> &mut Self {
        self.api_usage = Some(value);
        self
    }

    pub fn apm_fargate_percentage(&mut self, value: f64) -> &mut Self {
        self.apm_fargate_percentage = Some(value);
        self
    }

    pub fn apm_fargate_usage(&mut self, value: f64) -> &mut Self {
        self.apm_fargate_usage = Some(value);
        self
    }

    pub fn apm_host_percentage(&mut self, value: f64) -> &mut Self {
        self.apm_host_percentage = Some(value);
        self
    }

    pub fn apm_host_usage(&mut self, value: f64) -> &mut Self {
        self.apm_host_usage = Some(value);
        self
    }

    pub fn appsec_fargate_percentage(&mut self, value: f64) -> &mut Self {
        self.appsec_fargate_percentage = Some(value);
        self
    }

    pub fn appsec_fargate_usage(&mut self, value: f64) -> &mut Self {
        self.appsec_fargate_usage = Some(value);
        self
    }

    pub fn appsec_percentage(&mut self, value: f64) -> &mut Self {
        self.appsec_percentage = Some(value);
        self
    }

    pub fn appsec_usage(&mut self, value: f64) -> &mut Self {
        self.appsec_usage = Some(value);
        self
    }

    pub fn browser_percentage(&mut self, value: f64) -> &mut Self {
        self.browser_percentage = Some(value);
        self
    }

    pub fn browser_usage(&mut self, value: f64) -> &mut Self {
        self.browser_usage = Some(value);
        self
    }

    pub fn container_percentage(&mut self, value: f64) -> &mut Self {
        self.container_percentage = Some(value);
        self
    }

    pub fn container_usage(&mut self, value: f64) -> &mut Self {
        self.container_usage = Some(value);
        self
    }

    pub fn cspm_container_percentage(&mut self, value: f64) -> &mut Self {
        self.cspm_container_percentage = Some(value);
        self
    }

    pub fn cspm_container_usage(&mut self, value: f64) -> &mut Self {
        self.cspm_container_usage = Some(value);
        self
    }

    pub fn cspm_host_percentage(&mut self, value: f64) -> &mut Self {
        self.cspm_host_percentage = Some(value);
        self
    }

    pub fn cspm_host_usage(&mut self, value: f64) -> &mut Self {
        self.cspm_host_usage = Some(value);
        self
    }

    pub fn custom_timeseries_percentage(&mut self, value: f64) -> &mut Self {
        self.custom_timeseries_percentage = Some(value);
        self
    }

    pub fn custom_timeseries_usage(&mut self, value: f64) -> &mut Self {
        self.custom_timeseries_usage = Some(value);
        self
    }

    pub fn cws_container_percentage(&mut self, value: f64) -> &mut Self {
        self.cws_container_percentage = Some(value);
        self
    }

    pub fn cws_container_usage(&mut self, value: f64) -> &mut Self {
        self.cws_container_usage = Some(value);
        self
    }

    pub fn cws_host_percentage(&mut self, value: f64) -> &mut Self {
        self.cws_host_percentage = Some(value);
        self
    }

    pub fn cws_host_usage(&mut self, value: f64) -> &mut Self {
        self.cws_host_usage = Some(value);
        self
    }

    pub fn dbm_hosts_percentage(&mut self, value: f64) -> &mut Self {
        self.dbm_hosts_percentage = Some(value);
        self
    }

    pub fn dbm_hosts_usage(&mut self, value: f64) -> &mut Self {
        self.dbm_hosts_usage = Some(value);
        self
    }

    pub fn dbm_queries_percentage(&mut self, value: f64) -> &mut Self {
        self.dbm_queries_percentage = Some(value);
        self
    }

    pub fn dbm_queries_usage(&mut self, value: f64) -> &mut Self {
        self.dbm_queries_usage = Some(value);
        self
    }

    pub fn estimated_indexed_logs_percentage(&mut self, value: f64) -> &mut Self {
        self.estimated_indexed_logs_percentage = Some(value);
        self
    }

    pub fn estimated_indexed_logs_usage(&mut self, value: f64) -> &mut Self {
        self.estimated_indexed_logs_usage = Some(value);
        self
    }

    pub fn estimated_indexed_spans_percentage(&mut self, value: f64) -> &mut Self {
        self.estimated_indexed_spans_percentage = Some(value);
        self
    }

    pub fn estimated_indexed_spans_usage(&mut self, value: f64) -> &mut Self {
        self.estimated_indexed_spans_usage = Some(value);
        self
    }

    pub fn estimated_ingested_logs_percentage(&mut self, value: f64) -> &mut Self {
        self.estimated_ingested_logs_percentage = Some(value);
        self
    }

    pub fn estimated_ingested_logs_usage(&mut self, value: f64) -> &mut Self {
        self.estimated_ingested_logs_usage = Some(value);
        self
    }

    pub fn estimated_ingested_spans_percentage(&mut self, value: f64) -> &mut Self {
        self.estimated_ingested_spans_percentage = Some(value);
        self
    }

    pub fn estimated_ingested_spans_usage(&mut self, value: f64) -> &mut Self {
        self.estimated_ingested_spans_usage = Some(value);
        self
    }

    pub fn estimated_rum_sessions_percentage(&mut self, value: f64) -> &mut Self {
        self.estimated_rum_sessions_percentage = Some(value);
        self
    }

    pub fn estimated_rum_sessions_usage(&mut self, value: f64) -> &mut Self {
        self.estimated_rum_sessions_usage = Some(value);
        self
    }

    pub fn infra_host_percentage(&mut self, value: f64) -> &mut Self {
        self.infra_host_percentage = Some(value);
        self
    }

    pub fn infra_host_usage(&mut self, value: f64) -> &mut Self {
        self.infra_host_usage = Some(value);
        self
    }

    pub fn lambda_functions_percentage(&mut self, value: f64) -> &mut Self {
        self.lambda_functions_percentage = Some(value);
        self
    }

    pub fn lambda_functions_usage(&mut self, value: f64) -> &mut Self {
        self.lambda_functions_usage = Some(value);
        self
    }

    pub fn lambda_invocations_percentage(&mut self, value: f64) -> &mut Self {
        self.lambda_invocations_percentage = Some(value);
        self
    }

    pub fn lambda_invocations_usage(&mut self, value: f64) -> &mut Self {
        self.lambda_invocations_usage = Some(value);
        self
    }

    pub fn npm_host_percentage(&mut self, value: f64) -> &mut Self {
        self.npm_host_percentage = Some(value);
        self
    }

    pub fn npm_host_usage(&mut self, value: f64) -> &mut Self {
        self.npm_host_usage = Some(value);
        self
    }

    pub fn profiled_container_percentage(&mut self, value: f64) -> &mut Self {
        self.profiled_container_percentage = Some(value);
        self
    }

    pub fn profiled_container_usage(&mut self, value: f64) -> &mut Self {
        self.profiled_container_usage = Some(value);
        self
    }

    pub fn profiled_hosts_percentage(&mut self, value: f64) -> &mut Self {
        self.profiled_hosts_percentage = Some(value);
        self
    }

    pub fn profiled_hosts_usage(&mut self, value: f64) -> &mut Self {
        self.profiled_hosts_usage = Some(value);
        self
    }

    pub fn snmp_percentage(&mut self, value: f64) -> &mut Self {
        self.snmp_percentage = Some(value);
        self
    }

    pub fn snmp_usage(&mut self, value: f64) -> &mut Self {
        self.snmp_usage = Some(value);
        self
    }
}

impl Default for UsageAttributionValues {
    fn default() -> Self {
        Self::new()
    }
}
