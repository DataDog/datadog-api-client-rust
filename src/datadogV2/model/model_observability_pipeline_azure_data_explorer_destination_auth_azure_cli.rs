// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authenticate using the Azure CLI credentials available in the environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
    /// The Azure credential kind. The value should always be `azure_cli`.
    #[serde(rename = "azure_credential_kind")]
    pub azure_credential_kind:
        crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliKind,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
    pub fn new(
        azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliKind,
    ) -> ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
        ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
            azure_credential_kind,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliVisitor {
            type Value = ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_credential_kind: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliKind> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_credential_kind" => {
                            azure_credential_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _azure_credential_kind) = azure_credential_kind {
                                match _azure_credential_kind {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliKind::UnparsedObject(_azure_credential_kind) => {
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
                let azure_credential_kind = azure_credential_kind
                    .ok_or_else(|| M::Error::missing_field("azure_credential_kind"))?;

                let content = ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli {
                    azure_credential_kind,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCliVisitor)
    }
}
