// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updatable attributes of an Elastic Cloud CCM account. Every field is optional; only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudCcmAccountUpdateAttributes {
    /// Authentication methods supported by the Elastic Cloud CCM interface. Exactly one is set, selected by its `type`.
    #[serde(rename = "authentication")]
    pub authentication: Option<crate::datadogV2::model::ElasticCloudCcmAuthentication>,
    /// Dataflows for the Elastic Cloud CCM interface.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<Vec<crate::datadogV2::model::ElasticCloudCcmDataflow>>,
    /// Human-readable name of the account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Partial Elastic Cloud CCM interface settings for updates.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::ElasticCloudCcmSettingsUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudCcmAccountUpdateAttributes {
    pub fn new() -> ElasticCloudCcmAccountUpdateAttributes {
        ElasticCloudCcmAccountUpdateAttributes {
            authentication: None,
            dataflows: None,
            name: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV2::model::ElasticCloudCcmAuthentication,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn dataflows(
        mut self,
        value: Vec<crate::datadogV2::model::ElasticCloudCcmDataflow>,
    ) -> Self {
        self.dataflows = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::ElasticCloudCcmSettingsUpdate,
    ) -> Self {
        self.settings = Some(value);
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

impl Default for ElasticCloudCcmAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ElasticCloudCcmAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudCcmAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ElasticCloudCcmAccountUpdateAttributesVisitor {
            type Value = ElasticCloudCcmAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV2::model::ElasticCloudCcmAuthentication,
                > = None;
                let mut dataflows: Option<Vec<crate::datadogV2::model::ElasticCloudCcmDataflow>> =
                    None;
                let mut name: Option<String> = None;
                let mut settings: Option<crate::datadogV2::model::ElasticCloudCcmSettingsUpdate> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _authentication) = authentication {
                                match _authentication {
                                    crate::datadogV2::model::ElasticCloudCcmAuthentication::UnparsedObject(_authentication) => {
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
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ElasticCloudCcmAccountUpdateAttributes {
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

        deserializer.deserialize_any(ElasticCloudCcmAccountUpdateAttributesVisitor)
    }
}
