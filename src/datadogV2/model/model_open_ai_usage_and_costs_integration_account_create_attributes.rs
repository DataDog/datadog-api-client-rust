// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to create an OpenAI Usage and Costs integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
    /// Authentication for creating the OpenAI Usage and Costs integration account. Set exactly one method.
    #[serde(rename = "authentication")]
    pub authentication:
        crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest,
    /// Data Datadog collects from OpenAI Usage and Costs, keyed by dataflow ID. Datadog always collects organization usage metrics. Each dataflow in this object adds another type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<crate::datadogV2::model::OpenAIUsageAndCostsIntegrationDataflowsRequest>,
    /// Human-readable name of the OpenAI Usage and Costs integration account.
    #[serde(rename = "name")]
    pub name: String,
    /// Settings for creating the OpenAI Usage and Costs integration account.
    #[serde(rename = "settings")]
    pub settings: crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountSettingsRequest,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
    pub fn new(
        authentication: crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest,
        name: String,
        settings: crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountSettingsRequest,
    ) -> OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
        OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
            authentication,
            dataflows: None,
            name,
            settings,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataflows(
        mut self,
        value: crate::datadogV2::model::OpenAIUsageAndCostsIntegrationDataflowsRequest,
    ) -> Self {
        self.dataflows = Some(value);
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

impl<'de> Deserialize<'de> for OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIUsageAndCostsIntegrationAccountCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OpenAIUsageAndCostsIntegrationAccountCreateAttributesVisitor {
            type Value = OpenAIUsageAndCostsIntegrationAccountCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest> = None;
                let mut dataflows: Option<
                    crate::datadogV2::model::OpenAIUsageAndCostsIntegrationDataflowsRequest,
                > = None;
                let mut name: Option<String> = None;
                let mut settings: Option<
                    crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountSettingsRequest,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::OpenAIUsageAndCostsIntegrationAccountAuthenticationRequest::UnparsedObject(_authentication) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "dataflows" => {
                            if v.is_null() {
                                continue;
                            }
                            dataflows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let authentication =
                    authentication.ok_or_else(|| M::Error::missing_field("authentication"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let settings = settings.ok_or_else(|| M::Error::missing_field("settings"))?;

                let content = OpenAIUsageAndCostsIntegrationAccountCreateAttributes {
                    authentication,
                    dataflows,
                    name,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpenAIUsageAndCostsIntegrationAccountCreateAttributesVisitor)
    }
}
