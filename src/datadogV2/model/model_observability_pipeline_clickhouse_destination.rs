// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `clickhouse` destination sends log events to a ClickHouse database table over HTTP.
///
/// **Supported pipeline types:** logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClickhouseDestination {
    /// HTTP Basic Authentication credentials for the ClickHouse destination.
    /// When `strategy` is `basic`, provide `username_key` and `password_key` that reference environment variables or secrets containing the credentials.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuth>,
    /// Batching configuration for ClickHouse inserts.
    #[serde(rename = "batch")]
    pub batch: Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatch>,
    /// Batch encoding configuration for the ClickHouse destination.
    /// Required when `format` is `arrow_stream`. The `codec` field must be set to `arrow_stream`.
    #[serde(rename = "batch_encoding")]
    pub batch_encoding:
        Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncoding>,
    /// Compression setting for outbound HTTP requests to ClickHouse.
    /// Can be specified as a shorthand string (`"gzip"` or `"none"`) or as an object
    /// with an `algorithm` field and an optional `level` (gzip only, 1–9).
    #[serde(rename = "compression")]
    pub compression:
        Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression>,
    /// Optional ClickHouse database name. If omitted, the user's default database on the ClickHouse server is used.
    #[serde(rename = "database")]
    pub database: Option<String>,
    /// When `true`, enables flexible DateTime parsing on the ClickHouse server side.
    #[serde(rename = "date_time_best_effort")]
    pub date_time_best_effort: Option<bool>,
    /// Name of the environment variable or secret that contains the ClickHouse HTTP endpoint URL.
    /// Defaults to `DESTINATION_CLICKHOUSE_ENDPOINT_URL` (prefixed with `DD_OP_` at runtime).
    #[serde(rename = "endpoint_url_key")]
    pub endpoint_url_key: Option<String>,
    /// Insert format for events sent to ClickHouse.
    /// - `json_each_row`: Maps event fields to columns by name (ClickHouse `JSONEachRow`).
    /// - `json_as_object`: Inserts each event into a single `Object('json')` / `JSON` column (ClickHouse `JSONAsObject`).
    /// - `json_as_string`: Inserts each event into a single `String`-typed column as raw JSON (ClickHouse `JSONAsString`).
    /// - `arrow_stream`: Batches events using Apache Arrow IPC streaming format. Requires `batch_encoding`.
    #[serde(rename = "format")]
    pub format: Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationFormat>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// When `true`, fields not present in the target table schema are dropped instead of causing insert errors.
    /// When unset, the ClickHouse server's own `input_format_skip_unknown_fields` setting applies.
    #[serde(
        rename = "skip_unknown_fields",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub skip_unknown_fields: Option<Option<bool>>,
    /// Target ClickHouse table name. Events are inserted into this table.
    #[serde(rename = "table")]
    pub table: String,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The destination type. The value must be `clickhouse`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClickhouseDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        table: String,
        type_: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationType,
    ) -> ObservabilityPipelineClickhouseDestination {
        ObservabilityPipelineClickhouseDestination {
            auth: None,
            batch: None,
            batch_encoding: None,
            compression: None,
            database: None,
            date_time_best_effort: None,
            endpoint_url_key: None,
            format: None,
            id,
            inputs,
            skip_unknown_fields: None,
            table,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuth,
    ) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn batch(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatch,
    ) -> Self {
        self.batch = Some(value);
        self
    }

    pub fn batch_encoding(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncoding,
    ) -> Self {
        self.batch_encoding = Some(value);
        self
    }

    pub fn compression(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression,
    ) -> Self {
        self.compression = Some(value);
        self
    }

    pub fn database(mut self, value: String) -> Self {
        self.database = Some(value);
        self
    }

    pub fn date_time_best_effort(mut self, value: bool) -> Self {
        self.date_time_best_effort = Some(value);
        self
    }

    pub fn endpoint_url_key(mut self, value: String) -> Self {
        self.endpoint_url_key = Some(value);
        self
    }

    pub fn format(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationFormat,
    ) -> Self {
        self.format = Some(value);
        self
    }

    pub fn skip_unknown_fields(mut self, value: Option<bool>) -> Self {
        self.skip_unknown_fields = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClickhouseDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClickhouseDestinationVisitor {
            type Value = ObservabilityPipelineClickhouseDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuth,
                > = None;
                let mut batch: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatch,
                > = None;
                let mut batch_encoding: Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncoding> = None;
                let mut compression: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression,
                > = None;
                let mut database: Option<String> = None;
                let mut date_time_best_effort: Option<bool> = None;
                let mut endpoint_url_key: Option<String> = None;
                let mut format: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationFormat,
                > = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut skip_unknown_fields: Option<Option<bool>> = None;
                let mut table: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "batch" => {
                            if v.is_null() {
                                continue;
                            }
                            batch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "batch_encoding" => {
                            if v.is_null() {
                                continue;
                            }
                            batch_encoding =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compression" => {
                            if v.is_null() {
                                continue;
                            }
                            compression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _compression) = compression {
                                match _compression {
                                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompression::UnparsedObject(_compression) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "database" => {
                            if v.is_null() {
                                continue;
                            }
                            database = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date_time_best_effort" => {
                            if v.is_null() {
                                continue;
                            }
                            date_time_best_effort =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "endpoint_url_key" => {
                            if v.is_null() {
                                continue;
                            }
                            endpoint_url_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "format" => {
                            if v.is_null() {
                                continue;
                            }
                            format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _format) = format {
                                match _format {
                                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationFormat::UnparsedObject(_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "skip_unknown_fields" => {
                            skip_unknown_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table" => {
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let table = table.ok_or_else(|| M::Error::missing_field("table"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineClickhouseDestination {
                    auth,
                    batch,
                    batch_encoding,
                    compression,
                    database,
                    date_time_best_effort,
                    endpoint_url_key,
                    format,
                    id,
                    inputs,
                    skip_unknown_fields,
                    table,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClickhouseDestinationVisitor)
    }
}
