// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use log::warn;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub api_key_auth: Option<String>,
    pub app_key_auth: Option<String>,
    unstable_operations: HashMap<String, bool>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }

    pub fn set_unstable_operation_enabled(&mut self, operation: &str, enabled: bool) -> bool {
        if self.unstable_operations.contains_key(operation) {
            self.unstable_operations
                .insert(operation.to_string(), enabled);
            return true;
        }

        warn!(
            "Operation {} is not an unstable operation, can't enable/disable",
            operation
        );

        false
    }

    pub fn is_unstable_operation_enabled(&self, operation: &str) -> bool {
        if self.unstable_operations.contains_key(operation) {
            return self.unstable_operations.get(operation).unwrap().clone();
        }

        warn!(
            "Operation {} is not an unstable operation, is always enabled",
            operation
        );

        false
    }

    pub fn is_unstable_operation(&self, operation: &str) -> bool {
        if self.unstable_operations.contains_key(operation) {
            return true;
        }

        false
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let http_client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new());
        let unstable_operations = HashMap::from([
            ("v2.get_active_billing_dimensions".to_owned(), false),
            ("v2.get_monthly_cost_attribution".to_owned(), false),
            ("v2.create_dora_deployment".to_owned(), false),
            ("v2.create_dora_incident".to_owned(), false),
            ("v2.create_incident".to_owned(), false),
            ("v2.create_incident_integration".to_owned(), false),
            ("v2.create_incident_todo".to_owned(), false),
            ("v2.delete_incident".to_owned(), false),
            ("v2.delete_incident_integration".to_owned(), false),
            ("v2.delete_incident_todo".to_owned(), false),
            ("v2.get_incident".to_owned(), false),
            ("v2.get_incident_integration".to_owned(), false),
            ("v2.get_incident_todo".to_owned(), false),
            ("v2.list_incident_attachments".to_owned(), false),
            ("v2.list_incident_integrations".to_owned(), false),
            ("v2.list_incidents".to_owned(), false),
            ("v2.list_incident_todos".to_owned(), false),
            ("v2.search_incidents".to_owned(), false),
            ("v2.update_incident".to_owned(), false),
            ("v2.update_incident_attachments".to_owned(), false),
            ("v2.update_incident_integration".to_owned(), false),
            ("v2.update_incident_todo".to_owned(), false),
            ("v2.query_scalar_data".to_owned(), false),
            ("v2.query_timeseries_data".to_owned(), false),
            ("v2.get_finding".to_owned(), false),
            ("v2.list_findings".to_owned(), false),
            ("v2.mute_findings".to_owned(), false),
            ("v2.create_scorecard_outcomes_batch".to_owned(), false),
            ("v2.create_scorecard_rule".to_owned(), false),
            ("v2.delete_scorecard_rule".to_owned(), false),
            ("v2.list_scorecard_outcomes".to_owned(), false),
            ("v2.list_scorecard_rules".to_owned(), false),
            ("v2.create_incident_service".to_owned(), false),
            ("v2.delete_incident_service".to_owned(), false),
            ("v2.get_incident_service".to_owned(), false),
            ("v2.list_incident_services".to_owned(), false),
            ("v2.update_incident_service".to_owned(), false),
            ("v2.create_incident_team".to_owned(), false),
            ("v2.delete_incident_team".to_owned(), false),
            ("v2.get_incident_team".to_owned(), false),
            ("v2.list_incident_teams".to_owned(), false),
            ("v2.update_incident_team".to_owned(), false),
        ]);

        Configuration {
            base_path: "https://api.datadoghq.com".to_owned(),
            user_agent: Some(format!(
                "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
                option_env!("CARGO_PKG_VERSION").unwrap_or("?"),
                option_env!("DD_RUSTC_VERSION").unwrap_or("?"),
                env::consts::OS,
                env::consts::ARCH,
            )),
            client: http_client.build(),
            api_key_auth: env::var("DD_API_KEY").ok(),
            app_key_auth: env::var("DD_APP_KEY").ok(),
            unstable_operations: unstable_operations,
        }
    }
}
