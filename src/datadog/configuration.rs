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
            ("v2.list_fleet_agent_tracers".to_owned(), false),
            ("v2.list_fleet_agent_versions".to_owned(), false),
            ("v2.list_fleet_deployments".to_owned(), false),
            ("v2.list_fleet_schedules".to_owned(), false),
            ("v2.list_fleet_tracers".to_owned(), false),
            ("v2.trigger_fleet_schedule".to_owned(), false),
            ("v2.update_fleet_schedule".to_owned(), false),
            ("v2.aggregate_llm_obs_experimentation".to_owned(), false),
            ("v2.batch_update_llm_obs_dataset".to_owned(), false),
            ("v2.clone_llm_obs_dataset".to_owned(), false),
            ("v2.create_llm_obs_annotation_queue".to_owned(), false),
            (
                "v2.create_llm_obs_annotation_queue_interactions".to_owned(),
                false,
            ),
            ("v2.create_llm_obs_dataset".to_owned(), false),
            ("v2.create_llm_obs_dataset_records".to_owned(), false),
            ("v2.create_llm_obs_experiment".to_owned(), false),
            ("v2.create_llm_obs_experiment_events".to_owned(), false),
            ("v2.create_llm_obs_integration_inference".to_owned(), false),
            ("v2.create_llm_obs_project".to_owned(), false),
            ("v2.create_llm_obs_prompt".to_owned(), false),
            ("v2.create_llm_obs_prompt_version".to_owned(), false),
            ("v2.delete_llm_obs_annotation_queue".to_owned(), false),
            (
                "v2.delete_llm_obs_annotation_queue_interactions".to_owned(),
                false,
            ),
            ("v2.delete_llm_obs_annotations".to_owned(), false),
            ("v2.delete_llm_obs_custom_eval_config".to_owned(), false),
            ("v2.delete_llm_obs_data".to_owned(), false),
            ("v2.delete_llm_obs_dataset_records".to_owned(), false),
            ("v2.delete_llm_obs_datasets".to_owned(), false),
            ("v2.delete_llm_obs_experiments".to_owned(), false),
            ("v2.delete_llm_obs_patterns_config".to_owned(), false),
            ("v2.delete_llm_obs_projects".to_owned(), false),
            ("v2.delete_llm_obs_prompt".to_owned(), false),
            ("v2.export_llm_obs_dataset".to_owned(), false),
            ("v2.get_llm_obs_annotated_interactions".to_owned(), false),
            (
                "v2.get_llm_obs_annotated_interactions_by_trace_i_ds".to_owned(),
                false,
            ),
            (
                "v2.get_llm_obs_annotation_queue_label_schema".to_owned(),
                false,
            ),
            ("v2.get_llm_obs_custom_eval_config".to_owned(), false),
            ("v2.get_llm_obs_dataset_draft_state".to_owned(), false),
            ("v2.get_llm_obs_patterns_config".to_owned(), false),
            ("v2.get_llm_obs_patterns_run_status".to_owned(), false),
            ("v2.get_llm_obs_prompt".to_owned(), false),
            ("v2.get_llm_obs_prompt_version".to_owned(), false),
            ("v2.list_llm_obs_annotation_queues".to_owned(), false),
            ("v2.list_llm_obs_dataset_records".to_owned(), false),
            ("v2.list_llm_obs_datasets".to_owned(), false),
            ("v2.list_llm_obs_dataset_versions".to_owned(), false),
            ("v2.list_llm_obs_experiment_events".to_owned(), false),
            ("v2.list_llm_obs_experiment_events_v1".to_owned(), false),
            ("v2.list_llm_obs_experiment_events_v2".to_owned(), false),
            ("v2.list_llm_obs_experiments".to_owned(), false),
            ("v2.list_llm_obs_integration_accounts".to_owned(), false),
            ("v2.list_llm_obs_integration_models".to_owned(), false),
            (
                "v2.list_llm_obs_patterns_clustered_points".to_owned(),
                false,
            ),
            ("v2.list_llm_obs_patterns_configs".to_owned(), false),
            ("v2.list_llm_obs_patterns_runs".to_owned(), false),
            ("v2.list_llm_obs_patterns_topics".to_owned(), false),
            (
                "v2.list_llm_obs_patterns_topics_with_clustered_points".to_owned(),
                false,
            ),
            ("v2.list_llm_obs_projects".to_owned(), false),
            ("v2.list_llm_obs_prompts".to_owned(), false),
            ("v2.list_llm_obs_prompt_versions".to_owned(), false),
            ("v2.list_llm_obs_spans".to_owned(), false),
            ("v2.lock_llm_obs_dataset_draft_state".to_owned(), false),
            ("v2.restore_llm_obs_dataset_version".to_owned(), false),
            ("v2.search_llm_obs_experimentation".to_owned(), false),
            ("v2.search_llm_obs_spans".to_owned(), false),
            ("v2.simple_search_llm_obs_experimentation".to_owned(), false),
            ("v2.trigger_llm_obs_patterns".to_owned(), false),
            ("v2.unlock_llm_obs_dataset_draft_state".to_owned(), false),
            ("v2.update_llm_obs_annotation_queue".to_owned(), false),
            (
                "v2.update_llm_obs_annotation_queue_label_schema".to_owned(),
                false,
            ),
            ("v2.update_llm_obs_custom_eval_config".to_owned(), false),
            ("v2.update_llm_obs_dataset".to_owned(), false),
            ("v2.update_llm_obs_dataset_records".to_owned(), false),
            ("v2.update_llm_obs_experiment".to_owned(), false),
            ("v2.update_llm_obs_project".to_owned(), false),
            ("v2.update_llm_obs_prompt".to_owned(), false),
            ("v2.update_llm_obs_prompt_version".to_owned(), false),
            ("v2.upload_llm_obs_dataset_records_file".to_owned(), false),
            ("v2.upsert_llm_obs_annotations".to_owned(), false),
            ("v2.upsert_llm_obs_patterns_config".to_owned(), false),
            ("v2.create_annotation".to_owned(), false),
            ("v2.delete_annotation".to_owned(), false),
            ("v2.get_page_annotations".to_owned(), false),
            ("v2.list_annotations".to_owned(), false),
            ("v2.update_annotation".to_owned(), false),
            ("v2.anonymize_users".to_owned(), false),
            ("v2.validate".to_owned(), false),
            ("v2.create_open_api".to_owned(), false),
            ("v2.delete_open_api".to_owned(), false),
            ("v2.get_open_api".to_owned(), false),
            ("v2.list_apis".to_owned(), false),
            ("v2.update_open_api".to_owned(), false),
            ("v2.get_investigation".to_owned(), false),
            ("v2.list_investigations".to_owned(), false),
            ("v2.trigger_investigation".to_owned(), false),
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
            ("v2.activate_integration".to_owned(), false),
            ("v2.attach_linear_issue".to_owned(), false),
            (
                "v2.batch_get_security_monitoring_dataset_dependencies".to_owned(),
                false,
            ),
            (
                "v2.bulk_create_sample_log_generation_subscriptions".to_owned(),
                false,
            ),
            (
                "v2.bulk_export_security_monitoring_terraform_resources".to_owned(),
                false,
            ),
            ("v2.cancel_historical_job".to_owned(), false),
            ("v2.convert_job_result_to_signal".to_owned(), false),
            (
                "v2.convert_security_monitoring_terraform_resource".to_owned(),
                false,
            ),
            ("v2.create_io_c_triage_state".to_owned(), false),
            ("v2.create_linear_issues".to_owned(), false),
            (
                "v2.create_sample_log_generation_subscription".to_owned(),
                false,
            ),
            (
                "v2.create_security_findings_automation_due_date_rule".to_owned(),
                false,
            ),
            (
                "v2.create_security_findings_automation_mute_rule".to_owned(),
                false,
            ),
            (
                "v2.create_security_findings_automation_ticket_creation_rule".to_owned(),
                false,
            ),
            ("v2.create_security_monitoring_dataset".to_owned(), false),
            (
                "v2.create_security_monitoring_integration_config".to_owned(),
                false,
            ),
            ("v2.create_static_analysis_ast".to_owned(), false),
            (
                "v2.create_static_analysis_server_analysis".to_owned(),
                false,
            ),
            ("v2.deactivate_content_pack".to_owned(), false),
            ("v2.deactivate_integration".to_owned(), false),
            ("v2.delete_historical_job".to_owned(), false),
            (
                "v2.delete_sample_log_generation_subscription".to_owned(),
                false,
            ),
            (
                "v2.delete_security_findings_automation_due_date_rule".to_owned(),
                false,
            ),
            (
                "v2.delete_security_findings_automation_mute_rule".to_owned(),
                false,
            ),
            (
                "v2.delete_security_findings_automation_ticket_creation_rule".to_owned(),
                false,
            ),
            ("v2.delete_security_monitoring_dataset".to_owned(), false),
            (
                "v2.delete_security_monitoring_integration_config".to_owned(),
                false,
            ),
            (
                "v2.export_security_monitoring_terraform_resource".to_owned(),
                false,
            ),
            ("v2.get_content_packs_states".to_owned(), false),
            ("v2.get_entity_context".to_owned(), false),
            ("v2.get_entra_id_azure_app_registrations".to_owned(), false),
            ("v2.get_finding".to_owned(), false),
            ("v2.get_historical_job".to_owned(), false),
            ("v2.get_indicator_of_compromise".to_owned(), false),
            ("v2.get_rule_version_history".to_owned(), false),
            ("v2.get_secrets_rules".to_owned(), false),
            (
                "v2.get_security_findings_automation_due_date_rule".to_owned(),
                false,
            ),
            (
                "v2.get_security_findings_automation_mute_rule".to_owned(),
                false,
            ),
            (
                "v2.get_security_findings_automation_ticket_creation_rule".to_owned(),
                false,
            ),
            ("v2.get_security_monitoring_dataset".to_owned(), false),
            (
                "v2.get_security_monitoring_dataset_by_version".to_owned(),
                false,
            ),
            (
                "v2.get_security_monitoring_dataset_version_history".to_owned(),
                false,
            ),
            ("v2.get_security_monitoring_histsignal".to_owned(), false),
            (
                "v2.get_security_monitoring_histsignals_by_job_id".to_owned(),
                false,
            ),
            (
                "v2.get_security_monitoring_integration_config".to_owned(),
                false,
            ),
            ("v2.get_signal_entities".to_owned(), false),
            ("v2.get_single_entity_context".to_owned(), false),
            ("v2.get_static_analysis_default_rulesets".to_owned(), false),
            ("v2.get_static_analysis_node_types".to_owned(), false),
            ("v2.get_static_analysis_ruleset".to_owned(), false),
            ("v2.get_static_analysis_tree_sitter_wasm".to_owned(), false),
            ("v2.import_security_vulnerabilities".to_owned(), false),
            ("v2.list_findings".to_owned(), false),
            ("v2.list_historical_jobs".to_owned(), false),
            ("v2.list_indicators_of_compromise".to_owned(), false),
            ("v2.list_multiple_rulesets".to_owned(), false),
            (
                "v2.list_sample_log_generation_subscriptions".to_owned(),
                false,
            ),
            ("v2.list_scanned_assets_metadata".to_owned(), false),
            (
                "v2.list_security_findings_automation_due_date_rules".to_owned(),
                false,
            ),
            (
                "v2.list_security_findings_automation_mute_rules".to_owned(),
                false,
            ),
            (
                "v2.list_security_findings_automation_ticket_creation_rules".to_owned(),
                false,
            ),
            ("v2.list_security_monitoring_datasets".to_owned(), false),
            ("v2.list_security_monitoring_histsignals".to_owned(), false),
            (
                "v2.list_security_monitoring_integration_configs".to_owned(),
                false,
            ),
            ("v2.list_static_analysis_codegen_rulesets".to_owned(), false),
            ("v2.list_vulnerabilities".to_owned(), false),
            ("v2.list_vulnerable_assets".to_owned(), false),
            (
                "v2.reorder_security_findings_automation_due_date_rules".to_owned(),
                false,
            ),
            (
                "v2.reorder_security_findings_automation_mute_rules".to_owned(),
                false,
            ),
            (
                "v2.reorder_security_findings_automation_ticket_creation_rules".to_owned(),
                false,
            ),
            ("v2.restore_security_monitoring_rule".to_owned(), false),
            ("v2.run_historical_job".to_owned(), false),
            (
                "v2.search_security_monitoring_histsignals".to_owned(),
                false,
            ),
            ("v2.update_findings_assignee".to_owned(), false),
            (
                "v2.update_security_findings_automation_due_date_rule".to_owned(),
                false,
            ),
            (
                "v2.update_security_findings_automation_mute_rule".to_owned(),
                false,
            ),
            (
                "v2.update_security_findings_automation_ticket_creation_rule".to_owned(),
                false,
            ),
            ("v2.update_security_monitoring_dataset".to_owned(), false),
            (
                "v2.update_security_monitoring_integration_config".to_owned(),
                false,
            ),
            (
                "v2.validate_security_monitoring_integration_config".to_owned(),
                false,
            ),
            (
                "v2.validate_security_monitoring_integration_credentials".to_owned(),
                false,
            ),
            ("v2.get_code_coverage_branch_summary".to_owned(), false),
            ("v2.get_code_coverage_commit_summary".to_owned(), false),
            ("v2.get_rule_based_view".to_owned(), false),
            ("v2.get_commitments_commitment_list".to_owned(), false),
            ("v2.get_commitments_coverage_scalar".to_owned(), false),
            ("v2.get_commitments_coverage_timeseries".to_owned(), false),
            (
                "v2.get_commitments_on_demand_hotspots_scalar".to_owned(),
                false,
            ),
            ("v2.get_commitments_savings_scalar".to_owned(), false),
            ("v2.get_commitments_savings_timeseries".to_owned(), false),
            ("v2.get_commitments_utilization_scalar".to_owned(), false),
            (
                "v2.get_commitments_utilization_timeseries".to_owned(),
                false,
            ),
            ("v2.get_cost_anomaly".to_owned(), false),
            ("v2.get_cost_tag_metadata_currency".to_owned(), false),
            ("v2.list_cost_anomalies".to_owned(), false),
            ("v2.list_cost_tag_key_sources".to_owned(), false),
            ("v2.list_cost_tag_metadata".to_owned(), false),
            ("v2.list_cost_tag_metadata_metrics".to_owned(), false),
            ("v2.list_cost_tag_metadata_months".to_owned(), false),
            ("v2.list_cost_tag_metadata_orchestrators".to_owned(), false),
            ("v2.search_cost_recommendations".to_owned(), false),
            ("v2.create_ownership_feedback".to_owned(), false),
            ("v2.get_ownership_evidence".to_owned(), false),
            ("v2.get_ownership_inference".to_owned(), false),
            ("v2.list_ownership_history".to_owned(), false),
            ("v2.list_ownership_history_by_owner_type".to_owned(), false),
            ("v2.list_ownership_inferences".to_owned(), false),
            ("v2.get_csm_agentless_host_facet_info".to_owned(), false),
            ("v2.get_csm_unified_host_facet_info".to_owned(), false),
            ("v2.list_csm_agentless_host_facets".to_owned(), false),
            ("v2.list_csm_agentless_hosts".to_owned(), false),
            ("v2.list_csm_unified_host_facets".to_owned(), false),
            ("v2.list_csm_unified_hosts".to_owned(), false),
            (
                "v2.list_shared_dashboards_by_dashboard_id".to_owned(),
                false,
            ),
            ("v2.create_dashboard_secure_embed".to_owned(), false),
            ("v2.delete_dashboard_secure_embed".to_owned(), false),
            ("v2.get_dashboard_secure_embed".to_owned(), false),
            ("v2.update_dashboard_secure_embed".to_owned(), false),
            ("v2.get_dashboard_usage".to_owned(), false),
            ("v2.list_dashboards_usage".to_owned(), false),
            (
                "v2.get_data_observability_monitor_run_status".to_owned(),
                false,
            ),
            ("v2.run_data_observability_monitor".to_owned(), false),
            ("v2.create_dataset".to_owned(), false),
            ("v2.delete_dataset".to_owned(), false),
            ("v2.get_all_datasets".to_owned(), false),
            ("v2.get_dataset".to_owned(), false),
            ("v2.update_dataset".to_owned(), false),
            ("v2.execute_ddsql_tabular_query".to_owned(), false),
            ("v2.fetch_ddsql_tabular_query".to_owned(), false),
            ("v2.cancel_data_deletion_request".to_owned(), false),
            ("v2.create_data_deletion_request".to_owned(), false),
            ("v2.get_data_deletion_requests".to_owned(), false),
            ("v2.create_deployment_gate".to_owned(), false),
            ("v2.create_deployment_rule".to_owned(), false),
            ("v2.delete_deployment_gate".to_owned(), false),
            ("v2.delete_deployment_rule".to_owned(), false),
            ("v2.get_deployment_gate".to_owned(), false),
            ("v2.get_deployment_gate_rules".to_owned(), false),
            (
                "v2.get_deployment_gates_evaluation_result".to_owned(),
                false,
            ),
            ("v2.get_deployment_rule".to_owned(), false),
            ("v2.list_deployment_gates".to_owned(), false),
            ("v2.trigger_deployment_gates_evaluation".to_owned(), false),
            ("v2.update_deployment_gate".to_owned(), false),
            ("v2.update_deployment_rule".to_owned(), false),
            ("v2.clone_form".to_owned(), false),
            ("v2.create_and_publish_form".to_owned(), false),
            ("v2.create_form".to_owned(), false),
            ("v2.delete_form".to_owned(), false),
            ("v2.get_form".to_owned(), false),
            ("v2.list_forms".to_owned(), false),
            ("v2.publish_form".to_owned(), false),
            ("v2.update_form".to_owned(), false),
            ("v2.upsert_and_publish_form_version".to_owned(), false),
            ("v2.upsert_form_version".to_owned(), false),
            ("v2.update_org_saml_configurations".to_owned(), false),
            ("v2.get_governance_control".to_owned(), false),
            ("v2.list_governance_controls".to_owned(), false),
            ("v2.update_governance_control".to_owned(), false),
            ("v2.list_governance_insights".to_owned(), false),
            ("v2.create_hamr_org_connection".to_owned(), false),
            ("v2.get_hamr_org_connection".to_owned(), false),
            ("v2.delete_entity_integration_config".to_owned(), false),
            ("v2.get_entity_integration_config".to_owned(), false),
            ("v2.update_entity_integration_config".to_owned(), false),
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
            ("v2.create_incident_user_defined_field".to_owned(), false),
            ("v2.create_incident_user_defined_role".to_owned(), false),
            ("v2.delete_global_incident_handle".to_owned(), false),
            ("v2.delete_incident".to_owned(), false),
            ("v2.delete_incident_attachment".to_owned(), false),
            ("v2.delete_incident_integration".to_owned(), false),
            ("v2.delete_incident_notification_rule".to_owned(), false),
            ("v2.delete_incident_notification_template".to_owned(), false),
            ("v2.delete_incident_postmortem_template".to_owned(), false),
            ("v2.delete_incident_todo".to_owned(), false),
            ("v2.delete_incident_type".to_owned(), false),
            ("v2.delete_incident_user_defined_field".to_owned(), false),
            ("v2.delete_incident_user_defined_role".to_owned(), false),
            ("v2.get_global_incident_settings".to_owned(), false),
            ("v2.get_incident".to_owned(), false),
            ("v2.get_incident_integration".to_owned(), false),
            ("v2.get_incident_notification_rule".to_owned(), false),
            ("v2.get_incident_notification_template".to_owned(), false),
            ("v2.get_incident_postmortem_template".to_owned(), false),
            ("v2.get_incident_todo".to_owned(), false),
            ("v2.get_incident_type".to_owned(), false),
            ("v2.get_incident_user_defined_field".to_owned(), false),
            ("v2.get_incident_user_defined_role".to_owned(), false),
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
            ("v2.list_incident_user_defined_fields".to_owned(), false),
            ("v2.list_incident_user_defined_roles".to_owned(), false),
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
            ("v2.update_incident_user_defined_field".to_owned(), false),
            ("v2.update_incident_user_defined_role".to_owned(), false),
            ("v2.create_aws_account_ccm_config".to_owned(), false),
            ("v2.delete_aws_account_ccm_config".to_owned(), false),
            ("v2.get_aws_account_ccm_config".to_owned(), false),
            ("v2.get_aws_metric_name_filter_preview".to_owned(), false),
            ("v2.preview_aws_metric_name_filter".to_owned(), false),
            ("v2.update_aws_account_ccm_config".to_owned(), false),
            ("v2.validate_awsccm_config".to_owned(), false),
            ("v2.create_jira_issue_template".to_owned(), false),
            ("v2.delete_jira_account".to_owned(), false),
            ("v2.delete_jira_issue_template".to_owned(), false),
            ("v2.get_jira_issue_template".to_owned(), false),
            ("v2.list_jira_accounts".to_owned(), false),
            ("v2.list_jira_issue_templates".to_owned(), false),
            ("v2.update_jira_issue_template".to_owned(), false),
            ("v2.create_tenancy_config".to_owned(), false),
            ("v2.get_tenancy_configs".to_owned(), false),
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
            ("v2.create_tag_indexing_rule".to_owned(), false),
            ("v2.create_tag_indexing_rule_exemption".to_owned(), false),
            ("v2.delete_tag_indexing_rule".to_owned(), false),
            ("v2.delete_tag_indexing_rule_exemption".to_owned(), false),
            ("v2.get_tag_indexing_rule".to_owned(), false),
            ("v2.get_tag_indexing_rule_exemption".to_owned(), false),
            ("v2.list_tag_indexing_rules".to_owned(), false),
            ("v2.list_tag_indexing_rules_for_metric".to_owned(), false),
            ("v2.reorder_tag_indexing_rules".to_owned(), false),
            ("v2.update_tag_indexing_rule".to_owned(), false),
            ("v2.delete_model_lab_run".to_owned(), false),
            ("v2.get_model_lab_artifact_content".to_owned(), false),
            ("v2.get_model_lab_project".to_owned(), false),
            ("v2.get_model_lab_run".to_owned(), false),
            ("v2.list_model_lab_project_artifacts".to_owned(), false),
            ("v2.list_model_lab_project_facet_keys".to_owned(), false),
            ("v2.list_model_lab_project_facet_values".to_owned(), false),
            ("v2.list_model_lab_projects".to_owned(), false),
            ("v2.list_model_lab_run_artifacts".to_owned(), false),
            ("v2.list_model_lab_run_facet_keys".to_owned(), false),
            ("v2.list_model_lab_run_facet_values".to_owned(), false),
            ("v2.list_model_lab_runs".to_owned(), false),
            ("v2.pin_model_lab_run".to_owned(), false),
            ("v2.star_model_lab_project".to_owned(), false),
            ("v2.unpin_model_lab_run".to_owned(), false),
            ("v2.unstar_model_lab_project".to_owned(), false),
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
            ("v2.list_network_health_insights".to_owned(), false),
            ("v2.delete_scopes_restriction".to_owned(), false),
            ("v2.get_o_auth2_well_known_sites".to_owned(), false),
            ("v2.get_scopes_restriction".to_owned(), false),
            ("v2.register_o_auth_client".to_owned(), false),
            ("v2.upsert_scopes_restriction".to_owned(), false),
            ("v2.disable_customer_org".to_owned(), false),
            ("v2.bulk_update_org_group_memberships".to_owned(), false),
            ("v2.create_org_group".to_owned(), false),
            ("v2.create_org_group_policy".to_owned(), false),
            ("v2.create_org_group_policy_override".to_owned(), false),
            ("v2.delete_org_group".to_owned(), false),
            ("v2.delete_org_group_policy".to_owned(), false),
            ("v2.delete_org_group_policy_override".to_owned(), false),
            ("v2.get_org_group".to_owned(), false),
            ("v2.get_org_group_membership".to_owned(), false),
            ("v2.get_org_group_policy".to_owned(), false),
            ("v2.get_org_group_policy_override".to_owned(), false),
            ("v2.list_org_group_memberships".to_owned(), false),
            ("v2.list_org_group_policies".to_owned(), false),
            ("v2.list_org_group_policy_configs".to_owned(), false),
            ("v2.list_org_group_policy_overrides".to_owned(), false),
            ("v2.list_org_group_policy_suggestions".to_owned(), false),
            ("v2.list_org_groups".to_owned(), false),
            ("v2.update_org_group".to_owned(), false),
            ("v2.update_org_group_membership".to_owned(), false),
            ("v2.update_org_group_policy".to_owned(), false),
            ("v2.update_org_group_policy_override".to_owned(), false),
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
            ("v2.get_pruned_trace_by_id".to_owned(), false),
            ("v2.get_trace_by_id".to_owned(), false),
            ("v2.get_pup_bump_test".to_owned(), false),
            ("v2.get_asm_service_by_name".to_owned(), false),
            ("v2.get_rum_sdk_config".to_owned(), false),
            ("v2.update_rum_sdk_config".to_owned(), false),
            ("v2.create_report_schedule".to_owned(), false),
            ("v2.patch_report_schedule".to_owned(), false),
            ("v2.delete_sourcemaps".to_owned(), false),
            ("v2.get_service_repository_info".to_owned(), false),
            ("v2.get_sourcemaps".to_owned(), false),
            ("v2.list_sourcemaps".to_owned(), false),
            ("v2.restore_sourcemaps".to_owned(), false),
            ("v2.delete_rum_rate_limit_config".to_owned(), false),
            ("v2.get_rum_rate_limit_config".to_owned(), false),
            ("v2.update_rum_rate_limit_config".to_owned(), false),
            ("v2.query_aggregated_long_tasks".to_owned(), false),
            ("v2.query_aggregated_signals_problems".to_owned(), false),
            ("v2.query_aggregated_waterfall".to_owned(), false),
            ("v2.create_scorecard_outcomes_batch".to_owned(), false),
            ("v2.get_entity_risk_score".to_owned(), false),
            ("v2.list_entity_risk_scores".to_owned(), false),
            ("v2.create_slo_report_job".to_owned(), false),
            ("v2.get_slo_report".to_owned(), false),
            ("v2.get_slo_report_job_status".to_owned(), false),
            ("v2.get_slo_status".to_owned(), false),
            ("v2.create_snapshot".to_owned(), false),
            ("v2.get_spa_recommendations".to_owned(), false),
            ("v2.get_spa_recommendations_with_shard".to_owned(), false),
            ("v2.create_ai_custom_rule".to_owned(), false),
            ("v2.create_ai_custom_rule_revision".to_owned(), false),
            ("v2.create_ai_custom_ruleset".to_owned(), false),
            ("v2.create_ai_memory_violation_result".to_owned(), false),
            ("v2.create_custom_rule".to_owned(), false),
            ("v2.create_custom_rule_revision".to_owned(), false),
            ("v2.create_custom_ruleset".to_owned(), false),
            ("v2.create_sca_resolve_vulnerable_symbols".to_owned(), false),
            ("v2.create_sca_result".to_owned(), false),
            ("v2.create_sca_scan".to_owned(), false),
            ("v2.delete_ai_custom_rule".to_owned(), false),
            ("v2.delete_ai_custom_ruleset".to_owned(), false),
            ("v2.delete_ai_memory_violation_result".to_owned(), false),
            ("v2.delete_custom_rule".to_owned(), false),
            ("v2.delete_custom_ruleset".to_owned(), false),
            ("v2.get_ai_custom_rule".to_owned(), false),
            ("v2.get_ai_custom_rule_revision".to_owned(), false),
            ("v2.get_ai_custom_ruleset".to_owned(), false),
            ("v2.get_custom_rule".to_owned(), false),
            ("v2.get_custom_rule_revision".to_owned(), false),
            ("v2.get_custom_ruleset".to_owned(), false),
            ("v2.get_sca_scan".to_owned(), false),
            ("v2.list_ai_custom_rule_revisions".to_owned(), false),
            ("v2.list_ai_custom_rulesets".to_owned(), false),
            ("v2.list_ai_memory_violation_results".to_owned(), false),
            ("v2.list_ai_prompts".to_owned(), false),
            ("v2.list_custom_rule_revisions".to_owned(), false),
            ("v2.list_custom_rulesets".to_owned(), false),
            ("v2.list_sca_licenses".to_owned(), false),
            ("v2.revert_custom_rule_revision".to_owned(), false),
            ("v2.update_ai_custom_ruleset".to_owned(), false),
            ("v2.update_custom_ruleset".to_owned(), false),
            ("v2.create_tag_policy".to_owned(), false),
            ("v2.delete_tag_policy".to_owned(), false),
            ("v2.get_tag_policy".to_owned(), false),
            ("v2.get_tag_policy_score".to_owned(), false),
            ("v2.list_tag_policies".to_owned(), false),
            ("v2.update_tag_policy".to_owned(), false),
            ("v2.add_member_team".to_owned(), false),
            ("v2.list_member_teams".to_owned(), false),
            ("v2.remove_member_team".to_owned(), false),
            ("v2.create_web_integration_account".to_owned(), false),
            ("v2.delete_web_integration_account".to_owned(), false),
            ("v2.get_web_integration_account".to_owned(), false),
            ("v2.list_web_integration_accounts".to_owned(), false),
            ("v2.update_web_integration_account".to_owned(), false),
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
                                "uk1.datadoghq.com".into(),
                                "datadoghq.eu".into(),
                                "ddog-gov.com".into(),
                                "us2.ddog-gov.com".into(),
                                "uk1.datadoghq.com".into(),
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
                                        "uk1.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                        "us2.ddog-gov.com".into(),
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
                                        "uk1.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                        "us2.ddog-gov.com".into(),
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
                                        "uk1.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                        "us2.ddog-gov.com".into(),
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
                                        "uk1.datadoghq.com".into(),
                                        "datadoghq.eu".into(),
                                        "ddog-gov.com".into(),
                                        "us2.ddog-gov.com".into(),
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
                        url: "https://{site}".into(),
                        description: "No description provided".into(),
                        variables: HashMap::from([(
                            "site".into(),
                            ServerVariable {
                                description: "The intake domain for the regional site.".into(),
                                default_value: "browser-intake-datadoghq.com".into(),
                                enum_values: vec![
                                    "browser-intake-datadoghq.com".into(),
                                    "browser-intake-us3-datadoghq.com".into(),
                                    "browser-intake-us5-datadoghq.com".into(),
                                    "browser-intake-ap1-datadoghq.com".into(),
                                    "browser-intake-ap2-datadoghq.com".into(),
                                    "browser-intake-datadoghq.eu".into(),
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
                                    default_value: "api".into(),
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
