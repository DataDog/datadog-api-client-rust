// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A notification target that receives change alerts for a feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotificationRuleTarget {
    /// Configuration for a notification target. Which fields apply depends on the target's `type`.
    #[serde(rename = "configuration")]
    pub configuration: crate::datadogV2::model::NotificationRuleTargetConfiguration,
    /// The type of notification target.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::NotificationRuleTargetType,
    /// Schema version of `configuration`.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationRuleTarget {
    pub fn new(
        configuration: crate::datadogV2::model::NotificationRuleTargetConfiguration,
        type_: crate::datadogV2::model::NotificationRuleTargetType,
        version: i64,
    ) -> NotificationRuleTarget {
        NotificationRuleTarget {
            configuration,
            type_,
            version,
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

impl<'de> Deserialize<'de> for NotificationRuleTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationRuleTargetVisitor;
        impl<'a> Visitor<'a> for NotificationRuleTargetVisitor {
            type Value = NotificationRuleTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration: Option<
                    crate::datadogV2::model::NotificationRuleTargetConfiguration,
                > = None;
                let mut type_: Option<crate::datadogV2::model::NotificationRuleTargetType> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration" => {
                            configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::NotificationRuleTargetType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let configuration =
                    configuration.ok_or_else(|| M::Error::missing_field("configuration"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = NotificationRuleTarget {
                    configuration,
                    type_,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationRuleTargetVisitor)
    }
}
