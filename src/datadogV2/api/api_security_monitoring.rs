// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetFindingOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_finding`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetFindingOptionalParams {
    /// Return the finding for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
}

impl GetFindingOptionalParams {
    /// Return the finding for a given snapshot of time (Unix ms).
    pub fn snapshot_timestamp(mut self, value: i64) -> Self {
        self.snapshot_timestamp = Some(value);
        self
    }
}

/// GetSBOMOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_sbom`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetSBOMOptionalParams {
    /// The container image `repo_digest` for the SBOM request. When the requested asset type is 'Image', this filter is mandatory.
    pub filter_repo_digest: Option<String>,
}

impl GetSBOMOptionalParams {
    /// The container image `repo_digest` for the SBOM request. When the requested asset type is 'Image', this filter is mandatory.
    pub fn filter_repo_digest(mut self, value: String) -> Self {
        self.filter_repo_digest = Some(value);
        self
    }
}

/// ListFindingsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_findings`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFindingsOptionalParams {
    /// Limit the number of findings returned. Must be <= 1000.
    pub page_limit: Option<i64>,
    /// Return findings for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
    /// Return the next page of findings pointed to by the cursor.
    pub page_cursor: Option<String>,
    /// Return findings that have these associated tags (repeatable).
    pub filter_tags: Option<String>,
    /// Return findings that have changed from pass to fail or vice versa on a specified date (Unix ms) or date range (using comparison operators).
    pub filter_evaluation_changed_at: Option<String>,
    /// Set to `true` to return findings that are muted. Set to `false` to return unmuted findings.
    pub filter_muted: Option<bool>,
    /// Return findings for the specified rule ID.
    pub filter_rule_id: Option<String>,
    /// Return findings for the specified rule.
    pub filter_rule_name: Option<String>,
    /// Return only findings for the specified resource type.
    pub filter_resource_type: Option<String>,
    /// Return findings that were found on a specified date (Unix ms) or date range (using comparison operators).
    pub filter_discovery_timestamp: Option<String>,
    /// Return only `pass` or `fail` findings.
    pub filter_evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// Return only findings with the specified status.
    pub filter_status: Option<crate::datadogV2::model::FindingStatus>,
    /// Return findings that match the selected vulnerability types (repeatable).
    pub filter_vulnerability_type: Option<Vec<crate::datadogV2::model::FindingVulnerabilityType>>,
}

impl ListFindingsOptionalParams {
    /// Limit the number of findings returned. Must be <= 1000.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Return findings for a given snapshot of time (Unix ms).
    pub fn snapshot_timestamp(mut self, value: i64) -> Self {
        self.snapshot_timestamp = Some(value);
        self
    }
    /// Return the next page of findings pointed to by the cursor.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Return findings that have these associated tags (repeatable).
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Return findings that have changed from pass to fail or vice versa on a specified date (Unix ms) or date range (using comparison operators).
    pub fn filter_evaluation_changed_at(mut self, value: String) -> Self {
        self.filter_evaluation_changed_at = Some(value);
        self
    }
    /// Set to `true` to return findings that are muted. Set to `false` to return unmuted findings.
    pub fn filter_muted(mut self, value: bool) -> Self {
        self.filter_muted = Some(value);
        self
    }
    /// Return findings for the specified rule ID.
    pub fn filter_rule_id(mut self, value: String) -> Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Return findings for the specified rule.
    pub fn filter_rule_name(mut self, value: String) -> Self {
        self.filter_rule_name = Some(value);
        self
    }
    /// Return only findings for the specified resource type.
    pub fn filter_resource_type(mut self, value: String) -> Self {
        self.filter_resource_type = Some(value);
        self
    }
    /// Return findings that were found on a specified date (Unix ms) or date range (using comparison operators).
    pub fn filter_discovery_timestamp(mut self, value: String) -> Self {
        self.filter_discovery_timestamp = Some(value);
        self
    }
    /// Return only `pass` or `fail` findings.
    pub fn filter_evaluation(mut self, value: crate::datadogV2::model::FindingEvaluation) -> Self {
        self.filter_evaluation = Some(value);
        self
    }
    /// Return only findings with the specified status.
    pub fn filter_status(mut self, value: crate::datadogV2::model::FindingStatus) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Return findings that match the selected vulnerability types (repeatable).
    pub fn filter_vulnerability_type(
        mut self,
        value: Vec<crate::datadogV2::model::FindingVulnerabilityType>,
    ) -> Self {
        self.filter_vulnerability_type = Some(value);
        self
    }
}

/// ListHistoricalJobsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_historical_jobs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListHistoricalJobsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// The order of the jobs in results.
    pub sort: Option<String>,
    /// Query used to filter items from the fetched list.
    pub filter_query: Option<String>,
}

impl ListHistoricalJobsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// The order of the jobs in results.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Query used to filter items from the fetched list.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
}

/// ListSecurityMonitoringRulesOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSecurityMonitoringRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
}

impl ListSecurityMonitoringRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// ListSecurityMonitoringSignalsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_signals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSecurityMonitoringSignalsOptionalParams {
    /// The search query for security signals.
    pub filter_query: Option<String>,
    /// The minimum timestamp for requested security signals.
    pub filter_from: Option<chrono::DateTime<chrono::Utc>>,
    /// The maximum timestamp for requested security signals.
    pub filter_to: Option<chrono::DateTime<chrono::Utc>>,
    /// The order of the security signals in results.
    pub sort: Option<crate::datadogV2::model::SecurityMonitoringSignalsSort>,
    /// A list of results using the cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// The maximum number of security signals in the response.
    pub page_limit: Option<i32>,
}

impl ListSecurityMonitoringSignalsOptionalParams {
    /// The search query for security signals.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// The minimum timestamp for requested security signals.
    pub fn filter_from(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// The maximum timestamp for requested security signals.
    pub fn filter_to(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// The order of the security signals in results.
    pub fn sort(mut self, value: crate::datadogV2::model::SecurityMonitoringSignalsSort) -> Self {
        self.sort = Some(value);
        self
    }
    /// A list of results using the cursor provided in the previous query.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// The maximum number of security signals in the response.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListVulnerabilitiesOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_vulnerabilities`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListVulnerabilitiesOptionalParams {
    /// Its value must come from the `links` section of the response of the first request. Do not manually edit it.
    pub page_token: Option<String>,
    /// The page number to be retrieved. It should be equal or greater than `1`
    pub page_number: Option<i64>,
    /// Filter by vulnerability type.
    pub filter_type: Option<crate::datadogV2::model::VulnerabilityType>,
    /// Filter by vulnerability base (i.e. from the original advisory) severity score.
    pub filter_cvss_base_score_op: Option<f64>,
    /// Filter by vulnerability base severity.
    pub filter_cvss_base_severity: Option<crate::datadogV2::model::VulnerabilitySeverity>,
    /// Filter by vulnerability base CVSS vector.
    pub filter_cvss_base_vector: Option<String>,
    /// Filter by vulnerability Datadog severity score.
    pub filter_cvss_datadog_score_op: Option<f64>,
    /// Filter by vulnerability Datadog severity.
    pub filter_cvss_datadog_severity: Option<crate::datadogV2::model::VulnerabilitySeverity>,
    /// Filter by vulnerability Datadog CVSS vector.
    pub filter_cvss_datadog_vector: Option<String>,
    /// Filter by the status of the vulnerability.
    pub filter_status: Option<crate::datadogV2::model::VulnerabilityStatus>,
    /// Filter by the tool of the vulnerability.
    pub filter_tool: Option<crate::datadogV2::model::VulnerabilityTool>,
    /// Filter by library name.
    pub filter_library_name: Option<String>,
    /// Filter by library version.
    pub filter_library_version: Option<String>,
    /// Filter by advisory ID.
    pub filter_advisory_id: Option<String>,
    /// Filter by exploitation probability.
    pub filter_risks_exploitation_probability: Option<bool>,
    /// Filter by POC exploit availability.
    pub filter_risks_poc_exploit_available: Option<bool>,
    /// Filter by public exploit availability.
    pub filter_risks_exploit_available: Option<bool>,
    /// Filter by vulnerability [EPSS](<https://www.first.org/epss/>) severity score.
    pub filter_risks_epss_score_op: Option<f64>,
    /// Filter by vulnerability [EPSS](<https://www.first.org/epss/>) severity.
    pub filter_risks_epss_severity: Option<crate::datadogV2::model::VulnerabilitySeverity>,
    /// Filter by language.
    pub filter_language: Option<String>,
    /// Filter by ecosystem.
    pub filter_ecosystem: Option<crate::datadogV2::model::VulnerabilityEcosystem>,
    /// Filter by vulnerability location.
    pub filter_code_location_location: Option<String>,
    /// Filter by vulnerability file path.
    pub filter_code_location_file_path: Option<String>,
    /// Filter by method.
    pub filter_code_location_method: Option<String>,
    /// Filter by fix availability.
    pub filter_fix_available: Option<bool>,
    /// Filter by vulnerability `repo_digest` (when the vulnerability is related to `Image` asset).
    pub filter_repo_digests: Option<String>,
    /// Filter by asset name.
    pub filter_asset_name: Option<String>,
    /// Filter by asset type.
    pub filter_asset_type: Option<crate::datadogV2::model::AssetType>,
    /// Filter by the first version of the asset this vulnerability has been detected on.
    pub filter_asset_version_first: Option<String>,
    /// Filter by the last version of the asset this vulnerability has been detected on.
    pub filter_asset_version_last: Option<String>,
    /// Filter by the repository url associated to the asset.
    pub filter_asset_repository_url: Option<String>,
    /// Filter whether the asset is in production or not.
    pub filter_asset_risks_in_production: Option<bool>,
    /// Filter whether the asset is under attack or not.
    pub filter_asset_risks_under_attack: Option<bool>,
    /// Filter whether the asset is publicly accessible or not.
    pub filter_asset_risks_is_publicly_accessible: Option<bool>,
    /// Filter whether the asset is publicly accessible or not.
    pub filter_asset_risks_has_privileged_access: Option<bool>,
    /// Filter whether the asset  has access to sensitive data or not.
    pub filter_asset_risks_has_access_to_sensitive_data: Option<bool>,
    /// Filter by asset environments.
    pub filter_asset_environments: Option<String>,
    /// Filter by asset architecture.
    pub filter_asset_arch: Option<String>,
    /// Filter by asset operating system name.
    pub filter_asset_operating_system_name: Option<String>,
    /// Filter by asset operating system version.
    pub filter_asset_operating_system_version: Option<String>,
}

impl ListVulnerabilitiesOptionalParams {
    /// Its value must come from the `links` section of the response of the first request. Do not manually edit it.
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
    /// The page number to be retrieved. It should be equal or greater than `1`
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by vulnerability type.
    pub fn filter_type(mut self, value: crate::datadogV2::model::VulnerabilityType) -> Self {
        self.filter_type = Some(value);
        self
    }
    /// Filter by vulnerability base (i.e. from the original advisory) severity score.
    pub fn filter_cvss_base_score_op(mut self, value: f64) -> Self {
        self.filter_cvss_base_score_op = Some(value);
        self
    }
    /// Filter by vulnerability base severity.
    pub fn filter_cvss_base_severity(
        mut self,
        value: crate::datadogV2::model::VulnerabilitySeverity,
    ) -> Self {
        self.filter_cvss_base_severity = Some(value);
        self
    }
    /// Filter by vulnerability base CVSS vector.
    pub fn filter_cvss_base_vector(mut self, value: String) -> Self {
        self.filter_cvss_base_vector = Some(value);
        self
    }
    /// Filter by vulnerability Datadog severity score.
    pub fn filter_cvss_datadog_score_op(mut self, value: f64) -> Self {
        self.filter_cvss_datadog_score_op = Some(value);
        self
    }
    /// Filter by vulnerability Datadog severity.
    pub fn filter_cvss_datadog_severity(
        mut self,
        value: crate::datadogV2::model::VulnerabilitySeverity,
    ) -> Self {
        self.filter_cvss_datadog_severity = Some(value);
        self
    }
    /// Filter by vulnerability Datadog CVSS vector.
    pub fn filter_cvss_datadog_vector(mut self, value: String) -> Self {
        self.filter_cvss_datadog_vector = Some(value);
        self
    }
    /// Filter by the status of the vulnerability.
    pub fn filter_status(mut self, value: crate::datadogV2::model::VulnerabilityStatus) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Filter by the tool of the vulnerability.
    pub fn filter_tool(mut self, value: crate::datadogV2::model::VulnerabilityTool) -> Self {
        self.filter_tool = Some(value);
        self
    }
    /// Filter by library name.
    pub fn filter_library_name(mut self, value: String) -> Self {
        self.filter_library_name = Some(value);
        self
    }
    /// Filter by library version.
    pub fn filter_library_version(mut self, value: String) -> Self {
        self.filter_library_version = Some(value);
        self
    }
    /// Filter by advisory ID.
    pub fn filter_advisory_id(mut self, value: String) -> Self {
        self.filter_advisory_id = Some(value);
        self
    }
    /// Filter by exploitation probability.
    pub fn filter_risks_exploitation_probability(mut self, value: bool) -> Self {
        self.filter_risks_exploitation_probability = Some(value);
        self
    }
    /// Filter by POC exploit availability.
    pub fn filter_risks_poc_exploit_available(mut self, value: bool) -> Self {
        self.filter_risks_poc_exploit_available = Some(value);
        self
    }
    /// Filter by public exploit availability.
    pub fn filter_risks_exploit_available(mut self, value: bool) -> Self {
        self.filter_risks_exploit_available = Some(value);
        self
    }
    /// Filter by vulnerability [EPSS](<https://www.first.org/epss/>) severity score.
    pub fn filter_risks_epss_score_op(mut self, value: f64) -> Self {
        self.filter_risks_epss_score_op = Some(value);
        self
    }
    /// Filter by vulnerability [EPSS](<https://www.first.org/epss/>) severity.
    pub fn filter_risks_epss_severity(
        mut self,
        value: crate::datadogV2::model::VulnerabilitySeverity,
    ) -> Self {
        self.filter_risks_epss_severity = Some(value);
        self
    }
    /// Filter by language.
    pub fn filter_language(mut self, value: String) -> Self {
        self.filter_language = Some(value);
        self
    }
    /// Filter by ecosystem.
    pub fn filter_ecosystem(
        mut self,
        value: crate::datadogV2::model::VulnerabilityEcosystem,
    ) -> Self {
        self.filter_ecosystem = Some(value);
        self
    }
    /// Filter by vulnerability location.
    pub fn filter_code_location_location(mut self, value: String) -> Self {
        self.filter_code_location_location = Some(value);
        self
    }
    /// Filter by vulnerability file path.
    pub fn filter_code_location_file_path(mut self, value: String) -> Self {
        self.filter_code_location_file_path = Some(value);
        self
    }
    /// Filter by method.
    pub fn filter_code_location_method(mut self, value: String) -> Self {
        self.filter_code_location_method = Some(value);
        self
    }
    /// Filter by fix availability.
    pub fn filter_fix_available(mut self, value: bool) -> Self {
        self.filter_fix_available = Some(value);
        self
    }
    /// Filter by vulnerability `repo_digest` (when the vulnerability is related to `Image` asset).
    pub fn filter_repo_digests(mut self, value: String) -> Self {
        self.filter_repo_digests = Some(value);
        self
    }
    /// Filter by asset name.
    pub fn filter_asset_name(mut self, value: String) -> Self {
        self.filter_asset_name = Some(value);
        self
    }
    /// Filter by asset type.
    pub fn filter_asset_type(mut self, value: crate::datadogV2::model::AssetType) -> Self {
        self.filter_asset_type = Some(value);
        self
    }
    /// Filter by the first version of the asset this vulnerability has been detected on.
    pub fn filter_asset_version_first(mut self, value: String) -> Self {
        self.filter_asset_version_first = Some(value);
        self
    }
    /// Filter by the last version of the asset this vulnerability has been detected on.
    pub fn filter_asset_version_last(mut self, value: String) -> Self {
        self.filter_asset_version_last = Some(value);
        self
    }
    /// Filter by the repository url associated to the asset.
    pub fn filter_asset_repository_url(mut self, value: String) -> Self {
        self.filter_asset_repository_url = Some(value);
        self
    }
    /// Filter whether the asset is in production or not.
    pub fn filter_asset_risks_in_production(mut self, value: bool) -> Self {
        self.filter_asset_risks_in_production = Some(value);
        self
    }
    /// Filter whether the asset is under attack or not.
    pub fn filter_asset_risks_under_attack(mut self, value: bool) -> Self {
        self.filter_asset_risks_under_attack = Some(value);
        self
    }
    /// Filter whether the asset is publicly accessible or not.
    pub fn filter_asset_risks_is_publicly_accessible(mut self, value: bool) -> Self {
        self.filter_asset_risks_is_publicly_accessible = Some(value);
        self
    }
    /// Filter whether the asset is publicly accessible or not.
    pub fn filter_asset_risks_has_privileged_access(mut self, value: bool) -> Self {
        self.filter_asset_risks_has_privileged_access = Some(value);
        self
    }
    /// Filter whether the asset  has access to sensitive data or not.
    pub fn filter_asset_risks_has_access_to_sensitive_data(mut self, value: bool) -> Self {
        self.filter_asset_risks_has_access_to_sensitive_data = Some(value);
        self
    }
    /// Filter by asset environments.
    pub fn filter_asset_environments(mut self, value: String) -> Self {
        self.filter_asset_environments = Some(value);
        self
    }
    /// Filter by asset architecture.
    pub fn filter_asset_arch(mut self, value: String) -> Self {
        self.filter_asset_arch = Some(value);
        self
    }
    /// Filter by asset operating system name.
    pub fn filter_asset_operating_system_name(mut self, value: String) -> Self {
        self.filter_asset_operating_system_name = Some(value);
        self
    }
    /// Filter by asset operating system version.
    pub fn filter_asset_operating_system_version(mut self, value: String) -> Self {
        self.filter_asset_operating_system_version = Some(value);
        self
    }
}

/// ListVulnerableAssetsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_vulnerable_assets`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListVulnerableAssetsOptionalParams {
    /// Its value must come from the `links` section of the response of the first request. Do not manually edit it.
    pub page_token: Option<String>,
    /// The page number to be retrieved. It should be equal or greater than `1`
    pub page_number: Option<i64>,
    /// Filter by name.
    pub filter_name: Option<String>,
    /// Filter by type.
    pub filter_type: Option<crate::datadogV2::model::AssetType>,
    /// Filter by the first version of the asset since it has been vulnerable.
    pub filter_version_first: Option<String>,
    /// Filter by the last detected version of the asset.
    pub filter_version_last: Option<String>,
    /// Filter by the repository url associated to the asset.
    pub filter_repository_url: Option<String>,
    /// Filter whether the asset is in production or not.
    pub filter_risks_in_production: Option<bool>,
    /// Filter whether the asset (Service) is under attack or not.
    pub filter_risks_under_attack: Option<bool>,
    /// Filter whether the asset (Host) is publicly accessible or not.
    pub filter_risks_is_publicly_accessible: Option<bool>,
    /// Filter whether the asset (Host) has privileged access or not.
    pub filter_risks_has_privileged_access: Option<bool>,
    /// Filter whether the asset (Host)  has access to sensitive data or not.
    pub filter_risks_has_access_to_sensitive_data: Option<bool>,
    /// Filter by environment.
    pub filter_environments: Option<String>,
    /// Filter by architecture.
    pub filter_arch: Option<String>,
    /// Filter by operating system name.
    pub filter_operating_system_name: Option<String>,
    /// Filter by operating system version.
    pub filter_operating_system_version: Option<String>,
}

impl ListVulnerableAssetsOptionalParams {
    /// Its value must come from the `links` section of the response of the first request. Do not manually edit it.
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
    /// The page number to be retrieved. It should be equal or greater than `1`
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter by type.
    pub fn filter_type(mut self, value: crate::datadogV2::model::AssetType) -> Self {
        self.filter_type = Some(value);
        self
    }
    /// Filter by the first version of the asset since it has been vulnerable.
    pub fn filter_version_first(mut self, value: String) -> Self {
        self.filter_version_first = Some(value);
        self
    }
    /// Filter by the last detected version of the asset.
    pub fn filter_version_last(mut self, value: String) -> Self {
        self.filter_version_last = Some(value);
        self
    }
    /// Filter by the repository url associated to the asset.
    pub fn filter_repository_url(mut self, value: String) -> Self {
        self.filter_repository_url = Some(value);
        self
    }
    /// Filter whether the asset is in production or not.
    pub fn filter_risks_in_production(mut self, value: bool) -> Self {
        self.filter_risks_in_production = Some(value);
        self
    }
    /// Filter whether the asset (Service) is under attack or not.
    pub fn filter_risks_under_attack(mut self, value: bool) -> Self {
        self.filter_risks_under_attack = Some(value);
        self
    }
    /// Filter whether the asset (Host) is publicly accessible or not.
    pub fn filter_risks_is_publicly_accessible(mut self, value: bool) -> Self {
        self.filter_risks_is_publicly_accessible = Some(value);
        self
    }
    /// Filter whether the asset (Host) has privileged access or not.
    pub fn filter_risks_has_privileged_access(mut self, value: bool) -> Self {
        self.filter_risks_has_privileged_access = Some(value);
        self
    }
    /// Filter whether the asset (Host)  has access to sensitive data or not.
    pub fn filter_risks_has_access_to_sensitive_data(mut self, value: bool) -> Self {
        self.filter_risks_has_access_to_sensitive_data = Some(value);
        self
    }
    /// Filter by environment.
    pub fn filter_environments(mut self, value: String) -> Self {
        self.filter_environments = Some(value);
        self
    }
    /// Filter by architecture.
    pub fn filter_arch(mut self, value: String) -> Self {
        self.filter_arch = Some(value);
        self
    }
    /// Filter by operating system name.
    pub fn filter_operating_system_name(mut self, value: String) -> Self {
        self.filter_operating_system_name = Some(value);
        self
    }
    /// Filter by operating system version.
    pub fn filter_operating_system_version(mut self, value: String) -> Self {
        self.filter_operating_system_version = Some(value);
        self
    }
}

/// SearchSecurityMonitoringSignalsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::search_security_monitoring_signals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchSecurityMonitoringSignalsOptionalParams {
    pub body: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequest>,
}

impl SearchSecurityMonitoringSignalsOptionalParams {
    pub fn body(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequest,
    ) -> Self {
        self.body = Some(value);
        self
    }
}

/// CancelHistoricalJobError is a struct for typed errors of method [`SecurityMonitoringAPI::cancel_historical_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelHistoricalJobError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ConvertExistingSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::convert_existing_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertExistingSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ConvertJobResultToSignalError is a struct for typed errors of method [`SecurityMonitoringAPI::convert_job_result_to_signal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertJobResultToSignalError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ConvertSecurityMonitoringRuleFromJSONToTerraformError is a struct for typed errors of method [`SecurityMonitoringAPI::convert_security_monitoring_rule_from_json_to_terraform`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertSecurityMonitoringRuleFromJSONToTerraformError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateInboxRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_inbox_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInboxRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMuteRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_mute_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMuteRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::create_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSecurityMonitoringSuppressionError is a struct for typed errors of method [`SecurityMonitoringAPI::create_security_monitoring_suppression`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityMonitoringSuppressionError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSignalNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_signal_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSignalNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateVulnerabilityNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_vulnerability_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateVulnerabilityNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteHistoricalJobError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_historical_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHistoricalJobError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteInboxRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_inbox_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInboxRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteMuteRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_mute_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMuteRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityMonitoringSuppressionError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_security_monitoring_suppression`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityMonitoringSuppressionError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSignalNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_signal_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSignalNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteVulnerabilityNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_vulnerability_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteVulnerabilityNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_assignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalIncidentsError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_incidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalIncidentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalStateError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFindingError is a struct for typed errors of method [`SecurityMonitoringAPI::get_finding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFindingError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHistoricalJobError is a struct for typed errors of method [`SecurityMonitoringAPI::get_historical_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoricalJobError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetInboxRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_inbox_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetInboxRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::get_inbox_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMuteRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_mute_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMuteRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMuteRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::get_mute_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMuteRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSBOMError is a struct for typed errors of method [`SecurityMonitoringAPI::get_sbom`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSBOMError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringSignalError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_monitoring_signal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringSignalError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringSuppressionError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_monitoring_suppression`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringSuppressionError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSignalNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_signal_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSignalNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSignalNotificationRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::get_signal_notification_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSignalNotificationRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetVulnerabilityNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_vulnerability_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVulnerabilityNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetVulnerabilityNotificationRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::get_vulnerability_notification_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVulnerabilityNotificationRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFindingsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_findings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFindingsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListHistoricalJobsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_historical_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHistoricalJobsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityFiltersError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityFiltersError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_monitoring_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringSignalsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_monitoring_signals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringSignalsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringSuppressionsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_monitoring_suppressions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringSuppressionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListVulnerabilitiesError is a struct for typed errors of method [`SecurityMonitoringAPI::list_vulnerabilities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVulnerabilitiesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListVulnerableAssetsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_vulnerable_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVulnerableAssetsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// MuteFindingsError is a struct for typed errors of method [`SecurityMonitoringAPI::mute_findings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MuteFindingsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchInboxRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::patch_inbox_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchInboxRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchMuteRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::patch_mute_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchMuteRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchSignalNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::patch_signal_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchSignalNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchVulnerabilityNotificationRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::patch_vulnerability_notification_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchVulnerabilityNotificationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ReorderInboxRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::reorder_inbox_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderInboxRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ReorderMuteRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::reorder_mute_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderMuteRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RunHistoricalJobError is a struct for typed errors of method [`SecurityMonitoringAPI::run_historical_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunHistoricalJobError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchSecurityMonitoringSignalsError is a struct for typed errors of method [`SecurityMonitoringAPI::search_security_monitoring_signals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSecurityMonitoringSignalsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// TestExistingSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::test_existing_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestExistingSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// TestSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::test_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateInboxRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::update_inbox_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateInboxRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMuteRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::update_mute_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMuteRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::update_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::update_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityMonitoringSuppressionError is a struct for typed errors of method [`SecurityMonitoringAPI::update_security_monitoring_suppression`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityMonitoringSuppressionError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::validate_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateSecurityMonitoringRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create and manage your security rules, signals, filters, and more. See the [Datadog Security page](<https://docs.datadoghq.com/security/>) for more information.
#[derive(Debug, Clone)]
pub struct SecurityMonitoringAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for SecurityMonitoringAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl SecurityMonitoringAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Cancel a historical job.
    pub async fn cancel_historical_job(
        &self,
        job_id: String,
    ) -> Result<(), datadog::Error<CancelHistoricalJobError>> {
        match self.cancel_historical_job_with_http_info(job_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Cancel a historical job.
    pub async fn cancel_historical_job_with_http_info(
        &self,
        job_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<CancelHistoricalJobError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.cancel_historical_job";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.cancel_historical_job' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs/{job_id}/cancel",
            local_configuration.get_operation_host(operation_id),
            job_id = datadog::urlencode(job_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<CancelHistoricalJobError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Convert an existing rule from JSON to Terraform for datadog provider
    /// resource datadog_security_monitoring_rule.
    pub async fn convert_existing_security_monitoring_rule(
        &self,
        rule_id: String,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleConvertResponse,
        datadog::Error<ConvertExistingSecurityMonitoringRuleError>,
    > {
        match self
            .convert_existing_security_monitoring_rule_with_http_info(rule_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Convert an existing rule from JSON to Terraform for datadog provider
    /// resource datadog_security_monitoring_rule.
    pub async fn convert_existing_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleConvertResponse>,
        datadog::Error<ConvertExistingSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.convert_existing_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}/convert",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringRuleConvertResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ConvertExistingSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Convert a job result to a signal.
    pub async fn convert_job_result_to_signal(
        &self,
        body: crate::datadogV2::model::ConvertJobResultsToSignalsRequest,
    ) -> Result<(), datadog::Error<ConvertJobResultToSignalError>> {
        match self.convert_job_result_to_signal_with_http_info(body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Convert a job result to a signal.
    pub async fn convert_job_result_to_signal_with_http_info(
        &self,
        body: crate::datadogV2::model::ConvertJobResultsToSignalsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ConvertJobResultToSignalError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.convert_job_result_to_signal";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.convert_job_result_to_signal' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs/signal_convert",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<ConvertJobResultToSignalError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Convert a rule that doesn't (yet) exist from JSON to Terraform for datadog provider
    /// resource datadog_security_monitoring_rule.
    pub async fn convert_security_monitoring_rule_from_json_to_terraform(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleConvertPayload,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleConvertResponse,
        datadog::Error<ConvertSecurityMonitoringRuleFromJSONToTerraformError>,
    > {
        match self
            .convert_security_monitoring_rule_from_json_to_terraform_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Convert a rule that doesn't (yet) exist from JSON to Terraform for datadog provider
    /// resource datadog_security_monitoring_rule.
    pub async fn convert_security_monitoring_rule_from_json_to_terraform_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleConvertPayload,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleConvertResponse>,
        datadog::Error<ConvertSecurityMonitoringRuleFromJSONToTerraformError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.convert_security_monitoring_rule_from_json_to_terraform";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/convert",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringRuleConvertResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ConvertSecurityMonitoringRuleFromJSONToTerraformError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new inbox rule and return the created rule.
    pub async fn create_inbox_rule(
        &self,
        body: crate::datadogV2::model::CreateInboxRuleParameters,
    ) -> Result<crate::datadogV2::model::InboxRuleResponse, datadog::Error<CreateInboxRuleError>>
    {
        match self.create_inbox_rule_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new inbox rule and return the created rule.
    pub async fn create_inbox_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateInboxRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::InboxRuleResponse>,
        datadog::Error<CreateInboxRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_inbox_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::InboxRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateInboxRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new mute rule and return the created rule.
    pub async fn create_mute_rule(
        &self,
        body: crate::datadogV2::model::CreateMuteRuleParameters,
    ) -> Result<crate::datadogV2::model::MuteRuleResponse, datadog::Error<CreateMuteRuleError>>
    {
        match self.create_mute_rule_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new mute rule and return the created rule.
    pub async fn create_mute_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateMuteRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MuteRuleResponse>,
        datadog::Error<CreateMuteRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_mute_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MuteRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateMuteRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn create_security_filter(
        &self,
        body: crate::datadogV2::model::SecurityFilterCreateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityFilterResponse,
        datadog::Error<CreateSecurityFilterError>,
    > {
        match self.create_security_filter_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn create_security_filter_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityFilterCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        datadog::Error<CreateSecurityFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_security_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityFilterResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a detection rule.
    pub async fn create_security_monitoring_rule(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleCreatePayload,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleResponse,
        datadog::Error<CreateSecurityMonitoringRuleError>,
    > {
        match self
            .create_security_monitoring_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a detection rule.
    pub async fn create_security_monitoring_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleCreatePayload,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        datadog::Error<CreateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new suppression rule.
    pub async fn create_security_monitoring_suppression(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringSuppressionCreateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
        datadog::Error<CreateSecurityMonitoringSuppressionError>,
    > {
        match self
            .create_security_monitoring_suppression_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new suppression rule.
    pub async fn create_security_monitoring_suppression_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringSuppressionCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSuppressionResponse>,
        datadog::Error<CreateSecurityMonitoringSuppressionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_security_monitoring_suppression";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/suppressions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSecurityMonitoringSuppressionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new notification rule for security signals and return the created rule.
    pub async fn create_signal_notification_rule(
        &self,
        body: crate::datadogV2::model::CreateNotificationRuleParameters,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<CreateSignalNotificationRuleError>,
    > {
        match self
            .create_signal_notification_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new notification rule for security signals and return the created rule.
    pub async fn create_signal_notification_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateNotificationRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<CreateSignalNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_signal_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/signals/notification_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSignalNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new notification rule for security vulnerabilities and return the created rule.
    pub async fn create_vulnerability_notification_rule(
        &self,
        body: crate::datadogV2::model::CreateNotificationRuleParameters,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<CreateVulnerabilityNotificationRuleError>,
    > {
        match self
            .create_vulnerability_notification_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new notification rule for security vulnerabilities and return the created rule.
    pub async fn create_vulnerability_notification_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateNotificationRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<CreateVulnerabilityNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_vulnerability_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/notification_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateVulnerabilityNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an existing job.
    pub async fn delete_historical_job(
        &self,
        job_id: String,
    ) -> Result<(), datadog::Error<DeleteHistoricalJobError>> {
        match self.delete_historical_job_with_http_info(job_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing job.
    pub async fn delete_historical_job_with_http_info(
        &self,
        job_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteHistoricalJobError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_historical_job";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_historical_job' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs/{job_id}",
            local_configuration.get_operation_host(operation_id),
            job_id = datadog::urlencode(job_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteHistoricalJobError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an inbox rule
    pub async fn delete_inbox_rule(
        &self,
        inbox_rule_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteInboxRuleError>> {
        match self.delete_inbox_rule_with_http_info(inbox_rule_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an inbox rule
    pub async fn delete_inbox_rule_with_http_info(
        &self,
        inbox_rule_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteInboxRuleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_inbox_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules/{inbox_rule_id}",
            local_configuration.get_operation_host(operation_id),
            inbox_rule_id = datadog::urlencode(inbox_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteInboxRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a mute rule
    pub async fn delete_mute_rule(
        &self,
        mute_rule_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteMuteRuleError>> {
        match self.delete_mute_rule_with_http_info(mute_rule_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a mute rule
    pub async fn delete_mute_rule_with_http_info(
        &self,
        mute_rule_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteMuteRuleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_mute_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules/{mute_rule_id}",
            local_configuration.get_operation_host(operation_id),
            mute_rule_id = datadog::urlencode(mute_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteMuteRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific security filter.
    pub async fn delete_security_filter(
        &self,
        security_filter_id: String,
    ) -> Result<(), datadog::Error<DeleteSecurityFilterError>> {
        match self
            .delete_security_filter_with_http_info(security_filter_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific security filter.
    pub async fn delete_security_filter_with_http_info(
        &self,
        security_filter_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteSecurityFilterError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_security_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.get_operation_host(operation_id),
            security_filter_id = datadog::urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an existing rule. Default rules cannot be deleted.
    pub async fn delete_security_monitoring_rule(
        &self,
        rule_id: String,
    ) -> Result<(), datadog::Error<DeleteSecurityMonitoringRuleError>> {
        match self
            .delete_security_monitoring_rule_with_http_info(rule_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing rule. Default rules cannot be deleted.
    pub async fn delete_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteSecurityMonitoringRuleError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific suppression rule.
    pub async fn delete_security_monitoring_suppression(
        &self,
        suppression_id: String,
    ) -> Result<(), datadog::Error<DeleteSecurityMonitoringSuppressionError>> {
        match self
            .delete_security_monitoring_suppression_with_http_info(suppression_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific suppression rule.
    pub async fn delete_security_monitoring_suppression_with_http_info(
        &self,
        suppression_id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteSecurityMonitoringSuppressionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_security_monitoring_suppression";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/suppressions/{suppression_id}",
            local_configuration.get_operation_host(operation_id),
            suppression_id = datadog::urlencode(suppression_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSecurityMonitoringSuppressionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a notification rule for security signals.
    pub async fn delete_signal_notification_rule(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteSignalNotificationRuleError>> {
        match self
            .delete_signal_notification_rule_with_http_info(id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a notification rule for security signals.
    pub async fn delete_signal_notification_rule_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteSignalNotificationRuleError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_signal_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/signals/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSignalNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a notification rule for security vulnerabilities.
    pub async fn delete_vulnerability_notification_rule(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteVulnerabilityNotificationRuleError>> {
        match self
            .delete_vulnerability_notification_rule_with_http_info(id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a notification rule for security vulnerabilities.
    pub async fn delete_vulnerability_notification_rule_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteVulnerabilityNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_vulnerability_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteVulnerabilityNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalAssigneeUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        datadog::Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        match self
            .edit_security_monitoring_signal_assignee_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalAssigneeUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        >,
        datadog::Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.edit_security_monitoring_signal_assignee";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/assignee",
            local_configuration.get_operation_host(operation_id),
            signal_id = datadog::urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalAssigneeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Change the related incidents for a security signal.
    pub async fn edit_security_monitoring_signal_incidents(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        datadog::Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        match self
            .edit_security_monitoring_signal_incidents_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Change the related incidents for a security signal.
    pub async fn edit_security_monitoring_signal_incidents_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        >,
        datadog::Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.edit_security_monitoring_signal_incidents";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/incidents",
            local_configuration.get_operation_host(operation_id),
            signal_id = datadog::urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalIncidentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        datadog::Error<EditSecurityMonitoringSignalStateError>,
    > {
        match self
            .edit_security_monitoring_signal_state_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
        >,
        datadog::Error<EditSecurityMonitoringSignalStateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.edit_security_monitoring_signal_state";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/state",
            local_configuration.get_operation_host(operation_id),
            signal_id = datadog::urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalStateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns a single finding with message and resource configuration.
    pub async fn get_finding(
        &self,
        finding_id: String,
        params: GetFindingOptionalParams,
    ) -> Result<crate::datadogV2::model::GetFindingResponse, datadog::Error<GetFindingError>> {
        match self.get_finding_with_http_info(finding_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns a single finding with message and resource configuration.
    pub async fn get_finding_with_http_info(
        &self,
        finding_id: String,
        params: GetFindingOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetFindingResponse>,
        datadog::Error<GetFindingError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_finding";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_finding' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let snapshot_timestamp = params.snapshot_timestamp;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings/{finding_id}",
            local_configuration.get_operation_host(operation_id),
            finding_id = datadog::urlencode(finding_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetFindingResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetFindingError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a job's details.
    pub async fn get_historical_job(
        &self,
        job_id: String,
    ) -> Result<crate::datadogV2::model::HistoricalJobResponse, datadog::Error<GetHistoricalJobError>>
    {
        match self.get_historical_job_with_http_info(job_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a job's details.
    pub async fn get_historical_job_with_http_info(
        &self,
        job_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::HistoricalJobResponse>,
        datadog::Error<GetHistoricalJobError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_historical_job";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_historical_job' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs/{job_id}",
            local_configuration.get_operation_host(operation_id),
            job_id = datadog::urlencode(job_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::HistoricalJobResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetHistoricalJobError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of an inbox rule.
    pub async fn get_inbox_rule(
        &self,
        inbox_rule_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::InboxRuleResponse, datadog::Error<GetInboxRuleError>> {
        match self.get_inbox_rule_with_http_info(inbox_rule_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of an inbox rule.
    pub async fn get_inbox_rule_with_http_info(
        &self,
        inbox_rule_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::InboxRuleResponse>,
        datadog::Error<GetInboxRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_inbox_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules/{inbox_rule_id}",
            local_configuration.get_operation_host(operation_id),
            inbox_rule_id = datadog::urlencode(inbox_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::InboxRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetInboxRuleError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the ordered list of inbox rules in the pipeline (first match applies)
    pub async fn get_inbox_rules(
        &self,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<GetInboxRulesError>,
    > {
        match self.get_inbox_rules_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the ordered list of inbox rules in the pipeline (first match applies)
    pub async fn get_inbox_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<GetInboxRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_inbox_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetInboxRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a mute rule.
    pub async fn get_mute_rule(
        &self,
        mute_rule_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::MuteRuleResponse, datadog::Error<GetMuteRuleError>> {
        match self.get_mute_rule_with_http_info(mute_rule_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of a mute rule.
    pub async fn get_mute_rule_with_http_info(
        &self,
        mute_rule_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MuteRuleResponse>,
        datadog::Error<GetMuteRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_mute_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules/{mute_rule_id}",
            local_configuration.get_operation_host(operation_id),
            mute_rule_id = datadog::urlencode(mute_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MuteRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMuteRuleError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the ordered list of mute rules in the pipeline (first match applies)
    pub async fn get_mute_rules(
        &self,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<GetMuteRulesError>,
    > {
        match self.get_mute_rules_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the ordered list of mute rules in the pipeline (first match applies)
    pub async fn get_mute_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<GetMuteRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_mute_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMuteRulesError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single SBOM related to an asset by its type and name.
    ///
    pub async fn get_sbom(
        &self,
        asset_type: crate::datadogV2::model::AssetType,
        filter_asset_name: String,
        params: GetSBOMOptionalParams,
    ) -> Result<crate::datadogV2::model::GetSBOMResponse, datadog::Error<GetSBOMError>> {
        match self
            .get_sbom_with_http_info(asset_type, filter_asset_name, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a single SBOM related to an asset by its type and name.
    ///
    pub async fn get_sbom_with_http_info(
        &self,
        asset_type: crate::datadogV2::model::AssetType,
        filter_asset_name: String,
        params: GetSBOMOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetSBOMResponse>,
        datadog::Error<GetSBOMError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_sbom";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_sbom' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_repo_digest = params.filter_repo_digest;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/sboms/{asset_type}",
            local_configuration.get_operation_host(operation_id),
            asset_type = datadog::urlencode(asset_type.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[asset_name]", &filter_asset_name.to_string())]);
        if let Some(ref local_query_param) = filter_repo_digest {
            local_req_builder =
                local_req_builder.query(&[("filter[repo_digest]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetSBOMResponse>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSBOMError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn get_security_filter(
        &self,
        security_filter_id: String,
    ) -> Result<
        crate::datadogV2::model::SecurityFilterResponse,
        datadog::Error<GetSecurityFilterError>,
    > {
        match self
            .get_security_filter_with_http_info(security_filter_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of a specific security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn get_security_filter_with_http_info(
        &self,
        security_filter_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        datadog::Error<GetSecurityFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_security_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.get_operation_host(operation_id),
            security_filter_id = datadog::urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityFilterResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a rule's details.
    pub async fn get_security_monitoring_rule(
        &self,
        rule_id: String,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleResponse,
        datadog::Error<GetSecurityMonitoringRuleError>,
    > {
        match self
            .get_security_monitoring_rule_with_http_info(rule_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a rule's details.
    pub async fn get_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        datadog::Error<GetSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a signal's details.
    pub async fn get_security_monitoring_signal(
        &self,
        signal_id: String,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalResponse,
        datadog::Error<GetSecurityMonitoringSignalError>,
    > {
        match self
            .get_security_monitoring_signal_with_http_info(signal_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a signal's details.
    pub async fn get_security_monitoring_signal_with_http_info(
        &self,
        signal_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalResponse>,
        datadog::Error<GetSecurityMonitoringSignalError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_security_monitoring_signal";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}",
            local_configuration.get_operation_host(operation_id),
            signal_id = datadog::urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringSignalResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSecurityMonitoringSignalError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific suppression rule.
    pub async fn get_security_monitoring_suppression(
        &self,
        suppression_id: String,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
        datadog::Error<GetSecurityMonitoringSuppressionError>,
    > {
        match self
            .get_security_monitoring_suppression_with_http_info(suppression_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of a specific suppression rule.
    pub async fn get_security_monitoring_suppression_with_http_info(
        &self,
        suppression_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSuppressionResponse>,
        datadog::Error<GetSecurityMonitoringSuppressionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_security_monitoring_suppression";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/suppressions/{suppression_id}",
            local_configuration.get_operation_host(operation_id),
            suppression_id = datadog::urlencode(suppression_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSecurityMonitoringSuppressionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a notification rule for security signals.
    pub async fn get_signal_notification_rule(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<GetSignalNotificationRuleError>,
    > {
        match self.get_signal_notification_rule_with_http_info(id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of a notification rule for security signals.
    pub async fn get_signal_notification_rule_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<GetSignalNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_signal_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/signals/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSignalNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the list of notification rules for security signals.
    pub async fn get_signal_notification_rules(
        &self,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<GetSignalNotificationRulesError>,
    > {
        match self.get_signal_notification_rules_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the list of notification rules for security signals.
    pub async fn get_signal_notification_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<GetSignalNotificationRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_signal_notification_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/signals/notification_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSignalNotificationRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a notification rule for security vulnerabilities.
    pub async fn get_vulnerability_notification_rule(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<GetVulnerabilityNotificationRuleError>,
    > {
        match self
            .get_vulnerability_notification_rule_with_http_info(id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the details of a notification rule for security vulnerabilities.
    pub async fn get_vulnerability_notification_rule_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<GetVulnerabilityNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_vulnerability_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetVulnerabilityNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the list of notification rules for security vulnerabilities.
    pub async fn get_vulnerability_notification_rules(
        &self,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<GetVulnerabilityNotificationRulesError>,
    > {
        match self
            .get_vulnerability_notification_rules_with_http_info()
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns the list of notification rules for security vulnerabilities.
    pub async fn get_vulnerability_notification_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<GetVulnerabilityNotificationRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_vulnerability_notification_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/notification_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetVulnerabilityNotificationRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of findings. These include both misconfigurations and identity risks.
    ///
    /// **Note**: To filter and return only identity risks, add the following query parameter: `?filter[tags]=dd_rule_type:ciem`
    ///
    /// ### Filtering
    ///
    /// Filters can be applied by appending query parameters to the URL.
    ///
    ///   - Using a single filter: `?filter[attribute_key]=attribute_value`
    ///   - Chaining filters: `?filter[attribute_key]=attribute_value&filter[attribute_key]=attribute_value...`
    ///   - Filtering on tags: `?filter[tags]=tag_key:tag_value&filter[tags]=tag_key_2:tag_value_2`
    ///
    /// Here, `attribute_key` can be any of the filter keys described further below.
    ///
    /// Query parameters of type `integer` support comparison operators (`>`, `>=`, `<`, `<=`). This is particularly useful when filtering by `evaluation_changed_at` or `resource_discovery_timestamp`. For example: `?filter[evaluation_changed_at]=>20123123121`.
    ///
    /// You can also use the negation operator on strings. For example, use `filter[resource_type]=-aws*` to filter for any non-AWS resources.
    ///
    /// The operator must come after the equal sign. For example, to filter with the `>=` operator, add the operator after the equal sign: `filter[evaluation_changed_at]=>=1678809373257`.
    ///
    /// Query parameters must be only among the documented ones and with values of correct types. Duplicated query parameters (e.g. `filter[status]=low&filter[status]=info`) are not allowed.
    ///
    /// ### Response
    ///
    /// The response includes an array of finding objects, pagination metadata, and a count of items that match the query.
    ///
    /// Each finding object contains the following:
    ///
    /// - The finding ID that can be used in a `GetFinding` request to retrieve the full finding details.
    /// - Core attributes, including status, evaluation, high-level resource details, muted state, and rule details.
    /// - `evaluation_changed_at` and `resource_discovery_date` time stamps.
    /// - An array of associated tags.
    ///
    pub async fn list_findings(
        &self,
        params: ListFindingsOptionalParams,
    ) -> Result<crate::datadogV2::model::ListFindingsResponse, datadog::Error<ListFindingsError>>
    {
        match self.list_findings_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_findings_with_pagination(
        &self,
        mut params: ListFindingsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::Finding, datadog::Error<ListFindingsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_findings(params.clone()).await?;

                let r = resp.data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(page) = resp.meta.page else { break };
                let Some(cursor) = page.cursor else { break };

                params.page_cursor = Some(cursor);
            }
        }
    }

    /// Get a list of findings. These include both misconfigurations and identity risks.
    ///
    /// **Note**: To filter and return only identity risks, add the following query parameter: `?filter[tags]=dd_rule_type:ciem`
    ///
    /// ### Filtering
    ///
    /// Filters can be applied by appending query parameters to the URL.
    ///
    ///   - Using a single filter: `?filter[attribute_key]=attribute_value`
    ///   - Chaining filters: `?filter[attribute_key]=attribute_value&filter[attribute_key]=attribute_value...`
    ///   - Filtering on tags: `?filter[tags]=tag_key:tag_value&filter[tags]=tag_key_2:tag_value_2`
    ///
    /// Here, `attribute_key` can be any of the filter keys described further below.
    ///
    /// Query parameters of type `integer` support comparison operators (`>`, `>=`, `<`, `<=`). This is particularly useful when filtering by `evaluation_changed_at` or `resource_discovery_timestamp`. For example: `?filter[evaluation_changed_at]=>20123123121`.
    ///
    /// You can also use the negation operator on strings. For example, use `filter[resource_type]=-aws*` to filter for any non-AWS resources.
    ///
    /// The operator must come after the equal sign. For example, to filter with the `>=` operator, add the operator after the equal sign: `filter[evaluation_changed_at]=>=1678809373257`.
    ///
    /// Query parameters must be only among the documented ones and with values of correct types. Duplicated query parameters (e.g. `filter[status]=low&filter[status]=info`) are not allowed.
    ///
    /// ### Response
    ///
    /// The response includes an array of finding objects, pagination metadata, and a count of items that match the query.
    ///
    /// Each finding object contains the following:
    ///
    /// - The finding ID that can be used in a `GetFinding` request to retrieve the full finding details.
    /// - Core attributes, including status, evaluation, high-level resource details, muted state, and rule details.
    /// - `evaluation_changed_at` and `resource_discovery_date` time stamps.
    /// - An array of associated tags.
    ///
    pub async fn list_findings_with_http_info(
        &self,
        params: ListFindingsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListFindingsResponse>,
        datadog::Error<ListFindingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_findings";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_findings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_limit = params.page_limit;
        let snapshot_timestamp = params.snapshot_timestamp;
        let page_cursor = params.page_cursor;
        let filter_tags = params.filter_tags;
        let filter_evaluation_changed_at = params.filter_evaluation_changed_at;
        let filter_muted = params.filter_muted;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_name = params.filter_rule_name;
        let filter_resource_type = params.filter_resource_type;
        let filter_discovery_timestamp = params.filter_discovery_timestamp;
        let filter_evaluation = params.filter_evaluation;
        let filter_status = params.filter_status;
        let filter_vulnerability_type = params.filter_vulnerability_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_evaluation_changed_at {
            local_req_builder = local_req_builder.query(&[(
                "filter[evaluation_changed_at]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_muted {
            local_req_builder =
                local_req_builder.query(&[("filter[muted]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_resource_type {
            local_req_builder = local_req_builder
                .query(&[("filter[resource_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_discovery_timestamp {
            local_req_builder = local_req_builder.query(&[(
                "filter[discovery_timestamp]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_evaluation {
            local_req_builder =
                local_req_builder.query(&[("filter[evaluation]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_vulnerability_type {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[vulnerability_type]", &param.to_string())]);
            }
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListFindingsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListFindingsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List historical jobs.
    pub async fn list_historical_jobs(
        &self,
        params: ListHistoricalJobsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListHistoricalJobsResponse,
        datadog::Error<ListHistoricalJobsError>,
    > {
        match self.list_historical_jobs_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List historical jobs.
    pub async fn list_historical_jobs_with_http_info(
        &self,
        params: ListHistoricalJobsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListHistoricalJobsResponse>,
        datadog::Error<ListHistoricalJobsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_historical_jobs";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_historical_jobs' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter_query = params.filter_query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListHistoricalJobsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListHistoricalJobsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of configured security filters with their definitions.
    pub async fn list_security_filters(
        &self,
    ) -> Result<
        crate::datadogV2::model::SecurityFiltersResponse,
        datadog::Error<ListSecurityFiltersError>,
    > {
        match self.list_security_filters_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured security filters with their definitions.
    pub async fn list_security_filters_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityFiltersResponse>,
        datadog::Error<ListSecurityFiltersError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_security_filters";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityFiltersResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListSecurityFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List rules.
    pub async fn list_security_monitoring_rules(
        &self,
        params: ListSecurityMonitoringRulesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringListRulesResponse,
        datadog::Error<ListSecurityMonitoringRulesError>,
    > {
        match self
            .list_security_monitoring_rules_with_http_info(params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List rules.
    pub async fn list_security_monitoring_rules_with_http_info(
        &self,
        params: ListSecurityMonitoringRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringListRulesResponse>,
        datadog::Error<ListSecurityMonitoringRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_security_monitoring_rules";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringListRulesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListSecurityMonitoringRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// The list endpoint returns security signals that match a search query.
    /// Both this endpoint and the POST endpoint can be used interchangeably when listing
    /// security signals.
    pub async fn list_security_monitoring_signals(
        &self,
        params: ListSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
        datadog::Error<ListSecurityMonitoringSignalsError>,
    > {
        match self
            .list_security_monitoring_signals_with_http_info(params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_security_monitoring_signals_with_pagination(
        &self,
        mut params: ListSecurityMonitoringSignalsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::SecurityMonitoringSignal,
            datadog::Error<ListSecurityMonitoringSignalsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_security_monitoring_signals(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.page_cursor = Some(after);
            }
        }
    }

    /// The list endpoint returns security signals that match a search query.
    /// Both this endpoint and the POST endpoint can be used interchangeably when listing
    /// security signals.
    pub async fn list_security_monitoring_signals_with_http_info(
        &self,
        params: ListSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        datadog::Error<ListSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_security_monitoring_signals";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder = local_req_builder.query(&[(
                "filter[from]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder = local_req_builder.query(&[(
                "filter[to]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListSecurityMonitoringSignalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of all suppression rules.
    pub async fn list_security_monitoring_suppressions(
        &self,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSuppressionsResponse,
        datadog::Error<ListSecurityMonitoringSuppressionsError>,
    > {
        match self
            .list_security_monitoring_suppressions_with_http_info()
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the list of all suppression rules.
    pub async fn list_security_monitoring_suppressions_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSuppressionsResponse>,
        datadog::Error<ListSecurityMonitoringSuppressionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_security_monitoring_suppressions";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/suppressions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSuppressionsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListSecurityMonitoringSuppressionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of vulnerabilities.
    ///
    /// ### Pagination
    ///
    /// Pagination is enabled by default in both `vulnerabilities` and `assets`. The size of the page varies depending on the endpoint and cannot be modified. To automate the request of the next page, you can use the links section in the response.
    ///
    /// This endpoint will return paginated responses. The pages are stored in the links section of the response:
    ///
    /// ```JSON
    /// {
    ///   "data": [...],
    ///   "meta": {...},
    ///   "links": {
    ///     "self": "<https://.../api/v2/security/vulnerabilities",>
    ///     "first": "<https://.../api/v2/security/vulnerabilities?page[number]=1&page[token]=abc",>
    ///     "last": "<https://.../api/v2/security/vulnerabilities?page[number]=43&page[token]=abc",>
    ///     "next": "<https://.../api/v2/security/vulnerabilities?page[number]=2&page[token]=abc">
    ///   }
    /// }
    /// ```
    ///
    ///
    /// - `links.previous` is empty if the first page is requested.
    /// - `links.next` is empty if the last page is requested.
    ///
    /// #### Token
    ///
    /// Vulnerabilities can be created, updated or deleted at any point in time.
    ///
    /// Upon the first request, a token is created to ensure consistency across subsequent paginated requests.
    ///
    /// A token is valid only for 24 hours.
    ///
    /// #### First request
    ///
    /// We consider a request to be the first request when there is no `page[token]` parameter.
    ///
    /// The response of this first request contains the newly created token in the `links` section.
    ///
    /// This token can then be used in the subsequent paginated requests.
    ///
    /// #### Subsequent requests
    ///
    /// Any request containing valid `page[token]` and `page[number]` parameters will be considered a subsequent request.
    ///
    /// If the `token` is invalid, a `404` response will be returned.
    ///
    /// If the page `number` is invalid, a `400` response will be returned.
    ///
    /// ### Filtering
    ///
    /// The request can include some filter parameters to filter the data to be retrieved. The format of the filter parameters follows the [JSON:API format](<https://jsonapi.org/format/#fetching-filtering>): `filter[$prop_name]`, where `prop_name` is the property name in the entity being filtered by.
    ///
    /// All filters can include multiple values, where data will be filtered with an OR clause: `filter[title]=Title1,Title2` will filter all vulnerabilities where title is equal to `Title1` OR `Title2`.
    ///
    /// String filters are case sensitive.
    ///
    /// Boolean filters accept `true` or `false` as values.
    ///
    /// Number filters must include an operator as a second filter input: `filter[$prop_name][$operator]`. For example, for the vulnerabilities endpoint: `filter[cvss.base.score][lte]=8`.
    ///
    /// Available operators are: `eq` (==), `lt` (<), `lte` (<=), `gt` (>) and `gte` (>=).
    ///
    /// ### Metadata
    ///
    /// Following [JSON:API format](<https://jsonapi.org/format/#document-meta>), object including non-standard meta-information.
    ///
    /// This endpoint includes the meta member in the response. For more details on each of the properties included in this section, check the endpoints response tables.
    ///
    /// ```JSON
    /// {
    ///   "data": [...],
    ///   "meta": {
    ///     "total": 1500,
    ///     "count": 18732,
    ///     "token": "some_token"
    ///   },
    ///   "links": {...}
    /// }
    /// ```
    ///
    pub async fn list_vulnerabilities(
        &self,
        params: ListVulnerabilitiesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListVulnerabilitiesResponse,
        datadog::Error<ListVulnerabilitiesError>,
    > {
        match self.list_vulnerabilities_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a list of vulnerabilities.
    ///
    /// ### Pagination
    ///
    /// Pagination is enabled by default in both `vulnerabilities` and `assets`. The size of the page varies depending on the endpoint and cannot be modified. To automate the request of the next page, you can use the links section in the response.
    ///
    /// This endpoint will return paginated responses. The pages are stored in the links section of the response:
    ///
    /// ```JSON
    /// {
    ///   "data": [...],
    ///   "meta": {...},
    ///   "links": {
    ///     "self": "<https://.../api/v2/security/vulnerabilities",>
    ///     "first": "<https://.../api/v2/security/vulnerabilities?page[number]=1&page[token]=abc",>
    ///     "last": "<https://.../api/v2/security/vulnerabilities?page[number]=43&page[token]=abc",>
    ///     "next": "<https://.../api/v2/security/vulnerabilities?page[number]=2&page[token]=abc">
    ///   }
    /// }
    /// ```
    ///
    ///
    /// - `links.previous` is empty if the first page is requested.
    /// - `links.next` is empty if the last page is requested.
    ///
    /// #### Token
    ///
    /// Vulnerabilities can be created, updated or deleted at any point in time.
    ///
    /// Upon the first request, a token is created to ensure consistency across subsequent paginated requests.
    ///
    /// A token is valid only for 24 hours.
    ///
    /// #### First request
    ///
    /// We consider a request to be the first request when there is no `page[token]` parameter.
    ///
    /// The response of this first request contains the newly created token in the `links` section.
    ///
    /// This token can then be used in the subsequent paginated requests.
    ///
    /// #### Subsequent requests
    ///
    /// Any request containing valid `page[token]` and `page[number]` parameters will be considered a subsequent request.
    ///
    /// If the `token` is invalid, a `404` response will be returned.
    ///
    /// If the page `number` is invalid, a `400` response will be returned.
    ///
    /// ### Filtering
    ///
    /// The request can include some filter parameters to filter the data to be retrieved. The format of the filter parameters follows the [JSON:API format](<https://jsonapi.org/format/#fetching-filtering>): `filter[$prop_name]`, where `prop_name` is the property name in the entity being filtered by.
    ///
    /// All filters can include multiple values, where data will be filtered with an OR clause: `filter[title]=Title1,Title2` will filter all vulnerabilities where title is equal to `Title1` OR `Title2`.
    ///
    /// String filters are case sensitive.
    ///
    /// Boolean filters accept `true` or `false` as values.
    ///
    /// Number filters must include an operator as a second filter input: `filter[$prop_name][$operator]`. For example, for the vulnerabilities endpoint: `filter[cvss.base.score][lte]=8`.
    ///
    /// Available operators are: `eq` (==), `lt` (<), `lte` (<=), `gt` (>) and `gte` (>=).
    ///
    /// ### Metadata
    ///
    /// Following [JSON:API format](<https://jsonapi.org/format/#document-meta>), object including non-standard meta-information.
    ///
    /// This endpoint includes the meta member in the response. For more details on each of the properties included in this section, check the endpoints response tables.
    ///
    /// ```JSON
    /// {
    ///   "data": [...],
    ///   "meta": {
    ///     "total": 1500,
    ///     "count": 18732,
    ///     "token": "some_token"
    ///   },
    ///   "links": {...}
    /// }
    /// ```
    ///
    pub async fn list_vulnerabilities_with_http_info(
        &self,
        params: ListVulnerabilitiesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListVulnerabilitiesResponse>,
        datadog::Error<ListVulnerabilitiesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_vulnerabilities";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_vulnerabilities' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_token = params.page_token;
        let page_number = params.page_number;
        let filter_type = params.filter_type;
        let filter_cvss_base_score_op = params.filter_cvss_base_score_op;
        let filter_cvss_base_severity = params.filter_cvss_base_severity;
        let filter_cvss_base_vector = params.filter_cvss_base_vector;
        let filter_cvss_datadog_score_op = params.filter_cvss_datadog_score_op;
        let filter_cvss_datadog_severity = params.filter_cvss_datadog_severity;
        let filter_cvss_datadog_vector = params.filter_cvss_datadog_vector;
        let filter_status = params.filter_status;
        let filter_tool = params.filter_tool;
        let filter_library_name = params.filter_library_name;
        let filter_library_version = params.filter_library_version;
        let filter_advisory_id = params.filter_advisory_id;
        let filter_risks_exploitation_probability = params.filter_risks_exploitation_probability;
        let filter_risks_poc_exploit_available = params.filter_risks_poc_exploit_available;
        let filter_risks_exploit_available = params.filter_risks_exploit_available;
        let filter_risks_epss_score_op = params.filter_risks_epss_score_op;
        let filter_risks_epss_severity = params.filter_risks_epss_severity;
        let filter_language = params.filter_language;
        let filter_ecosystem = params.filter_ecosystem;
        let filter_code_location_location = params.filter_code_location_location;
        let filter_code_location_file_path = params.filter_code_location_file_path;
        let filter_code_location_method = params.filter_code_location_method;
        let filter_fix_available = params.filter_fix_available;
        let filter_repo_digests = params.filter_repo_digests;
        let filter_asset_name = params.filter_asset_name;
        let filter_asset_type = params.filter_asset_type;
        let filter_asset_version_first = params.filter_asset_version_first;
        let filter_asset_version_last = params.filter_asset_version_last;
        let filter_asset_repository_url = params.filter_asset_repository_url;
        let filter_asset_risks_in_production = params.filter_asset_risks_in_production;
        let filter_asset_risks_under_attack = params.filter_asset_risks_under_attack;
        let filter_asset_risks_is_publicly_accessible =
            params.filter_asset_risks_is_publicly_accessible;
        let filter_asset_risks_has_privileged_access =
            params.filter_asset_risks_has_privileged_access;
        let filter_asset_risks_has_access_to_sensitive_data =
            params.filter_asset_risks_has_access_to_sensitive_data;
        let filter_asset_environments = params.filter_asset_environments;
        let filter_asset_arch = params.filter_asset_arch;
        let filter_asset_operating_system_name = params.filter_asset_operating_system_name;
        let filter_asset_operating_system_version = params.filter_asset_operating_system_version;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_token {
            local_req_builder =
                local_req_builder.query(&[("page[token]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_type {
            local_req_builder =
                local_req_builder.query(&[("filter[type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_cvss_base_score_op {
            local_req_builder = local_req_builder.query(&[(
                "filter[cvss.base.score][`$op`]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_cvss_base_severity {
            local_req_builder = local_req_builder
                .query(&[("filter[cvss.base.severity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_cvss_base_vector {
            local_req_builder = local_req_builder
                .query(&[("filter[cvss.base.vector]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_cvss_datadog_score_op {
            local_req_builder = local_req_builder.query(&[(
                "filter[cvss.datadog.score][`$op`]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_cvss_datadog_severity {
            local_req_builder = local_req_builder.query(&[(
                "filter[cvss.datadog.severity]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_cvss_datadog_vector {
            local_req_builder = local_req_builder.query(&[(
                "filter[cvss.datadog.vector]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tool {
            local_req_builder =
                local_req_builder.query(&[("filter[tool]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_library_name {
            local_req_builder = local_req_builder
                .query(&[("filter[library.name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_library_version {
            local_req_builder = local_req_builder
                .query(&[("filter[library.version]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_advisory_id {
            local_req_builder =
                local_req_builder.query(&[("filter[advisory_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_risks_exploitation_probability {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.exploitation_probability]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_poc_exploit_available {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.poc_exploit_available]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_exploit_available {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.exploit_available]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_epss_score_op {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.epss.score][`$op`]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_epss_severity {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.epss.severity]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_language {
            local_req_builder =
                local_req_builder.query(&[("filter[language]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_ecosystem {
            local_req_builder =
                local_req_builder.query(&[("filter[ecosystem]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_code_location_location {
            local_req_builder = local_req_builder.query(&[(
                "filter[code_location.location]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_code_location_file_path {
            local_req_builder = local_req_builder.query(&[(
                "filter[code_location.file_path]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_code_location_method {
            local_req_builder = local_req_builder.query(&[(
                "filter[code_location.method]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_fix_available {
            local_req_builder = local_req_builder
                .query(&[("filter[fix_available]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_repo_digests {
            local_req_builder = local_req_builder
                .query(&[("filter[repo_digests]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_name {
            local_req_builder =
                local_req_builder.query(&[("filter[asset.name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_type {
            local_req_builder =
                local_req_builder.query(&[("filter[asset.type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_version_first {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.version.first]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_version_last {
            local_req_builder = local_req_builder
                .query(&[("filter[asset.version.last]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_repository_url {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.repository_url]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_risks_in_production {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.risks.in_production]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_risks_under_attack {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.risks.under_attack]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_risks_is_publicly_accessible {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.risks.is_publicly_accessible]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_risks_has_privileged_access {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.risks.has_privileged_access]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_risks_has_access_to_sensitive_data {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.risks.has_access_to_sensitive_data]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_environments {
            local_req_builder = local_req_builder
                .query(&[("filter[asset.environments]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_arch {
            local_req_builder =
                local_req_builder.query(&[("filter[asset.arch]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_asset_operating_system_name {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.operating_system.name]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_asset_operating_system_version {
            local_req_builder = local_req_builder.query(&[(
                "filter[asset.operating_system.version]",
                &local_query_param.to_string(),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListVulnerabilitiesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListVulnerabilitiesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of vulnerable assets.
    ///
    /// ### Pagination
    ///
    /// Please review the [Pagination section for the "List Vulnerabilities"](#pagination) endpoint.
    ///
    /// ### Filtering
    ///
    /// Please review the [Filtering section for the "List Vulnerabilities"](#filtering) endpoint.
    ///
    /// ### Metadata
    ///
    /// Please review the [Metadata section for the "List Vulnerabilities"](#metadata) endpoint.
    ///
    pub async fn list_vulnerable_assets(
        &self,
        params: ListVulnerableAssetsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListVulnerableAssetsResponse,
        datadog::Error<ListVulnerableAssetsError>,
    > {
        match self.list_vulnerable_assets_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a list of vulnerable assets.
    ///
    /// ### Pagination
    ///
    /// Please review the [Pagination section for the "List Vulnerabilities"](#pagination) endpoint.
    ///
    /// ### Filtering
    ///
    /// Please review the [Filtering section for the "List Vulnerabilities"](#filtering) endpoint.
    ///
    /// ### Metadata
    ///
    /// Please review the [Metadata section for the "List Vulnerabilities"](#metadata) endpoint.
    ///
    pub async fn list_vulnerable_assets_with_http_info(
        &self,
        params: ListVulnerableAssetsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListVulnerableAssetsResponse>,
        datadog::Error<ListVulnerableAssetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_vulnerable_assets";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_vulnerable_assets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_token = params.page_token;
        let page_number = params.page_number;
        let filter_name = params.filter_name;
        let filter_type = params.filter_type;
        let filter_version_first = params.filter_version_first;
        let filter_version_last = params.filter_version_last;
        let filter_repository_url = params.filter_repository_url;
        let filter_risks_in_production = params.filter_risks_in_production;
        let filter_risks_under_attack = params.filter_risks_under_attack;
        let filter_risks_is_publicly_accessible = params.filter_risks_is_publicly_accessible;
        let filter_risks_has_privileged_access = params.filter_risks_has_privileged_access;
        let filter_risks_has_access_to_sensitive_data =
            params.filter_risks_has_access_to_sensitive_data;
        let filter_environments = params.filter_environments;
        let filter_arch = params.filter_arch;
        let filter_operating_system_name = params.filter_operating_system_name;
        let filter_operating_system_version = params.filter_operating_system_version;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/assets",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_token {
            local_req_builder =
                local_req_builder.query(&[("page[token]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_type {
            local_req_builder =
                local_req_builder.query(&[("filter[type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_version_first {
            local_req_builder = local_req_builder
                .query(&[("filter[version.first]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_version_last {
            local_req_builder = local_req_builder
                .query(&[("filter[version.last]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_repository_url {
            local_req_builder = local_req_builder
                .query(&[("filter[repository_url]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_risks_in_production {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.in_production]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_under_attack {
            local_req_builder = local_req_builder
                .query(&[("filter[risks.under_attack]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_risks_is_publicly_accessible {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.is_publicly_accessible]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_has_privileged_access {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.has_privileged_access]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_risks_has_access_to_sensitive_data {
            local_req_builder = local_req_builder.query(&[(
                "filter[risks.has_access_to_sensitive_data]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_environments {
            local_req_builder = local_req_builder
                .query(&[("filter[environments]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_arch {
            local_req_builder =
                local_req_builder.query(&[("filter[arch]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_operating_system_name {
            local_req_builder = local_req_builder.query(&[(
                "filter[operating_system.name]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_operating_system_version {
            local_req_builder = local_req_builder.query(&[(
                "filter[operating_system.version]",
                &local_query_param.to_string(),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListVulnerableAssetsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListVulnerableAssetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Mute or unmute findings.
    pub async fn mute_findings(
        &self,
        body: crate::datadogV2::model::BulkMuteFindingsRequest,
    ) -> Result<crate::datadogV2::model::BulkMuteFindingsResponse, datadog::Error<MuteFindingsError>>
    {
        match self.mute_findings_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Mute or unmute findings.
    pub async fn mute_findings_with_http_info(
        &self,
        body: crate::datadogV2::model::BulkMuteFindingsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BulkMuteFindingsResponse>,
        datadog::Error<MuteFindingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.mute_findings";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.mute_findings' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::BulkMuteFindingsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<MuteFindingsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update the inbox rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_inbox_rule(
        &self,
        inbox_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchInboxRulesParameters,
    ) -> Result<crate::datadogV2::model::InboxRuleResponse, datadog::Error<PatchInboxRuleError>>
    {
        match self
            .patch_inbox_rule_with_http_info(inbox_rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update the inbox rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_inbox_rule_with_http_info(
        &self,
        inbox_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchInboxRulesParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::InboxRuleResponse>,
        datadog::Error<PatchInboxRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.patch_inbox_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules/{inbox_rule_id}",
            local_configuration.get_operation_host(operation_id),
            inbox_rule_id = datadog::urlencode(inbox_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::InboxRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PatchInboxRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update the mute rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_mute_rule(
        &self,
        mute_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchMuteRuleParameters,
    ) -> Result<crate::datadogV2::model::MuteRuleResponse, datadog::Error<PatchMuteRuleError>> {
        match self
            .patch_mute_rule_with_http_info(mute_rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update the mute rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_mute_rule_with_http_info(
        &self,
        mute_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::PatchMuteRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MuteRuleResponse>,
        datadog::Error<PatchMuteRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.patch_mute_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules/{mute_rule_id}",
            local_configuration.get_operation_host(operation_id),
            mute_rule_id = datadog::urlencode(mute_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MuteRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PatchMuteRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update the notification rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_signal_notification_rule(
        &self,
        id: String,
        body: crate::datadogV2::model::PatchNotificationRuleParameters,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<PatchSignalNotificationRuleError>,
    > {
        match self
            .patch_signal_notification_rule_with_http_info(id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update the notification rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_signal_notification_rule_with_http_info(
        &self,
        id: String,
        body: crate::datadogV2::model::PatchNotificationRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<PatchSignalNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.patch_signal_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/signals/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PatchSignalNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update the notification rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_vulnerability_notification_rule(
        &self,
        id: String,
        body: crate::datadogV2::model::PatchNotificationRuleParameters,
    ) -> Result<
        crate::datadogV2::model::NotificationRuleResponse,
        datadog::Error<PatchVulnerabilityNotificationRuleError>,
    > {
        match self
            .patch_vulnerability_notification_rule_with_http_info(id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update the notification rule. All fields are optional; if a field is not provided, it is not updated.
    pub async fn patch_vulnerability_notification_rule_with_http_info(
        &self,
        id: String,
        body: crate::datadogV2::model::PatchNotificationRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::NotificationRuleResponse>,
        datadog::Error<PatchVulnerabilityNotificationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.patch_vulnerability_notification_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/notification_rules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::NotificationRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PatchVulnerabilityNotificationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Reorder the list of inbox rules in the pipeline and return the reordered list of rules.
    /// To reorder fields, you must provide the full list of pipeline rules in the new order.
    pub async fn reorder_inbox_rules(
        &self,
        body: crate::datadogV2::model::ReorderInboxRulesParameters,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<ReorderInboxRulesError>,
    > {
        match self.reorder_inbox_rules_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Reorder the list of inbox rules in the pipeline and return the reordered list of rules.
    /// To reorder fields, you must provide the full list of pipeline rules in the new order.
    pub async fn reorder_inbox_rules_with_http_info(
        &self,
        body: crate::datadogV2::model::ReorderInboxRulesParameters,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<ReorderInboxRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_inbox_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules/reorder",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ReorderInboxRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Reorder the list of mute rules in the pipeline and return the reordered list of rules.
    /// To reorder fields, you must provide the full list of pipeline rules in the new order.
    pub async fn reorder_mute_rules(
        &self,
        body: crate::datadogV2::model::ReorderMuteRulesParameters,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<ReorderMuteRulesError>,
    > {
        match self.reorder_mute_rules_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Reorder the list of mute rules in the pipeline and return the reordered list of rules.
    /// To reorder fields, you must provide the full list of pipeline rules in the new order.
    pub async fn reorder_mute_rules_with_http_info(
        &self,
        body: crate::datadogV2::model::ReorderMuteRulesParameters,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<ReorderMuteRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_mute_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules/reorder",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ReorderMuteRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Run a historical job.
    pub async fn run_historical_job(
        &self,
        body: crate::datadogV2::model::RunHistoricalJobRequest,
    ) -> Result<crate::datadogV2::model::JobCreateResponse, datadog::Error<RunHistoricalJobError>>
    {
        match self.run_historical_job_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Run a historical job.
    pub async fn run_historical_job_with_http_info(
        &self,
        body: crate::datadogV2::model::RunHistoricalJobRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::JobCreateResponse>,
        datadog::Error<RunHistoricalJobError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.run_historical_job";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.run_historical_job' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/siem-historical-detections/jobs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::JobCreateResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<RunHistoricalJobError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns security signals that match a search query.
    /// Both this endpoint and the GET endpoint can be used interchangeably for listing
    /// security signals.
    pub async fn search_security_monitoring_signals(
        &self,
        params: SearchSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
        datadog::Error<SearchSecurityMonitoringSignalsError>,
    > {
        match self
            .search_security_monitoring_signals_with_http_info(params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn search_security_monitoring_signals_with_pagination(
        &self,
        mut params: SearchSecurityMonitoringSignalsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::SecurityMonitoringSignal,
            datadog::Error<SearchSecurityMonitoringSignalsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::SecurityMonitoringSignalListRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::SecurityMonitoringSignalListRequestPage::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_security_monitoring_signals(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.body.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(after);
            }
        }
    }

    /// Returns security signals that match a search query.
    /// Both this endpoint and the GET endpoint can be used interchangeably for listing
    /// security signals.
    pub async fn search_security_monitoring_signals_with_http_info(
        &self,
        params: SearchSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        datadog::Error<SearchSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_security_monitoring_signals";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SearchSecurityMonitoringSignalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Test an existing rule.
    pub async fn test_existing_security_monitoring_rule(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleTestRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleTestResponse,
        datadog::Error<TestExistingSecurityMonitoringRuleError>,
    > {
        match self
            .test_existing_security_monitoring_rule_with_http_info(rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Test an existing rule.
    pub async fn test_existing_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleTestRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleTestResponse>,
        datadog::Error<TestExistingSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.test_existing_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}/test",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringRuleTestResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<TestExistingSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Test a rule.
    pub async fn test_security_monitoring_rule(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleTestRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleTestResponse,
        datadog::Error<TestSecurityMonitoringRuleError>,
    > {
        match self
            .test_security_monitoring_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Test a rule.
    pub async fn test_security_monitoring_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleTestRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleTestResponse>,
        datadog::Error<TestSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.test_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/test",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringRuleTestResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<TestSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the whole inbox rule. If an optional field is not provided, it is set to its default value.
    pub async fn update_inbox_rule(
        &self,
        inbox_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateInboxRuleParameters,
    ) -> Result<crate::datadogV2::model::InboxRuleResponse, datadog::Error<UpdateInboxRuleError>>
    {
        match self
            .update_inbox_rule_with_http_info(inbox_rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update the whole inbox rule. If an optional field is not provided, it is set to its default value.
    pub async fn update_inbox_rule_with_http_info(
        &self,
        inbox_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateInboxRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::InboxRuleResponse>,
        datadog::Error<UpdateInboxRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_inbox_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/inbox_rules/{inbox_rule_id}",
            local_configuration.get_operation_host(operation_id),
            inbox_rule_id = datadog::urlencode(inbox_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::InboxRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateInboxRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the whole mute rule. If an optional field is not provided, it is set to its default value.
    pub async fn update_mute_rule(
        &self,
        mute_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateMuteRuleParameters,
    ) -> Result<crate::datadogV2::model::MuteRuleResponse, datadog::Error<UpdateMuteRuleError>>
    {
        match self
            .update_mute_rule_with_http_info(mute_rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update the whole mute rule. If an optional field is not provided, it is set to its default value.
    pub async fn update_mute_rule_with_http_info(
        &self,
        mute_rule_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateMuteRuleParameters,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MuteRuleResponse>,
        datadog::Error<UpdateMuteRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_mute_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/vulnerabilities/pipelines/mute_rules/{mute_rule_id}",
            local_configuration.get_operation_host(operation_id),
            mute_rule_id = datadog::urlencode(mute_rule_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MuteRuleResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateMuteRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a specific security filter.
    /// Returns the security filter object when the request is successful.
    pub async fn update_security_filter(
        &self,
        security_filter_id: String,
        body: crate::datadogV2::model::SecurityFilterUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityFilterResponse,
        datadog::Error<UpdateSecurityFilterError>,
    > {
        match self
            .update_security_filter_with_http_info(security_filter_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a specific security filter.
    /// Returns the security filter object when the request is successful.
    pub async fn update_security_filter_with_http_info(
        &self,
        security_filter_id: String,
        body: crate::datadogV2::model::SecurityFilterUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        datadog::Error<UpdateSecurityFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_security_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.get_operation_host(operation_id),
            security_filter_id = datadog::urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityFilterResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an existing rule. When updating `cases`, `queries` or `options`, the whole field
    /// must be included. For example, when modifying a query all queries must be included.
    /// Default rules can only be updated to be enabled, to change notifications, or to update
    /// the tags (default tags cannot be removed).
    pub async fn update_security_monitoring_rule(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleUpdatePayload,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringRuleResponse,
        datadog::Error<UpdateSecurityMonitoringRuleError>,
    > {
        match self
            .update_security_monitoring_rule_with_http_info(rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update an existing rule. When updating `cases`, `queries` or `options`, the whole field
    /// must be included. For example, when modifying a query all queries must be included.
    /// Default rules can only be updated to be enabled, to change notifications, or to update
    /// the tags (default tags cannot be removed).
    pub async fn update_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleUpdatePayload,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        datadog::Error<UpdateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::SecurityMonitoringRuleResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a specific suppression rule.
    pub async fn update_security_monitoring_suppression(
        &self,
        suppression_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSuppressionUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
        datadog::Error<UpdateSecurityMonitoringSuppressionError>,
    > {
        match self
            .update_security_monitoring_suppression_with_http_info(suppression_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a specific suppression rule.
    pub async fn update_security_monitoring_suppression_with_http_info(
        &self,
        suppression_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSuppressionUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityMonitoringSuppressionResponse>,
        datadog::Error<UpdateSecurityMonitoringSuppressionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_security_monitoring_suppression";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/suppressions/{suppression_id}",
            local_configuration.get_operation_host(operation_id),
            suppression_id = datadog::urlencode(suppression_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SecurityMonitoringSuppressionResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateSecurityMonitoringSuppressionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Validate a detection rule.
    pub async fn validate_security_monitoring_rule(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleValidatePayload,
    ) -> Result<(), datadog::Error<ValidateSecurityMonitoringRuleError>> {
        match self
            .validate_security_monitoring_rule_with_http_info(body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Validate a detection rule.
    pub async fn validate_security_monitoring_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleValidatePayload,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ValidateSecurityMonitoringRuleError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.validate_security_monitoring_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/validation",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<ValidateSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
