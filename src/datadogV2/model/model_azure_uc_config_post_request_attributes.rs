// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for Azure config Post Request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureUCConfigPostRequestAttributes {
    /// The tenant ID of the azure account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Bill config.
    #[serde(rename = "actual_bill_config")]
    pub actual_bill_config: crate::datadogV2::model::BillConfig,
    /// Bill config.
    #[serde(rename = "amortized_bill_config")]
    pub amortized_bill_config: crate::datadogV2::model::BillConfig,
    /// The client ID of the azure account.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Whether or not the Cloud Cost Management account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The scope of your observed subscription.
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureUCConfigPostRequestAttributes {
    pub fn new(
        account_id: String,
        actual_bill_config: crate::datadogV2::model::BillConfig,
        amortized_bill_config: crate::datadogV2::model::BillConfig,
        client_id: String,
        scope: String,
    ) -> AzureUCConfigPostRequestAttributes {
        AzureUCConfigPostRequestAttributes {
            account_id,
            actual_bill_config,
            amortized_bill_config,
            client_id,
            is_enabled: None,
            scope,
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AzureUCConfigPostRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureUCConfigPostRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AzureUCConfigPostRequestAttributesVisitor {
            type Value = AzureUCConfigPostRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut actual_bill_config: Option<crate::datadogV2::model::BillConfig> = None;
                let mut amortized_bill_config: Option<crate::datadogV2::model::BillConfig> = None;
                let mut client_id: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut scope: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "actual_bill_config" => {
                            actual_bill_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "amortized_bill_config" => {
                            amortized_bill_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let actual_bill_config = actual_bill_config
                    .ok_or_else(|| M::Error::missing_field("actual_bill_config"))?;
                let amortized_bill_config = amortized_bill_config
                    .ok_or_else(|| M::Error::missing_field("amortized_bill_config"))?;
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = AzureUCConfigPostRequestAttributes {
                    account_id,
                    actual_bill_config,
                    amortized_bill_config,
                    client_id,
                    is_enabled,
                    scope,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureUCConfigPostRequestAttributesVisitor)
    }
}
