// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

pub mod api;
pub use self::api::api_authentication;
pub use self::api::api_aws_integration;
pub use self::api::api_aws_logs_integration;
pub use self::api::api_azure_integration;
pub use self::api::api_dashboard_lists;
pub use self::api::api_dashboards;
pub use self::api::api_database_monitoring;
pub use self::api::api_downtimes;
pub use self::api::api_events;
pub use self::api::api_gcp_integration;
pub use self::api::api_hosts;
pub use self::api::api_ip_ranges;
pub use self::api::api_key_management;
pub use self::api::api_logs;
pub use self::api::api_logs_indexes;
pub use self::api::api_logs_pipelines;
pub use self::api::api_metrics;
pub use self::api::api_monitors;
pub use self::api::api_notebooks;
pub use self::api::api_organizations;
pub use self::api::api_pager_duty_integration;
pub use self::api::api_security_monitoring;
pub use self::api::api_service_checks;
pub use self::api::api_service_level_objective_corrections;
pub use self::api::api_service_level_objectives;
pub use self::api::api_slack_integration;
pub use self::api::api_snapshots;
pub use self::api::api_synthetics;
pub use self::api::api_tags;
pub use self::api::api_usage_metering;
pub use self::api::api_users;
pub use self::api::api_webhooks_integration;
pub mod model;
