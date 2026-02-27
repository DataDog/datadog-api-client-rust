// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use lazy_static::lazy_static;
use log::warn;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct ServerVariable {
    pub description: String,
    pub default_value: String,
    pub enum_values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ServerConfiguration {
    pub url: String,
    pub description: String,
    pub variables: HashMap<String, ServerVariable>,
}

impl ServerConfiguration {
    pub fn get_url(&self, variables: &HashMap<String, String>) -> String {
        let mut url = self.url.clone();
        for (name, variable) in &self.variables {
            let value = variables.get(name).unwrap_or(&variable.default_value);
            if !variable.enum_values.contains(value) && !variable.enum_values.is_empty() {
                panic!("Value {value} for variable {name} is not in the enum values");
            }
            url = url.replace(&format!("{{{name}}}"), &value);
        }
        url
    }
}

#[derive(Debug, Clone)]
pub struct APIKey {
    pub key: String,
    pub prefix: String,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Configuration {
    pub(crate) user_agent: String,
    pub(crate) unstable_operations: HashMap<String, bool>,
    pub(crate) auth_keys: HashMap<String, APIKey>,
    pub server_index: usize,
    pub server_variables: HashMap<String, String>,
    pub server_operation_index: HashMap<String, usize>,
    pub server_operation_variables: HashMap<String, HashMap<String, String>>,
    pub proxy_url: Option<String>,
    pub enable_retry: bool,
    pub max_retries: u32,
    /// OAuth2 bearer token. When set, `BearerTokenMiddleware` will inject
    /// `Authorization: Bearer <token>` on every request.
    /// Populated automatically from the `DD_ACCESS_TOKEN` environment variable.
    pub bearer_token: Option<String>,
}

impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_operation_host(&self, operation_str: &str) -> String {
        let operation = operation_str.to_string();
        let server_index = self
            .server_operation_index
            .get(&operation)
            .unwrap_or(&self.server_index);
        let server_variables = self
            .server_operation_variables
            .get(&operation)
            .unwrap_or(&self.server_variables);
        let servers = OPERATION_SERVERS.get(&operation).unwrap_or(&SERVERS);
        servers
            .get(*server_index)
            .expect(&format!("Server index for operation {operation} not found"))
            .get_url(&server_variables)
    }

    pub fn set_unstable_operation_enabled(&mut self, operation: &str, enabled: bool) -> bool {
        if self.unstable_operations.contains_key(operation) {
            self.unstable_operations
                .insert(operation.to_string(), enabled);
            return true;
        }

        warn!("Operation {operation} is not an unstable operation, can't enable/disable");
        false
    }

    pub fn is_unstable_operation_enabled(&self, operation: &str) -> bool {
        if self.unstable_operations.contains_key(operation) {
            return self.unstable_operations.get(operation).unwrap().clone();
        }

        warn!("Operation {operation} is not an unstable operation, is always enabled");
        false
    }

    pub fn is_unstable_operation(&self, operation: &str) -> bool {
        if self.unstable_operations.contains_key(operation) {
            return true;
        }

        false
    }

    pub fn set_auth_key(&mut self, operation_str: &str, api_key: APIKey) {
        self.auth_keys.insert(operation_str.to_string(), api_key);
    }

    pub fn set_bearer_token(&mut self, token: String) {
        self.bearer_token = Some(token);
    }

    pub fn set_proxy_url(&mut self, proxy_url: Option<String>) {
        self.proxy_url = proxy_url;
    }

    pub fn set_retry(&mut self, enable_retry: bool, max_retries: u32) {
        self.enable_retry = enable_retry;
        self.max_retries = max_retries;
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let unstable_operations = HashMap::from([
            ("v2.cancel_fleet_deployment".to_owned(), false),
            ("v2.create_fleet_deployment_configure".to_owned(), false),
            ("v2.create_fleet_deployment_upgrade".to_owned(), false),
            ("v2.create_fleet_schedule".to_owned(), false),
            ("v2.delete_fleet_schedule".to_owned(), false),
            ("v2.get_fleet_agent_info".to_owned(), false),
            ("v2.get_fleet_deployment".to_owned(), false),
            ("v2.get_fleet_schedule".to_owned(), false),
            ("v2.list_fleet_agents".to_owned(), false),
            ("v2.list_fleet_agent_versions".to_owned(), false),
            ("v2.list_fleet_deployments".to_owned(), false),
            ("v2.list_fleet_schedules".to_owned(), false),
            ("v2.trigger_fleet_schedule".to_owned(), false),
            ("v2.update_fleet_schedule".to_owned(), false),
            ("v2.create_open_api".to_owned(), false),
            ("v2.delete_open_api".to_owned(), false),
            ("v2.get_open_api".to_owned(), false),
            ("v2.list_apis".to_owned(), false),
            ("v2.update_open_api".to_owned(), false),
            ("v2.create_case_jira_issue".to_owned(), false),
            ("v2.create_case_notebook".to_owned(), false),
            ("v2.create_case_service_now_ticket".to_owned(), false),
            ("v2.link_incident".to_owned(), false),
            ("v2.link_jira_issue_to_case".to_owned(), false),
            ("v2.move_case_to_project".to_owned(), false),
            ("v2.unlink_jira_issue".to_owned(), false),
            ("v2.create_change_request".to_owned(), false),
            ("v2.create_change_request_branch".to_owned(), false),
            ("v2.delete_change_request_decision".to_owned(), false),
            ("v2.get_change_request".to_owned(), false),
            ("v2.update_change_request".to_owned(), false),
            ("v2.update_change_request_decision".to_owned(), false),
            ("v2.create_aws_cloud_auth_persona_mapping".to_owned(), false),
            ("v2.delete_aws_cloud_auth_persona_mapping".to_owned(), false),
            ("v2.get_aws_cloud_auth_persona_mapping".to_owned(), false),
            ("v2.list_aws_cloud_auth_persona_mappings".to_owned(), false),
            ("v2.activate_content_pack".to_owned(), false),
            ("v2.cancel_threat_hunting_job".to_owned(), false),
            ("v2.convert_job_result_to_signal".to_owned(), false),
            ("v2.deactivate_content_pack".to_owned(), false),
            ("v2.delete_threat_hunting_job".to_owned(), false),
            ("v2.get_content_packs_states".to_owned(), false),
            ("v2.get_finding".to_owned(), false),
            ("v2.get_rule_version_history".to_owned(), false),
            ("v2.get_secrets_rules".to_owned(), false),
            ("v2.get_security_monitoring_histsignal".to_owned(), false),
            (
                "v2.get_security_monitoring_histsignals_by_job_id".to_owned(),
                false,
            ),
            ("v2.get_threat_hunting_job".to_owned(), false),
            ("v2.list_findings".to_owned(), false),
            ("v2.list_multiple_rulesets".to_owned(), false),
            ("v2.list_scanned_assets_metadata".to_owned(), false),
            ("v2.list_security_monitoring_histsignals".to_owned(), false),
            ("v2.list_threat_hunting_jobs".to_owned(), false),
            ("v2.list_vulnerabilities".to_owned(), false),
            ("v2.list_vulnerable_assets".to_owned(), false),
            ("v2.mute_findings".to_owned(), false),
            ("v2.run_threat_hunting_job".to_owned(), false),
            (
                "v2.search_security_monitoring_histsignals".to_owned(),
                false,
            ),
            ("v2.get_code_coverage_branch_summary".to_owned(), false),
            ("v2.get_code_coverage_commit_summary".to_owned(), false),
            ("v2.create_dataset".to_owned(), false),
            ("v2.delete_dataset".to_owned(), false),
            ("v2.get_all_datasets".to_owned(), false),
            ("v2.get_dataset".to_owned(), false),
            ("v2.update_dataset".to_owned(), false),
            ("v2.cancel_data_deletion_request".to_owned(), false),
            ("v2.create_data_deletion_request".to_owned(), false),
            ("v2.get_data_deletion_requests".to_owned(), false),
            ("v2.create_deployment_gate".to_owned(), false),
            ("v2.create_deployment_rule".to_owned(), false),
            ("v2.delete_deployment_gate".to_owned(), false),
            ("v2.delete_deployment_rule".to_owned(), false),
            ("v2.get_deployment_gate".to_owned(), false),
            ("v2.get_deployment_gate_rules".to_owned(), false),
            ("v2.get_deployment_rule".to_owned(), false),
            ("v2.update_deployment_gate".to_owned(), false),
            ("v2.update_deployment_rule".to_owned(), false),
            ("v2.create_hamr_org_connection".to_owned(), false),
            ("v2.get_hamr_org_connection".to_owned(), false),
            ("v2.create_global_incident_handle".to_owned(), false),
            ("v2.create_incident".to_owned(), false),
            ("v2.create_incident_attachment".to_owned(), false),
            ("v2.create_incident_integration".to_owned(), false),
            ("v2.create_incident_notification_rule".to_owned(), false),
            ("v2.create_incident_notification_template".to_owned(), false),
            ("v2.create_incident_postmortem_attachment".to_owned(), false),
            ("v2.create_incident_postmortem_template".to_owned(), false),
            ("v2.create_incident_todo".to_owned(), false),
            ("v2.create_incident_type".to_owned(), false),
            ("v2.delete_global_incident_handle".to_owned(), false),
            ("v2.delete_incident".to_owned(), false),
            ("v2.delete_incident_attachment".to_owned(), false),
            ("v2.delete_incident_integration".to_owned(), false),
            ("v2.delete_incident_notification_rule".to_owned(), false),
            ("v2.delete_incident_notification_template".to_owned(), false),
            ("v2.delete_incident_postmortem_template".to_owned(), false),
            ("v2.delete_incident_todo".to_owned(), false),
            ("v2.delete_incident_type".to_owned(), false),
            ("v2.get_global_incident_settings".to_owned(), false),
            ("v2.get_incident".to_owned(), false),
            ("v2.get_incident_integration".to_owned(), false),
            ("v2.get_incident_notification_rule".to_owned(), false),
            ("v2.get_incident_notification_template".to_owned(), false),
            ("v2.get_incident_postmortem_template".to_owned(), false),
            ("v2.get_incident_todo".to_owned(), false),
            ("v2.get_incident_type".to_owned(), false),
            ("v2.import_incident".to_owned(), false),
            ("v2.list_global_incident_handles".to_owned(), false),
            ("v2.list_incident_attachments".to_owned(), false),
            ("v2.list_incident_integrations".to_owned(), false),
            ("v2.list_incident_notification_rules".to_owned(), false),
            ("v2.list_incident_notification_templates".to_owned(), false),
            ("v2.list_incident_postmortem_templates".to_owned(), false),
            ("v2.list_incidents".to_owned(), false),
            ("v2.list_incident_todos".to_owned(), false),
            ("v2.list_incident_types".to_owned(), false),
            ("v2.search_incidents".to_owned(), false),
            ("v2.update_global_incident_handle".to_owned(), false),
            ("v2.update_global_incident_settings".to_owned(), false),
            ("v2.update_incident".to_owned(), false),
            ("v2.update_incident_attachment".to_owned(), false),
            ("v2.update_incident_integration".to_owned(), false),
            ("v2.update_incident_notification_rule".to_owned(), false),
            ("v2.update_incident_notification_template".to_owned(), false),
            ("v2.update_incident_postmortem_template".to_owned(), false),
            ("v2.update_incident_todo".to_owned(), false),
            ("v2.update_incident_type".to_owned(), false),
            ("v2.create_jira_issue_template".to_owned(), false),
            ("v2.delete_jira_account".to_owned(), false),
            ("v2.delete_jira_issue_template".to_owned(), false),
            ("v2.get_jira_issue_template".to_owned(), false),
            ("v2.list_jira_accounts".to_owned(), false),
            ("v2.list_jira_issue_templates".to_owned(), false),
            ("v2.update_jira_issue_template".to_owned(), false),
            ("v2.create_tenancy_config".to_owned(), false),
            ("v2.get_tenancy_configs".to_owned(), false),
            ("v2.create_service_now_template".to_owned(), false),
            ("v2.delete_service_now_template".to_owned(), false),
            ("v2.get_service_now_template".to_owned(), false),
            ("v2.list_service_now_assignment_groups".to_owned(), false),
            ("v2.list_service_now_business_services".to_owned(), false),
            ("v2.list_service_now_instances".to_owned(), false),
            ("v2.list_service_now_templates".to_owned(), false),
            ("v2.list_service_now_users".to_owned(), false),
            ("v2.update_service_now_template".to_owned(), false),
            ("v2.add_role_to_restriction_query".to_owned(), false),
            ("v2.create_restriction_query".to_owned(), false),
            ("v2.delete_restriction_query".to_owned(), false),
            ("v2.get_restriction_query".to_owned(), false),
            ("v2.get_role_restriction_query".to_owned(), false),
            ("v2.list_restriction_queries".to_owned(), false),
            ("v2.list_restriction_query_roles".to_owned(), false),
            ("v2.list_user_restriction_queries".to_owned(), false),
            ("v2.remove_role_from_restriction_query".to_owned(), false),
            ("v2.replace_restriction_query".to_owned(), false),
            ("v2.update_restriction_query".to_owned(), false),
            ("v2.create_monitor_user_template".to_owned(), false),
            ("v2.delete_monitor_user_template".to_owned(), false),
            ("v2.get_monitor_user_template".to_owned(), false),
            ("v2.list_monitor_user_templates".to_owned(), false),
            ("v2.update_monitor_user_template".to_owned(), false),
            (
                "v2.validate_existing_monitor_user_template".to_owned(),
                false,
            ),
            ("v2.validate_monitor_user_template".to_owned(), false),
            ("v2.list_role_templates".to_owned(), false),
            ("v2.create_connection".to_owned(), false),
            ("v2.delete_connection".to_owned(), false),
            ("v2.get_account_facet_info".to_owned(), false),
            ("v2.get_mapping".to_owned(), false),
            ("v2.get_user_facet_info".to_owned(), false),
            ("v2.list_connections".to_owned(), false),
            ("v2.query_accounts".to_owned(), false),
            ("v2.query_event_filtered_users".to_owned(), false),
            ("v2.query_users".to_owned(), false),
            ("v2.update_connection".to_owned(), false),
            ("v2.create_scorecard_outcomes_batch".to_owned(), false),
            ("v2.create_scorecard_rule".to_owned(), false),
            ("v2.delete_scorecard_rule".to_owned(), false),
            ("v2.list_scorecard_outcomes".to_owned(), false),
            ("v2.list_scorecard_rules".to_owned(), false),
            ("v2.update_scorecard_outcomes_async".to_owned(), false),
            ("v2.update_scorecard_rule".to_owned(), false),
            ("v2.list_entity_risk_scores".to_owned(), false),
            ("v2.create_incident_service".to_owned(), false),
            ("v2.delete_incident_service".to_owned(), false),
            ("v2.get_incident_service".to_owned(), false),
            ("v2.list_incident_services".to_owned(), false),
            ("v2.update_incident_service".to_owned(), false),
            ("v2.create_slo_report_job".to_owned(), false),
            ("v2.get_slo_report".to_owned(), false),
            ("v2.get_slo_report_job_status".to_owned(), false),
            ("v2.get_slo_status".to_owned(), false),
            ("v2.get_spa_recommendations".to_owned(), false),
            ("v2.get_spa_recommendations_with_shard".to_owned(), false),
            ("v2.create_custom_rule".to_owned(), false),
            ("v2.create_custom_rule_revision".to_owned(), false),
            ("v2.create_sca_resolve_vulnerable_symbols".to_owned(), false),
            ("v2.create_sca_result".to_owned(), false),
            ("v2.delete_custom_rule".to_owned(), false),
            ("v2.delete_custom_ruleset".to_owned(), false),
            ("v2.get_custom_rule".to_owned(), false),
            ("v2.get_custom_rule_revision".to_owned(), false),
            ("v2.get_custom_ruleset".to_owned(), false),
            ("v2.list_custom_rule_revisions".to_owned(), false),
            ("v2.revert_custom_rule_revision".to_owned(), false),
            ("v2.update_custom_ruleset".to_owned(), false),
            ("v2.add_member_team".to_owned(), false),
            ("v2.list_member_teams".to_owned(), false),
            ("v2.remove_member_team".to_owned(), false),
            ("v2.create_incident_team".to_owned(), false),
            ("v2.delete_incident_team".to_owned(), false),
            ("v2.get_incident_team".to_owned(), false),
            ("v2.list_incident_teams".to_owned(), false),
            ("v2.update_incident_team".to_owned(), false),
            ("v2.search_flaky_tests".to_owned(), false),
            ("v2.update_flaky_tests".to_owned(), false),
        ]);
        let mut auth_keys: HashMap<String, APIKey> = HashMap::new();
        auth_keys.insert(
            "apiKeyAuth".to_owned(),
            APIKey {
                key: env::var("DD_API_KEY").unwrap_or_default(),
                prefix: "".to_owned(),
            },
        );
        auth_keys.insert(
            "appKeyAuth".to_owned(),
            APIKey {
                key: env::var("DD_APP_KEY").unwrap_or_default(),
                prefix: "".to_owned(),
            },
        );

        Self {
            user_agent: DEFAULT_USER_AGENT.clone(),
            unstable_operations,
            auth_keys,
            server_index: 0,
            server_variables: HashMap::from([(
                "site".into(),
                env::var("DD_SITE").unwrap_or("datadoghq.com".into()),
            )]),
            server_operation_index: HashMap::new(),
            server_operation_variables: HashMap::new(),
            proxy_url: None,
            enable_retry: false,
            max_retries: 3,
            bearer_token: env::var("DD_ACCESS_TOKEN").ok(),
        }
    }
}

lazy_static! {
    pub static ref DEFAULT_USER_AGENT: String = format!(
        "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
        option_env!("CARGO_PKG_VERSION").unwrap_or("?"),
        option_env!("DD_RUSTC_VERSION").unwrap_or("?"),
        env::consts::OS,
        env::consts::ARCH,
    );
    static ref SERVERS: Vec<ServerConfiguration> = {
        vec![
            ServerConfiguration {
                url: "https://{subdomain}.{site}".into(),
                description: "No description provided".into(),
                variables: HashMap::from([
                    (
                        "site".into(),
                        ServerVariable {
                            description: "The regional site for Datadog customers.".into(),
                            default_value: "datadoghq.com".into(),
                            enum_values: vec![
                                "datadoghq.com".into(),
                                "us3.datadoghq.com".into(),
                                "us5.datadoghq.com".into(),
                                "ap1.datadoghq.com".into(),
                                "ap2.datadoghq.com".into(),
                                "datadoghq.eu".into(),
                                "ddog-gov.com".into(),
                            ],
                        },
                    ),
                    (
                        "subdomain".into(),
                        ServerVariable {
                            description: "The subdomain where the API is deployed.".into(),
                            default_value: "api".into(),
                            enum_values: vec![],
                        },
                    ),
                ]),
            },
            ServerConfiguration {
                url: "{protocol}://{name}".into(),
                description: "No description provided".into(),
                variables: HashMap::from([
                    (
                        "name".into(),
                        ServerVariable {
                            description: "Full site DNS name.".into(),
                            default_value: "api.datadoghq.com".into(),
                            enum_values: vec![],
                        },
                    ),
                    (
                        "protocol".into(),
                        ServerVariable {
                            description: "The protocol for accessing the API.".into(),
                            default_value: "https".into(),
                            enum_values: vec![],
                        },
                    ),
                ]),
            },
            ServerConfiguration {
                url: "https://{subdomain}.{site}".into(),
                description: "No description provided".into(),
                variables: HashMap::from([
                    (
                        "site".into(),
                        ServerVariable {
                            description: "Any Datadog deployment.".into(),
                            default_value: "datadoghq.com".into(),
                            enum_values: vec![],
                        },
                    ),
                    (
                        "subdomain".into(),
                        ServerVariable {
                            description: "The subdomain where the API is deployed.".into(),
                            default_value: "api".into(),
                            enum_values: vec![],
                        },
                    ),
                ]),
            },
        ]
    };
    static ref OPERATION_SERVERS: HashMap<String, Vec<ServerConfiguration>> = {
        HashMap::from([
            (
                "v1.get_ip_ranges".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "The regional site for Datadog customers.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![
                                        "datadoghq.com".into(),
                                        "us3.datadoghq.com".into(),
                                        "us5.datadoghq.com".into(),
                                        "ap1.datadoghq.com".into(),
                                        "ap2.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                    ],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "ip-ranges".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "ip-ranges.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.datadoghq.com".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "subdomain".into(),
                            ServerVariable {
                                description: "The subdomain where the API is deployed.".into(),
                                default_value: "ip-ranges".into(),
                                enum_values: vec![],
                            },
                        )]),
                    },
                ],
            ),
            (
                "v1.submit_log".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "The regional site for Datadog customers.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![
                                        "datadoghq.com".into(),
                                        "us3.datadoghq.com".into(),
                                        "us5.datadoghq.com".into(),
                                        "ap1.datadoghq.com".into(),
                                        "ap2.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                    ],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "http-intake.logs".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "http-intake.logs.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "http-intake.logs".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.create_event".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "The regional site for customers.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![
                                        "datadoghq.com".into(),
                                        "us3.datadoghq.com".into(),
                                        "us5.datadoghq.com".into(),
                                        "ap1.datadoghq.com".into(),
                                        "ap2.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                    ],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "event-management-intake".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "event-management-intake.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "event-management-intake".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.submit_log".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "The regional site for customers.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![
                                        "datadoghq.com".into(),
                                        "us3.datadoghq.com".into(),
                                        "us5.datadoghq.com".into(),
                                        "ap1.datadoghq.com".into(),
                                        "ap2.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                    ],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "http-intake.logs".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "http-intake.logs.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "http-intake.logs".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.create_on_call_page".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "site".into(),
                            ServerVariable {
                                description: "The globally available endpoint for On-Call.".into(),
                                default_value: "navy.oncall.datadoghq.com".into(),
                                enum_values: vec![
                                    "lava.oncall.datadoghq.com".into(),
                                    "saffron.oncall.datadoghq.com".into(),
                                    "navy.oncall.datadoghq.com".into(),
                                    "coral.oncall.datadoghq.com".into(),
                                    "teal.oncall.datadoghq.com".into(),
                                    "beige.oncall.datadoghq.eu".into(),
                                ],
                            },
                        )]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "api.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "api".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.acknowledge_on_call_page".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "site".into(),
                            ServerVariable {
                                description: "The globally available endpoint for On-Call.".into(),
                                default_value: "navy.oncall.datadoghq.com".into(),
                                enum_values: vec![
                                    "lava.oncall.datadoghq.com".into(),
                                    "saffron.oncall.datadoghq.com".into(),
                                    "navy.oncall.datadoghq.com".into(),
                                    "coral.oncall.datadoghq.com".into(),
                                    "teal.oncall.datadoghq.com".into(),
                                    "beige.oncall.datadoghq.eu".into(),
                                ],
                            },
                        )]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "api.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "api".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.escalate_on_call_page".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "site".into(),
                            ServerVariable {
                                description: "The globally available endpoint for On-Call.".into(),
                                default_value: "navy.oncall.datadoghq.com".into(),
                                enum_values: vec![
                                    "lava.oncall.datadoghq.com".into(),
                                    "saffron.oncall.datadoghq.com".into(),
                                    "navy.oncall.datadoghq.com".into(),
                                    "coral.oncall.datadoghq.com".into(),
                                    "teal.oncall.datadoghq.com".into(),
                                    "beige.oncall.datadoghq.eu".into(),
                                ],
                            },
                        )]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "api.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "api".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.resolve_on_call_page".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "site".into(),
                            ServerVariable {
                                description: "The globally available endpoint for On-Call.".into(),
                                default_value: "navy.oncall.datadoghq.com".into(),
                                enum_values: vec![
                                    "lava.oncall.datadoghq.com".into(),
                                    "saffron.oncall.datadoghq.com".into(),
                                    "navy.oncall.datadoghq.com".into(),
                                    "coral.oncall.datadoghq.com".into(),
                                    "teal.oncall.datadoghq.com".into(),
                                    "beige.oncall.datadoghq.eu".into(),
                                ],
                            },
                        )]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "api.datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "api".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
            (
                "v2.submit_product_analytics_event".into(),
                vec![
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "The regional site for customers.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![
                                        "datadoghq.com".into(),
                                        "us3.datadoghq.com".into(),
                                        "us5.datadoghq.com".into(),
                                        "ap1.datadoghq.com".into(),
                                        "ap2.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                    ],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "browser-intake".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "{protocol}://{name}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "name".into(),
                                ServerVariable {
                                    description: "Full site DNS name.".into(),
                                    default_value: "browser-intake-datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "protocol".into(),
                                ServerVariable {
                                    description: "The protocol for accessing the API.".into(),
                                    default_value: "https".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                    ServerConfiguration {
                        url: "https://{subdomain}.{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([
                            (
                                "site".into(),
                                ServerVariable {
                                    description: "Any Datadog deployment.".into(),
                                    default_value: "datadoghq.com".into(),
                                    enum_values: vec![],
                                },
                            ),
                            (
                                "subdomain".into(),
                                ServerVariable {
                                    description: "The subdomain where the API is deployed.".into(),
                                    default_value: "browser-intake".into(),
                                    enum_values: vec![],
                                },
                            ),
                        ]),
                    },
                ],
            ),
        ])
    };
}
