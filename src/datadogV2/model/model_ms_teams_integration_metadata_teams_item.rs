// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Item in the Microsoft Teams integration metadata teams array.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MSTeamsIntegrationMetadataTeamsItem {
    /// Microsoft Teams channel ID.
    #[serde(rename = "ms_channel_id")]
    pub ms_channel_id: String,
    /// Microsoft Teams channel name.
    #[serde(rename = "ms_channel_name")]
    pub ms_channel_name: String,
    /// Microsoft Teams tenant ID.
    #[serde(rename = "ms_tenant_id")]
    pub ms_tenant_id: String,
    /// URL redirecting to the Microsoft Teams channel.
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MSTeamsIntegrationMetadataTeamsItem {
    pub fn new(
        ms_channel_id: String,
        ms_channel_name: String,
        ms_tenant_id: String,
        redirect_url: String,
    ) -> MSTeamsIntegrationMetadataTeamsItem {
        MSTeamsIntegrationMetadataTeamsItem {
            ms_channel_id,
            ms_channel_name,
            ms_tenant_id,
            redirect_url,
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

impl<'de> Deserialize<'de> for MSTeamsIntegrationMetadataTeamsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MSTeamsIntegrationMetadataTeamsItemVisitor;
        impl<'a> Visitor<'a> for MSTeamsIntegrationMetadataTeamsItemVisitor {
            type Value = MSTeamsIntegrationMetadataTeamsItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ms_channel_id: Option<String> = None;
                let mut ms_channel_name: Option<String> = None;
                let mut ms_tenant_id: Option<String> = None;
                let mut redirect_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ms_channel_id" => {
                            ms_channel_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ms_channel_name" => {
                            ms_channel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ms_tenant_id" => {
                            ms_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_url" => {
                            redirect_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let ms_channel_id =
                    ms_channel_id.ok_or_else(|| M::Error::missing_field("ms_channel_id"))?;
                let ms_channel_name =
                    ms_channel_name.ok_or_else(|| M::Error::missing_field("ms_channel_name"))?;
                let ms_tenant_id =
                    ms_tenant_id.ok_or_else(|| M::Error::missing_field("ms_tenant_id"))?;
                let redirect_url =
                    redirect_url.ok_or_else(|| M::Error::missing_field("redirect_url"))?;

                let content = MSTeamsIntegrationMetadataTeamsItem {
                    ms_channel_id,
                    ms_channel_name,
                    ms_tenant_id,
                    redirect_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MSTeamsIntegrationMetadataTeamsItemVisitor)
    }
}
