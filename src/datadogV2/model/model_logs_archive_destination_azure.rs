// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Azure archive destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveDestinationAzure {
    /// The container where the archive will be stored.
    #[serde(rename = "container")]
    pub container: String,
    /// The Azure archive's integration destination.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::LogsArchiveIntegrationAzure,
    /// The archive path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// The region where the archive will be stored.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The associated storage account.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// Type of the Azure archive destination.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::LogsArchiveDestinationAzureType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveDestinationAzure {
    pub fn new(
        container: String,
        integration: crate::datadogV2::model::LogsArchiveIntegrationAzure,
        storage_account: String,
        type_: crate::datadogV2::model::LogsArchiveDestinationAzureType,
    ) -> LogsArchiveDestinationAzure {
        LogsArchiveDestinationAzure {
            container,
            integration,
            path: None,
            region: None,
            storage_account,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
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

impl<'de> Deserialize<'de> for LogsArchiveDestinationAzure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveDestinationAzureVisitor;
        impl<'a> Visitor<'a> for LogsArchiveDestinationAzureVisitor {
            type Value = LogsArchiveDestinationAzure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut container: Option<String> = None;
                let mut integration: Option<crate::datadogV2::model::LogsArchiveIntegrationAzure> =
                    None;
                let mut path: Option<String> = None;
                let mut region: Option<String> = None;
                let mut storage_account: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::LogsArchiveDestinationAzureType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "container" => {
                            container = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            if v.is_null() {
                                continue;
                            }
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_account" => {
                            storage_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::LogsArchiveDestinationAzureType::UnparsedObject(_type_) => {
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
                let container = container.ok_or_else(|| M::Error::missing_field("container"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let storage_account =
                    storage_account.ok_or_else(|| M::Error::missing_field("storage_account"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArchiveDestinationAzure {
                    container,
                    integration,
                    path,
                    region,
                    storage_account,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveDestinationAzureVisitor)
    }
}
