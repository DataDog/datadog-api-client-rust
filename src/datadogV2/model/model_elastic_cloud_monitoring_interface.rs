// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Elastic Cloud monitoring interface (source-type) configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudMonitoringInterface {
    /// Authentication methods supported by the Elastic Cloud interface. Exactly one is set, selected by its `type`.
    #[serde(rename = "authentication")]
    pub authentication: crate::datadogV2::model::ElasticCloudAuthentication,
    /// Dataflows for the Elastic Cloud monitoring interface.
    #[serde(rename = "dataflows")]
    pub dataflows: Option<Vec<crate::datadogV2::model::ElasticCloudDataflow>>,
    /// Elastic Cloud interface settings.
    #[serde(rename = "settings")]
    pub settings: Option<crate::datadogV2::model::ElasticCloudSettings>,
    /// Interface discriminator for the Elastic Cloud monitoring interface.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ElasticCloudMonitoringInterfaceType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudMonitoringInterface {
    pub fn new(
        authentication: crate::datadogV2::model::ElasticCloudAuthentication,
        type_: crate::datadogV2::model::ElasticCloudMonitoringInterfaceType,
    ) -> ElasticCloudMonitoringInterface {
        ElasticCloudMonitoringInterface {
            authentication,
            dataflows: None,
            settings: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataflows(mut self, value: Vec<crate::datadogV2::model::ElasticCloudDataflow>) -> Self {
        self.dataflows = Some(value);
        self
    }

    pub fn settings(mut self, value: crate::datadogV2::model::ElasticCloudSettings) -> Self {
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

impl<'de> Deserialize<'de> for ElasticCloudMonitoringInterface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudMonitoringInterfaceVisitor;
        impl<'a> Visitor<'a> for ElasticCloudMonitoringInterfaceVisitor {
            type Value = ElasticCloudMonitoringInterface;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV2::model::ElasticCloudAuthentication,
                > = None;
                let mut dataflows: Option<Vec<crate::datadogV2::model::ElasticCloudDataflow>> =
                    None;
                let mut settings: Option<crate::datadogV2::model::ElasticCloudSettings> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ElasticCloudMonitoringInterfaceType,
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
                                    crate::datadogV2::model::ElasticCloudAuthentication::UnparsedObject(_authentication) => {
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
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ElasticCloudMonitoringInterfaceType::UnparsedObject(_type_) => {
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
                let authentication =
                    authentication.ok_or_else(|| M::Error::missing_field("authentication"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ElasticCloudMonitoringInterface {
                    authentication,
                    dataflows,
                    settings,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudMonitoringInterfaceVisitor)
    }
}
