// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Writable attributes used to create an Anthropic Usage and Costs integration account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
    /// Authentication for creating the Anthropic Usage and Costs integration account. Exactly one method is set.
    #[serde(rename = "authentication")]
    pub authentication:
        crate::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest,
    /// Data Datadog collects from Anthropic Usage and Costs, keyed by dataflow ID. Datadog always collects usage metrics. Each dataflow in this object adds another type of collection. Set `enabled` to start or stop a dataflow. The default for each dataflow applies when the account is created. On update, omitted fields keep their current values. If a dataflow depends on a Datadog product, its description names that product. If you enable a dataflow without that product, Datadog stores the setting but collects no data.
    #[serde(rename = "dataflows")]
    pub dataflows:
        Option<crate::datadogV2::model::AnthropicUsageAndCostsIntegrationDataflowsRequest>,
    /// Human-readable name of the Anthropic Usage and Costs integration account.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
    pub fn new(
        authentication: crate::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest,
        name: String,
    ) -> AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
        AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
            authentication,
            dataflows: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataflows(
        mut self,
        value: crate::datadogV2::model::AnthropicUsageAndCostsIntegrationDataflowsRequest,
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

impl<'de> Deserialize<'de> for AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnthropicUsageAndCostsIntegrationAccountCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AnthropicUsageAndCostsIntegrationAccountCreateAttributesVisitor {
            type Value = AnthropicUsageAndCostsIntegrationAccountCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<crate::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest> = None;
                let mut dataflows: Option<
                    crate::datadogV2::model::AnthropicUsageAndCostsIntegrationDataflowsRequest,
                > = None;
                let mut name: Option<String> = None;
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
                                    crate::datadogV2::model::AnthropicUsageAndCostsIntegrationAccountAuthenticationRequest::UnparsedObject(_authentication) => {
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

                let content = AnthropicUsageAndCostsIntegrationAccountCreateAttributes {
                    authentication,
                    dataflows,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(AnthropicUsageAndCostsIntegrationAccountCreateAttributesVisitor)
    }
}
