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

/// DeleteSourcemapsOptionalParams is a struct for passing parameters to the method [`RUMAPI::delete_sourcemaps`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteSourcemapsOptionalParams {
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_service: Option<Vec<String>>,
    /// Filter by version values (multiple values allowed, maximum 10).
    /// Required for `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_version: Option<Vec<String>>,
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub filter_variant: Option<Vec<String>>,
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub filter_id: Option<Vec<String>>,
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub filter_build_id: Option<Vec<String>>,
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub filter_uuid: Option<Vec<String>>,
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub filter_platform: Option<Vec<String>>,
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub filter_build_number: Option<Vec<String>>,
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub filter_bundle_name: Option<Vec<String>>,
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub filter_arch: Option<Vec<String>>,
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub filter_symbol_source: Option<Vec<String>>,
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub filter_origin: Option<Vec<String>>,
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub filter_origin_version: Option<Vec<String>>,
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub filter_filename: Option<String>,
    /// Filter by debug ID (single value). Supported for `react`.
    pub filter_debug_id: Option<String>,
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub filter_gnu_build_id: Option<String>,
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub filter_go_build_id: Option<String>,
    /// Filter by file hash (single value). Supported for `elf`.
    pub filter_file_hash: Option<String>,
}

impl DeleteSourcemapsOptionalParams {
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_service(mut self, value: Vec<String>) -> Self {
        self.filter_service = Some(value);
        self
    }
    /// Filter by version values (multiple values allowed, maximum 10).
    /// Required for `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_version(mut self, value: Vec<String>) -> Self {
        self.filter_version = Some(value);
        self
    }
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub fn filter_variant(mut self, value: Vec<String>) -> Self {
        self.filter_variant = Some(value);
        self
    }
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub fn filter_id(mut self, value: Vec<String>) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub fn filter_build_id(mut self, value: Vec<String>) -> Self {
        self.filter_build_id = Some(value);
        self
    }
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub fn filter_uuid(mut self, value: Vec<String>) -> Self {
        self.filter_uuid = Some(value);
        self
    }
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub fn filter_platform(mut self, value: Vec<String>) -> Self {
        self.filter_platform = Some(value);
        self
    }
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub fn filter_build_number(mut self, value: Vec<String>) -> Self {
        self.filter_build_number = Some(value);
        self
    }
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub fn filter_bundle_name(mut self, value: Vec<String>) -> Self {
        self.filter_bundle_name = Some(value);
        self
    }
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub fn filter_arch(mut self, value: Vec<String>) -> Self {
        self.filter_arch = Some(value);
        self
    }
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub fn filter_symbol_source(mut self, value: Vec<String>) -> Self {
        self.filter_symbol_source = Some(value);
        self
    }
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin(mut self, value: Vec<String>) -> Self {
        self.filter_origin = Some(value);
        self
    }
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin_version(mut self, value: Vec<String>) -> Self {
        self.filter_origin_version = Some(value);
        self
    }
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub fn filter_filename(mut self, value: String) -> Self {
        self.filter_filename = Some(value);
        self
    }
    /// Filter by debug ID (single value). Supported for `react`.
    pub fn filter_debug_id(mut self, value: String) -> Self {
        self.filter_debug_id = Some(value);
        self
    }
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub fn filter_gnu_build_id(mut self, value: String) -> Self {
        self.filter_gnu_build_id = Some(value);
        self
    }
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub fn filter_go_build_id(mut self, value: String) -> Self {
        self.filter_go_build_id = Some(value);
        self
    }
    /// Filter by file hash (single value). Supported for `elf`.
    pub fn filter_file_hash(mut self, value: String) -> Self {
        self.filter_file_hash = Some(value);
        self
    }
}

/// ListRUMEventsOptionalParams is a struct for passing parameters to the method [`RUMAPI::list_rum_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRUMEventsOptionalParams {
    /// Search query following RUM syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<chrono::DateTime<chrono::Utc>>,
    /// Order of events in results.
    pub sort: Option<crate::datadogV2::model::RUMSort>,
    /// List following results with a cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page_limit: Option<i32>,
}

impl ListRUMEventsOptionalParams {
    /// Search query following RUM syntax.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Minimum timestamp for requested events.
    pub fn filter_from(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Maximum timestamp for requested events.
    pub fn filter_to(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Order of events in results.
    pub fn sort(mut self, value: crate::datadogV2::model::RUMSort) -> Self {
        self.sort = Some(value);
        self
    }
    /// List following results with a cursor provided in the previous query.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of events in the response.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListSourcemapsOptionalParams is a struct for passing parameters to the method [`RUMAPI::list_sourcemaps`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSourcemapsOptionalParams {
    /// The type of source map. Defaults to `js`.
    pub mapkind: Option<crate::datadogV2::model::SourcemapMapKind>,
    /// The number of results to return per page. Must be at least 1.
    pub page_size: Option<i64>,
    /// The page number to retrieve, starting from 1.
    pub page_number: Option<i64>,
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_service: Option<Vec<String>>,
    /// Filter by version values (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_version: Option<Vec<String>>,
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub filter_variant: Option<Vec<String>>,
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub filter_id: Option<Vec<String>>,
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub filter_build_id: Option<Vec<String>>,
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub filter_uuid: Option<Vec<String>>,
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub filter_platform: Option<Vec<String>>,
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub filter_build_number: Option<Vec<String>>,
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub filter_bundle_name: Option<Vec<String>>,
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub filter_arch: Option<Vec<String>>,
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub filter_symbol_source: Option<Vec<String>>,
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub filter_origin: Option<Vec<String>>,
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub filter_origin_version: Option<Vec<String>>,
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub filter_filename: Option<String>,
    /// Filter by debug ID (single value). Supported for `react`.
    pub filter_debug_id: Option<String>,
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub filter_gnu_build_id: Option<String>,
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub filter_go_build_id: Option<String>,
    /// Filter by file hash (single value). Supported for `elf`.
    pub filter_file_hash: Option<String>,
}

impl ListSourcemapsOptionalParams {
    /// The type of source map. Defaults to `js`.
    pub fn mapkind(mut self, value: crate::datadogV2::model::SourcemapMapKind) -> Self {
        self.mapkind = Some(value);
        self
    }
    /// The number of results to return per page. Must be at least 1.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The page number to retrieve, starting from 1.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_service(mut self, value: Vec<String>) -> Self {
        self.filter_service = Some(value);
        self
    }
    /// Filter by version values (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_version(mut self, value: Vec<String>) -> Self {
        self.filter_version = Some(value);
        self
    }
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub fn filter_variant(mut self, value: Vec<String>) -> Self {
        self.filter_variant = Some(value);
        self
    }
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub fn filter_id(mut self, value: Vec<String>) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub fn filter_build_id(mut self, value: Vec<String>) -> Self {
        self.filter_build_id = Some(value);
        self
    }
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub fn filter_uuid(mut self, value: Vec<String>) -> Self {
        self.filter_uuid = Some(value);
        self
    }
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub fn filter_platform(mut self, value: Vec<String>) -> Self {
        self.filter_platform = Some(value);
        self
    }
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub fn filter_build_number(mut self, value: Vec<String>) -> Self {
        self.filter_build_number = Some(value);
        self
    }
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub fn filter_bundle_name(mut self, value: Vec<String>) -> Self {
        self.filter_bundle_name = Some(value);
        self
    }
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub fn filter_arch(mut self, value: Vec<String>) -> Self {
        self.filter_arch = Some(value);
        self
    }
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub fn filter_symbol_source(mut self, value: Vec<String>) -> Self {
        self.filter_symbol_source = Some(value);
        self
    }
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin(mut self, value: Vec<String>) -> Self {
        self.filter_origin = Some(value);
        self
    }
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin_version(mut self, value: Vec<String>) -> Self {
        self.filter_origin_version = Some(value);
        self
    }
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub fn filter_filename(mut self, value: String) -> Self {
        self.filter_filename = Some(value);
        self
    }
    /// Filter by debug ID (single value). Supported for `react`.
    pub fn filter_debug_id(mut self, value: String) -> Self {
        self.filter_debug_id = Some(value);
        self
    }
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub fn filter_gnu_build_id(mut self, value: String) -> Self {
        self.filter_gnu_build_id = Some(value);
        self
    }
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub fn filter_go_build_id(mut self, value: String) -> Self {
        self.filter_go_build_id = Some(value);
        self
    }
    /// Filter by file hash (single value). Supported for `elf`.
    pub fn filter_file_hash(mut self, value: String) -> Self {
        self.filter_file_hash = Some(value);
        self
    }
}

/// RestoreSourcemapsOptionalParams is a struct for passing parameters to the method [`RUMAPI::restore_sourcemaps`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct RestoreSourcemapsOptionalParams {
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_service: Option<Vec<String>>,
    /// Filter by version values (multiple values allowed, maximum 10).
    /// Required for `js`, `jvm`, `react`, and `flutter` map kinds.
    pub filter_version: Option<Vec<String>>,
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub filter_variant: Option<Vec<String>>,
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub filter_id: Option<Vec<String>>,
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub filter_build_id: Option<Vec<String>>,
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub filter_uuid: Option<Vec<String>>,
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub filter_platform: Option<Vec<String>>,
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub filter_build_number: Option<Vec<String>>,
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub filter_bundle_name: Option<Vec<String>>,
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub filter_arch: Option<Vec<String>>,
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub filter_symbol_source: Option<Vec<String>>,
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub filter_origin: Option<Vec<String>>,
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub filter_origin_version: Option<Vec<String>>,
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub filter_filename: Option<String>,
    /// Filter by debug ID (single value). Supported for `react`.
    pub filter_debug_id: Option<String>,
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub filter_gnu_build_id: Option<String>,
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub filter_go_build_id: Option<String>,
    /// Filter by file hash (single value). Supported for `elf`.
    pub filter_file_hash: Option<String>,
}

impl RestoreSourcemapsOptionalParams {
    /// Filter by service names (multiple values allowed). Required for
    /// `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_service(mut self, value: Vec<String>) -> Self {
        self.filter_service = Some(value);
        self
    }
    /// Filter by version values (multiple values allowed, maximum 10).
    /// Required for `js`, `jvm`, `react`, and `flutter` map kinds.
    pub fn filter_version(mut self, value: Vec<String>) -> Self {
        self.filter_version = Some(value);
        self
    }
    /// Filter by variant values (multiple values allowed). Supported for `jvm`.
    pub fn filter_variant(mut self, value: Vec<String>) -> Self {
        self.filter_variant = Some(value);
        self
    }
    /// Filter by source map ID values (multiple values allowed). Supported for all map kinds.
    pub fn filter_id(mut self, value: Vec<String>) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter by build ID values (multiple values allowed). Supported for `jvm`, `ndk`, and `il2cpp`.
    pub fn filter_build_id(mut self, value: Vec<String>) -> Self {
        self.filter_build_id = Some(value);
        self
    }
    /// Filter by UUID values (multiple values allowed). Supported for `ios`.
    pub fn filter_uuid(mut self, value: Vec<String>) -> Self {
        self.filter_uuid = Some(value);
        self
    }
    /// Filter by platform values (multiple values allowed). Supported for `react`.
    pub fn filter_platform(mut self, value: Vec<String>) -> Self {
        self.filter_platform = Some(value);
        self
    }
    /// Filter by build number values (multiple values allowed). Supported for `react`.
    pub fn filter_build_number(mut self, value: Vec<String>) -> Self {
        self.filter_build_number = Some(value);
        self
    }
    /// Filter by bundle name values (multiple values allowed). Supported for `react`.
    pub fn filter_bundle_name(mut self, value: Vec<String>) -> Self {
        self.filter_bundle_name = Some(value);
        self
    }
    /// Filter by architecture values (multiple values allowed). Supported
    /// for `flutter`, `elf`, and `ndk`.
    pub fn filter_arch(mut self, value: Vec<String>) -> Self {
        self.filter_arch = Some(value);
        self
    }
    /// Filter by symbol source values (multiple values allowed). Supported for `elf`.
    pub fn filter_symbol_source(mut self, value: Vec<String>) -> Self {
        self.filter_symbol_source = Some(value);
        self
    }
    /// Filter by origin values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin(mut self, value: Vec<String>) -> Self {
        self.filter_origin = Some(value);
        self
    }
    /// Filter by origin version values (multiple values allowed). Supported for `elf`.
    pub fn filter_origin_version(mut self, value: Vec<String>) -> Self {
        self.filter_origin_version = Some(value);
        self
    }
    /// Filter by filename (single value). Supported for `js`, `elf`, and `ndk`.
    pub fn filter_filename(mut self, value: String) -> Self {
        self.filter_filename = Some(value);
        self
    }
    /// Filter by debug ID (single value). Supported for `react`.
    pub fn filter_debug_id(mut self, value: String) -> Self {
        self.filter_debug_id = Some(value);
        self
    }
    /// Filter by GNU build ID (single value). Supported for `elf`.
    pub fn filter_gnu_build_id(mut self, value: String) -> Self {
        self.filter_gnu_build_id = Some(value);
        self
    }
    /// Filter by Go build ID (single value). Supported for `elf`.
    pub fn filter_go_build_id(mut self, value: String) -> Self {
        self.filter_go_build_id = Some(value);
        self
    }
    /// Filter by file hash (single value). Supported for `elf`.
    pub fn filter_file_hash(mut self, value: String) -> Self {
        self.filter_file_hash = Some(value);
        self
    }
}

/// AggregateRUMEventsError is a struct for typed errors of method [`RUMAPI::aggregate_rum_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateRUMEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateRUMApplicationError is a struct for typed errors of method [`RUMAPI::create_rum_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRUMApplicationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteRUMApplicationError is a struct for typed errors of method [`RUMAPI::delete_rum_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRUMApplicationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSourcemapsError is a struct for typed errors of method [`RUMAPI::delete_sourcemaps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSourcemapsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationError is a struct for typed errors of method [`RUMAPI::get_rum_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationsError is a struct for typed errors of method [`RUMAPI::get_rum_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetServiceRepositoryInfoError is a struct for typed errors of method [`RUMAPI::get_service_repository_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceRepositoryInfoError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSourcemapsError is a struct for typed errors of method [`RUMAPI::get_sourcemaps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSourcemapsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRUMEventsError is a struct for typed errors of method [`RUMAPI::list_rum_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRUMEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSourcemapsError is a struct for typed errors of method [`RUMAPI::list_sourcemaps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSourcemapsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RestoreSourcemapsError is a struct for typed errors of method [`RUMAPI::restore_sourcemaps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreSourcemapsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchRUMEventsError is a struct for typed errors of method [`RUMAPI::search_rum_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRUMEventsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRUMApplicationError is a struct for typed errors of method [`RUMAPI::update_rum_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRUMApplicationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage your Real User Monitoring (RUM) applications, and search or aggregate your RUM events over HTTP. See the [RUM & Session Replay page](<https://docs.datadoghq.com/real_user_monitoring/>) for more information
#[derive(Debug, Clone)]
pub struct RUMAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RUMAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RUMAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
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

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// The API endpoint to aggregate RUM events into buckets of computed metrics and timeseries.
    pub async fn aggregate_rum_events(
        &self,
        body: crate::datadogV2::model::RUMAggregateRequest,
    ) -> Result<
        crate::datadogV2::model::RUMAnalyticsAggregateResponse,
        datadog::Error<AggregateRUMEventsError>,
    > {
        match self.aggregate_rum_events_with_http_info(body).await {
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

    /// The API endpoint to aggregate RUM events into buckets of computed metrics and timeseries.
    pub async fn aggregate_rum_events_with_http_info(
        &self,
        body: crate::datadogV2::model::RUMAggregateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMAnalyticsAggregateResponse>,
        datadog::Error<AggregateRUMEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.aggregate_rum_events";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/analytics/aggregate",
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::RUMAnalyticsAggregateResponse>(
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
            let local_entity: Option<AggregateRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new RUM application in your organization.
    pub async fn create_rum_application(
        &self,
        body: crate::datadogV2::model::RUMApplicationCreateRequest,
    ) -> Result<
        crate::datadogV2::model::RUMApplicationResponse,
        datadog::Error<CreateRUMApplicationError>,
    > {
        match self.create_rum_application_with_http_info(body).await {
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

    /// Create a new RUM application in your organization.
    pub async fn create_rum_application_with_http_info(
        &self,
        body: crate::datadogV2::model::RUMApplicationCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        datadog::Error<CreateRUMApplicationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_rum_application";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications",
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::RUMApplicationResponse>(
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
            let local_entity: Option<CreateRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an existing RUM application in your organization.
    pub async fn delete_rum_application(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteRUMApplicationError>> {
        match self.delete_rum_application_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing RUM application in your organization.
    pub async fn delete_rum_application_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteRUMApplicationError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_rum_application";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
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
            let local_entity: Option<DeleteRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes source maps matching the specified filter criteria. Supports
    /// dry-run mode to preview which source maps would be deleted without
    /// performing the actual deletion.
    pub async fn delete_sourcemaps(
        &self,
        mapkind: crate::datadogV2::model::SourcemapMapKind,
        dry_run: bool,
        params: DeleteSourcemapsOptionalParams,
    ) -> Result<crate::datadogV2::model::SourcemapsResponse, datadog::Error<DeleteSourcemapsError>>
    {
        match self
            .delete_sourcemaps_with_http_info(mapkind, dry_run, params)
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

    /// Deletes source maps matching the specified filter criteria. Supports
    /// dry-run mode to preview which source maps would be deleted without
    /// performing the actual deletion.
    pub async fn delete_sourcemaps_with_http_info(
        &self,
        mapkind: crate::datadogV2::model::SourcemapMapKind,
        dry_run: bool,
        params: DeleteSourcemapsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SourcemapsResponse>,
        datadog::Error<DeleteSourcemapsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_sourcemaps";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_sourcemaps' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_service = params.filter_service;
        let filter_version = params.filter_version;
        let filter_variant = params.filter_variant;
        let filter_id = params.filter_id;
        let filter_build_id = params.filter_build_id;
        let filter_uuid = params.filter_uuid;
        let filter_platform = params.filter_platform;
        let filter_build_number = params.filter_build_number;
        let filter_bundle_name = params.filter_bundle_name;
        let filter_arch = params.filter_arch;
        let filter_symbol_source = params.filter_symbol_source;
        let filter_origin = params.filter_origin;
        let filter_origin_version = params.filter_origin_version;
        let filter_filename = params.filter_filename;
        let filter_debug_id = params.filter_debug_id;
        let filter_gnu_build_id = params.filter_gnu_build_id;
        let filter_go_build_id = params.filter_go_build_id;
        let filter_file_hash = params.filter_file_hash;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sourcemaps",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("mapkind", &mapkind.to_string())]);
        local_req_builder = local_req_builder.query(&[("dry_run", &dry_run.to_string())]);
        if let Some(ref local) = filter_service {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[service]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[version]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_variant {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[variant]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_id {
            for param in local {
                local_req_builder = local_req_builder.query(&[("filter[id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_id {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_uuid {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[uuid]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_platform {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[platform]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_number {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_number]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_bundle_name {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[bundle_name]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_arch {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[arch]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_symbol_source {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[symbol_source]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin_version]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_filename {
            local_req_builder =
                local_req_builder.query(&[("filter[filename]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_debug_id {
            local_req_builder =
                local_req_builder.query(&[("filter[debug_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_gnu_build_id {
            local_req_builder = local_req_builder
                .query(&[("filter[gnu_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_go_build_id {
            local_req_builder =
                local_req_builder.query(&[("filter[go_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_file_hash {
            local_req_builder =
                local_req_builder.query(&[("filter[file_hash]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::SourcemapsResponse>(
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
            let local_entity: Option<DeleteSourcemapsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the RUM application with given ID in your organization.
    pub async fn get_rum_application(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::RUMApplicationResponse,
        datadog::Error<GetRUMApplicationError>,
    > {
        match self.get_rum_application_with_http_info(id).await {
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

    /// Get the RUM application with given ID in your organization.
    pub async fn get_rum_application_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        datadog::Error<GetRUMApplicationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_rum_application";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
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
            match serde_json::from_str::<crate::datadogV2::model::RUMApplicationResponse>(
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
            let local_entity: Option<GetRUMApplicationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all the RUM applications in your organization.
    pub async fn get_rum_applications(
        &self,
    ) -> Result<
        crate::datadogV2::model::RUMApplicationsResponse,
        datadog::Error<GetRUMApplicationsError>,
    > {
        match self.get_rum_applications_with_http_info().await {
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

    /// List all the RUM applications in your organization.
    pub async fn get_rum_applications_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMApplicationsResponse>,
        datadog::Error<GetRUMApplicationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_rum_applications";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications",
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
            match serde_json::from_str::<crate::datadogV2::model::RUMApplicationsResponse>(
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
            let local_entity: Option<GetRUMApplicationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the repository URL and commit SHA associated with a given service and version.
    pub async fn get_service_repository_info(
        &self,
        body: crate::datadogV2::model::ServiceRepositoryInfoRequest,
    ) -> Result<
        crate::datadogV2::model::ServiceRepositoryInfoResponse,
        datadog::Error<GetServiceRepositoryInfoError>,
    > {
        match self.get_service_repository_info_with_http_info(body).await {
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

    /// Returns the repository URL and commit SHA associated with a given service and version.
    pub async fn get_service_repository_info_with_http_info(
        &self,
        body: crate::datadogV2::model::ServiceRepositoryInfoRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ServiceRepositoryInfoResponse>,
        datadog::Error<GetServiceRepositoryInfoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_service_repository_info";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_service_repository_info' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sourcemaps/service_repository_info",
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::ServiceRepositoryInfoResponse>(
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
            let local_entity: Option<GetServiceRepositoryInfoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves the content of a specific JavaScript source map file by its
    /// filename, service name, and version.
    pub async fn get_sourcemaps(
        &self,
        filename: String,
        service: String,
        version: String,
    ) -> Result<crate::datadogV2::model::SourcemapFileResponse, datadog::Error<GetSourcemapsError>>
    {
        match self
            .get_sourcemaps_with_http_info(filename, service, version)
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

    /// Retrieves the content of a specific JavaScript source map file by its
    /// filename, service name, and version.
    pub async fn get_sourcemaps_with_http_info(
        &self,
        filename: String,
        service: String,
        version: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SourcemapFileResponse>,
        datadog::Error<GetSourcemapsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_sourcemaps";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_sourcemaps' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sourcemaps",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("filename", &filename.to_string())]);
        local_req_builder = local_req_builder.query(&[("service", &service.to_string())]);
        local_req_builder = local_req_builder.query(&[("version", &version.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::SourcemapFileResponse>(
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
            let local_entity: Option<GetSourcemapsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest RUM events.
    ///
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn list_rum_events(
        &self,
        params: ListRUMEventsOptionalParams,
    ) -> Result<crate::datadogV2::model::RUMEventsResponse, datadog::Error<ListRUMEventsError>>
    {
        match self.list_rum_events_with_http_info(params).await {
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

    pub fn list_rum_events_with_pagination(
        &self,
        mut params: ListRUMEventsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::RUMEvent, datadog::Error<ListRUMEventsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_rum_events(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }
                if count == 0 {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.page_cursor = Some(after);
            }
        }
    }

    /// List endpoint returns events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to see your latest RUM events.
    ///
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn list_rum_events_with_http_info(
        &self,
        params: ListRUMEventsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMEventsResponse>,
        datadog::Error<ListRUMEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_rum_events";

        // unbox and build optional parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/events",
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
            match serde_json::from_str::<crate::datadogV2::model::RUMEventsResponse>(&local_content)
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
            let local_entity: Option<ListRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves a paginated list of source maps matching the specified filter criteria.
    pub async fn list_sourcemaps(
        &self,
        params: ListSourcemapsOptionalParams,
    ) -> Result<crate::datadogV2::model::ListSourcemapsResponse, datadog::Error<ListSourcemapsError>>
    {
        match self.list_sourcemaps_with_http_info(params).await {
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

    /// Retrieves a paginated list of source maps matching the specified filter criteria.
    pub async fn list_sourcemaps_with_http_info(
        &self,
        params: ListSourcemapsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListSourcemapsResponse>,
        datadog::Error<ListSourcemapsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_sourcemaps";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_sourcemaps' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let mapkind = params.mapkind;
        let page_size = params.page_size;
        let page_number = params.page_number;
        let filter_service = params.filter_service;
        let filter_version = params.filter_version;
        let filter_variant = params.filter_variant;
        let filter_id = params.filter_id;
        let filter_build_id = params.filter_build_id;
        let filter_uuid = params.filter_uuid;
        let filter_platform = params.filter_platform;
        let filter_build_number = params.filter_build_number;
        let filter_bundle_name = params.filter_bundle_name;
        let filter_arch = params.filter_arch;
        let filter_symbol_source = params.filter_symbol_source;
        let filter_origin = params.filter_origin;
        let filter_origin_version = params.filter_origin_version;
        let filter_filename = params.filter_filename;
        let filter_debug_id = params.filter_debug_id;
        let filter_gnu_build_id = params.filter_gnu_build_id;
        let filter_go_build_id = params.filter_go_build_id;
        let filter_file_hash = params.filter_file_hash;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sourcemaps/list",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = mapkind {
            local_req_builder =
                local_req_builder.query(&[("mapkind", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_service {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[service]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[version]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_variant {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[variant]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_id {
            for param in local {
                local_req_builder = local_req_builder.query(&[("filter[id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_id {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_uuid {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[uuid]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_platform {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[platform]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_number {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_number]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_bundle_name {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[bundle_name]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_arch {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[arch]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_symbol_source {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[symbol_source]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin_version]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_filename {
            local_req_builder =
                local_req_builder.query(&[("filter[filename]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_debug_id {
            local_req_builder =
                local_req_builder.query(&[("filter[debug_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_gnu_build_id {
            local_req_builder = local_req_builder
                .query(&[("filter[gnu_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_go_build_id {
            local_req_builder =
                local_req_builder.query(&[("filter[go_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_file_hash {
            local_req_builder =
                local_req_builder.query(&[("filter[file_hash]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ListSourcemapsResponse>(
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
            let local_entity: Option<ListSourcemapsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Restores previously deleted source maps matching the specified filter
    /// criteria. Supports dry-run mode to preview which source maps would be
    /// restored without performing the actual restoration.
    pub async fn restore_sourcemaps(
        &self,
        mapkind: crate::datadogV2::model::SourcemapMapKind,
        dry_run: bool,
        params: RestoreSourcemapsOptionalParams,
    ) -> Result<crate::datadogV2::model::SourcemapsResponse, datadog::Error<RestoreSourcemapsError>>
    {
        match self
            .restore_sourcemaps_with_http_info(mapkind, dry_run, params)
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

    /// Restores previously deleted source maps matching the specified filter
    /// criteria. Supports dry-run mode to preview which source maps would be
    /// restored without performing the actual restoration.
    pub async fn restore_sourcemaps_with_http_info(
        &self,
        mapkind: crate::datadogV2::model::SourcemapMapKind,
        dry_run: bool,
        params: RestoreSourcemapsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SourcemapsResponse>,
        datadog::Error<RestoreSourcemapsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.restore_sourcemaps";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.restore_sourcemaps' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_service = params.filter_service;
        let filter_version = params.filter_version;
        let filter_variant = params.filter_variant;
        let filter_id = params.filter_id;
        let filter_build_id = params.filter_build_id;
        let filter_uuid = params.filter_uuid;
        let filter_platform = params.filter_platform;
        let filter_build_number = params.filter_build_number;
        let filter_bundle_name = params.filter_bundle_name;
        let filter_arch = params.filter_arch;
        let filter_symbol_source = params.filter_symbol_source;
        let filter_origin = params.filter_origin;
        let filter_origin_version = params.filter_origin_version;
        let filter_filename = params.filter_filename;
        let filter_debug_id = params.filter_debug_id;
        let filter_gnu_build_id = params.filter_gnu_build_id;
        let filter_go_build_id = params.filter_go_build_id;
        let filter_file_hash = params.filter_file_hash;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sourcemaps/restore",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("mapkind", &mapkind.to_string())]);
        local_req_builder = local_req_builder.query(&[("dry_run", &dry_run.to_string())]);
        if let Some(ref local) = filter_service {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[service]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[version]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_variant {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[variant]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_id {
            for param in local {
                local_req_builder = local_req_builder.query(&[("filter[id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_id {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_id]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_uuid {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[uuid]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_platform {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[platform]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_build_number {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[build_number]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_bundle_name {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[bundle_name]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_arch {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[arch]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_symbol_source {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[symbol_source]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_origin_version {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[origin_version]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_filename {
            local_req_builder =
                local_req_builder.query(&[("filter[filename]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_debug_id {
            local_req_builder =
                local_req_builder.query(&[("filter[debug_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_gnu_build_id {
            local_req_builder = local_req_builder
                .query(&[("filter[gnu_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_go_build_id {
            local_req_builder =
                local_req_builder.query(&[("filter[go_build_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_file_hash {
            local_req_builder =
                local_req_builder.query(&[("filter[file_hash]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::SourcemapsResponse>(
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
            let local_entity: Option<RestoreSourcemapsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List endpoint returns RUM events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex RUM events filtering and search.
    ///
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn search_rum_events(
        &self,
        body: crate::datadogV2::model::RUMSearchEventsRequest,
    ) -> Result<crate::datadogV2::model::RUMEventsResponse, datadog::Error<SearchRUMEventsError>>
    {
        match self.search_rum_events_with_http_info(body).await {
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

    pub fn search_rum_events_with_pagination(
        &self,
        mut body: crate::datadogV2::model::RUMSearchEventsRequest,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::RUMEvent, datadog::Error<SearchRUMEventsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if body.page.is_none() {
                body.page = Some(crate::datadogV2::model::RUMQueryPageOptions::new());
            }
            if body.page.as_ref().unwrap().limit.is_none() {
                body.page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = body.page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_rum_events( body.clone(),).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }
                if count == 0 {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                body.page.as_mut().unwrap().cursor = Some(after);
            }
        }
    }

    /// List endpoint returns RUM events that match a RUM search query.
    /// [Results are paginated][1].
    ///
    /// Use this endpoint to build complex RUM events filtering and search.
    ///
    /// [1]: <https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination>
    pub async fn search_rum_events_with_http_info(
        &self,
        body: crate::datadogV2::model::RUMSearchEventsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMEventsResponse>,
        datadog::Error<SearchRUMEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_rum_events";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/events/search",
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::RUMEventsResponse>(&local_content)
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
            let local_entity: Option<SearchRUMEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the RUM application with given ID in your organization.
    pub async fn update_rum_application(
        &self,
        id: String,
        body: crate::datadogV2::model::RUMApplicationUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::RUMApplicationResponse,
        datadog::Error<UpdateRUMApplicationError>,
    > {
        match self.update_rum_application_with_http_info(id, body).await {
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

    /// Update the RUM application with given ID in your organization.
    pub async fn update_rum_application_with_http_info(
        &self,
        id: String,
        body: crate::datadogV2::model::RUMApplicationUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RUMApplicationResponse>,
        datadog::Error<UpdateRUMApplicationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_rum_application";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{id}",
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::RUMApplicationResponse>(
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
            let local_entity: Option<UpdateRUMApplicationError> =
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
