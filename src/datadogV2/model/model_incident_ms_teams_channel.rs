// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Microsoft Teams channel associated with an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentMSTeamsChannel {
    /// The Teams channel identifier.
    #[serde(rename = "ms_channel_id")]
    pub ms_channel_id: String,
    /// The name of the Teams channel.
    #[serde(rename = "ms_channel_name")]
    pub ms_channel_name: String,
    /// The Teams team identifier.
    #[serde(rename = "ms_team_id")]
    pub ms_team_id: Option<String>,
    /// The Teams tenant identifier.
    #[serde(rename = "ms_tenant_id")]
    pub ms_tenant_id: Option<String>,
    /// The redirect URL for the channel.
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
    /// The name of the Teams team.
    #[serde(rename = "team_name")]
    pub team_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentMSTeamsChannel {
    pub fn new(ms_channel_id: String, ms_channel_name: String) -> IncidentMSTeamsChannel {
        IncidentMSTeamsChannel {
            ms_channel_id,
            ms_channel_name,
            ms_team_id: None,
            ms_tenant_id: None,
            redirect_url: None,
            team_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ms_team_id(mut self, value: String) -> Self {
        self.ms_team_id = Some(value);
        self
    }

    pub fn ms_tenant_id(mut self, value: String) -> Self {
        self.ms_tenant_id = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: String) -> Self {
        self.redirect_url = Some(value);
        self
    }

    pub fn team_name(mut self, value: String) -> Self {
        self.team_name = Some(value);
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

impl<'de> Deserialize<'de> for IncidentMSTeamsChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentMSTeamsChannelVisitor;
        impl<'a> Visitor<'a> for IncidentMSTeamsChannelVisitor {
            type Value = IncidentMSTeamsChannel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ms_channel_id: Option<String> = None;
                let mut ms_channel_name: Option<String> = None;
                let mut ms_team_id: Option<String> = None;
                let mut ms_tenant_id: Option<String> = None;
                let mut redirect_url: Option<String> = None;
                let mut team_name: Option<String> = None;
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
                        "ms_team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            ms_team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ms_tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            ms_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_url" => {
                            if v.is_null() {
                                continue;
                            }
                            redirect_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_name" => {
                            if v.is_null() {
                                continue;
                            }
                            team_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentMSTeamsChannel {
                    ms_channel_id,
                    ms_channel_name,
                    ms_team_id,
                    ms_tenant_id,
                    redirect_url,
                    team_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentMSTeamsChannelVisitor)
    }
}
