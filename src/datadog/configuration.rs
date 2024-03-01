// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use lazy_static::lazy_static;
use log::warn;
use std::collections::HashMap;
use std::env;
use reqwest_retry::{
    default_on_request_success, policies::ExponentialBackoff, RetryTransientMiddleware, Retryable,
    RetryableStrategy,
};
use reqwest_middleware::ClientBuilder;

struct RetryableStatus;
impl RetryableStrategy for RetryableStatus {
    fn handle(
        &self,
        res: &Result<reqwest::Response, reqwest_middleware::Error>,
    ) -> Option<Retryable> {
        match res {
            Ok(success) => default_on_request_success(success),
            Err(_) => None,
        }
    }
}

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
    pub(crate) client: reqwest_middleware::ClientWithMiddleware,
    pub(crate) unstable_operations: HashMap<String, bool>,
    pub(crate) auth_keys: HashMap<String, APIKey>,
    pub server_index: usize,
    pub server_variables: HashMap<String, String>,
    pub server_operation_index: HashMap<String, usize>,
    pub server_operation_variables: HashMap<String, HashMap<String, String>>,
    pub enable_retry: bool,
    pub max_retries: u32,
}

impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn client(&mut self, client: reqwest_middleware::ClientWithMiddleware) {
        self.client = client;
    }

    pub fn get_operation_host(&self, operation_str: &str) -> String {
        let operation = operation_str.to_string();
        if let Some(servers) = OPERATION_SERVERS.get(&operation) {
            let server_index = self
                .server_operation_index
                .get(&operation)
                .cloned()
                .unwrap_or(0);
            return servers
                .get(server_index)
                .expect(&format!("Server index for operation {operation} not found"))
                .get_url(
                    &self
                        .server_operation_variables
                        .get(&operation)
                        .unwrap_or(&HashMap::new()),
                );
        }
        SERVERS
            .get(self.server_index)
            .expect("Server index not found.")
            .get_url(&self.server_variables)
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

    pub fn set_retry(&mut self, enable: bool, max: u32) {
        if enable {
            let backoff_policy = ExponentialBackoff::builder().build_with_max_retries(max);

            let retry_middleware = RetryTransientMiddleware::new_with_policy_and_strategy(
                backoff_policy,
                RetryableStatus,
            );
            let client_builder = ClientBuilder::new(reqwest::Client::new()).with(retry_middleware);
            self.client = client_builder.build();
        } else {
            self.client = ClientBuilder::new(reqwest::Client::new()).build();
        }
    }

}

impl Default for Configuration {
    fn default() -> Self {
        let http_client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new());
        let user_agent = format!(
            "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
            option_env!("CARGO_PKG_VERSION").unwrap_or("?"),
            option_env!("DD_RUSTC_VERSION").unwrap_or("?"),
            env::consts::OS,
            env::consts::ARCH,
        );
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
            user_agent,
            client: http_client.build(),
            unstable_operations,
            auth_keys,
            server_index: 0,
            server_variables: HashMap::new(),
            server_operation_index: HashMap::new(),
            server_operation_variables: HashMap::new(),
            enable_retry: false,
            max_retries: 3,
        }
    }
}

lazy_static! {
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
        ])
    };
}
