// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `databricks_zerobus` destination sends logs to Databricks using the Zerobus ingestion API, streaming data directly into your Databricks Lakehouse.
///
/// **Supported pipeline types:** Logs, rehydration
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineDatabricksZerobusDestination {
    /// OAuth credentials for authenticating with the Databricks Zerobus ingestion API.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationAuth,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// Your Databricks Zerobus ingestion endpoint. This is the endpoint used to stream data directly into your Databricks Lakehouse.
    #[serde(rename = "ingestion_endpoint")]
    pub ingestion_endpoint: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The fully qualified name of your target Databricks table. Make sure this table already exists in your Databricks workspace before deploying.
    #[serde(rename = "table_name")]
    pub table_name: String,
    /// The destination type. The value must be `databricks_zerobus`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationType,
    /// Your Databricks workspace URL. This is used to communicate with the Unity Catalog API.
    #[serde(rename = "unity_catalog_endpoint")]
    pub unity_catalog_endpoint: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineDatabricksZerobusDestination {
    pub fn new(
        auth: crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationAuth,
        id: String,
        ingestion_endpoint: String,
        inputs: Vec<String>,
        table_name: String,
        type_: crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationType,
        unity_catalog_endpoint: String,
    ) -> ObservabilityPipelineDatabricksZerobusDestination {
        ObservabilityPipelineDatabricksZerobusDestination {
            auth,
            buffer: None,
            id,
            ingestion_endpoint,
            inputs,
            table_name,
            type_,
            unity_catalog_endpoint,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineDatabricksZerobusDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineDatabricksZerobusDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineDatabricksZerobusDestinationVisitor {
            type Value = ObservabilityPipelineDatabricksZerobusDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<
                    crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationAuth,
                > = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut id: Option<String> = None;
                let mut ingestion_endpoint: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut table_name: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationType,
                > = None;
                let mut unity_catalog_endpoint: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ingestion_endpoint" => {
                            ingestion_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineDatabricksZerobusDestinationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unity_catalog_endpoint" => {
                            unity_catalog_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let ingestion_endpoint = ingestion_endpoint
                    .ok_or_else(|| M::Error::missing_field("ingestion_endpoint"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let unity_catalog_endpoint = unity_catalog_endpoint
                    .ok_or_else(|| M::Error::missing_field("unity_catalog_endpoint"))?;

                let content = ObservabilityPipelineDatabricksZerobusDestination {
                    auth,
                    buffer,
                    id,
                    ingestion_endpoint,
                    inputs,
                    table_name,
                    type_,
                    unity_catalog_endpoint,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineDatabricksZerobusDestinationVisitor)
    }
}
