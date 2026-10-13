// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `azure_data_explorer` destination sends log events to an Azure Data Explorer table.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestination {
    /// Authentication configuration for Azure Data Explorer. The `azure_credential_kind` field selects the credential type.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuth,
    /// Event batching settings for Azure Data Explorer ingestion.
    #[serde(rename = "batch")]
    pub batch:
        Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationBatch>,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// Gzip compression.
    #[serde(rename = "compression")]
    pub compression: Option<
        crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionGzip,
    >,
    /// The name of the Azure Data Explorer database to ingest into. Supports template syntax.
    #[serde(rename = "database")]
    pub database: String,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the environment variable or secret that holds the Azure Data Explorer ingestion endpoint URL.
    /// Defaults to `DESTINATION_AZURE_DATA_EXPLORER_INGESTION_ENDPOINT` (prefixed with `DD_OP_` at runtime).
    #[serde(rename = "ingestion_endpoint_key")]
    pub ingestion_endpoint_key: Option<String>,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The name of a pre-created ingestion mapping on the table used to map incoming events to columns. Supports template syntax.
    #[serde(
        rename = "mapping_reference",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub mapping_reference: Option<Option<String>>,
    /// The name of the Azure Data Explorer table to ingest into. Supports template syntax.
    #[serde(rename = "table")]
    pub table: String,
    /// The OAuth scope requested when acquiring an access token for Azure Data Explorer.
    /// Defaults to `<https://kusto.kusto.windows.net/.default`.>
    #[serde(rename = "token_scope")]
    pub token_scope: Option<String>,
    /// The destination type. The value should always be `azure_data_explorer`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAzureDataExplorerDestination {
    pub fn new(
        auth: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuth,
        database: String,
        id: String,
        inputs: Vec<String>,
        table: String,
        type_: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationType,
    ) -> ObservabilityPipelineAzureDataExplorerDestination {
        ObservabilityPipelineAzureDataExplorerDestination {
            auth,
            batch: None,
            buffer: None,
            compression: None,
            database,
            id,
            ingestion_endpoint_key: None,
            inputs,
            mapping_reference: None,
            table,
            token_scope: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn batch(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationBatch,
    ) -> Self {
        self.batch = Some(value);
        self
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn compression(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionGzip,
    ) -> Self {
        self.compression = Some(value);
        self
    }

    pub fn ingestion_endpoint_key(mut self, value: String) -> Self {
        self.ingestion_endpoint_key = Some(value);
        self
    }

    pub fn mapping_reference(mut self, value: Option<String>) -> Self {
        self.mapping_reference = Some(value);
        self
    }

    pub fn token_scope(mut self, value: String) -> Self {
        self.token_scope = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAzureDataExplorerDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAzureDataExplorerDestinationVisitor {
            type Value = ObservabilityPipelineAzureDataExplorerDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuth,
                > = None;
                let mut batch: Option<
                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationBatch,
                > = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut compression: Option<crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionGzip> = None;
                let mut database: Option<String> = None;
                let mut id: Option<String> = None;
                let mut ingestion_endpoint_key: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut mapping_reference: Option<Option<String>> = None;
                let mut table: Option<String> = None;
                let mut token_scope: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth) = auth {
                                match _auth {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuth::UnparsedObject(_auth) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "batch" => {
                            if v.is_null() {
                                continue;
                            }
                            batch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "buffer" => {
                            if v.is_null() {
                                continue;
                            }
                            buffer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _buffer) = buffer {
                                match _buffer {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptions::UnparsedObject(_buffer) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "compression" => {
                            if v.is_null() {
                                continue;
                            }
                            compression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "database" => {
                            database = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingestion_endpoint_key" => {
                            if v.is_null() {
                                continue;
                            }
                            ingestion_endpoint_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapping_reference" => {
                            mapping_reference =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table" => {
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            token_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationType::UnparsedObject(_type_) => {
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
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let database = database.ok_or_else(|| M::Error::missing_field("database"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let table = table.ok_or_else(|| M::Error::missing_field("table"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineAzureDataExplorerDestination {
                    auth,
                    batch,
                    buffer,
                    compression,
                    database,
                    id,
                    ingestion_endpoint_key,
                    inputs,
                    mapping_reference,
                    table,
                    token_scope,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAzureDataExplorerDestinationVisitor)
    }
}
