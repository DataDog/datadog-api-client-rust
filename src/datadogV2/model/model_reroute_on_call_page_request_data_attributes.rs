// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details about where to reroute the On-Call Page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RerouteOnCallPageRequestDataAttributes {
    /// An optional comment explaining why the Page is being rerouted.
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Information about the target to notify (such as a team or user).
    #[serde(rename = "target")]
    pub target: crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RerouteOnCallPageRequestDataAttributes {
    pub fn new(
        target: crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
    ) -> RerouteOnCallPageRequestDataAttributes {
        RerouteOnCallPageRequestDataAttributes {
            comment: None,
            target,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn comment(mut self, value: String) -> Self {
        self.comment = Some(value);
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

impl<'de> Deserialize<'de> for RerouteOnCallPageRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RerouteOnCallPageRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for RerouteOnCallPageRequestDataAttributesVisitor {
            type Value = RerouteOnCallPageRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comment: Option<String> = None;
                let mut target: Option<
                    crate::datadogV2::model::CreatePageRequestDataAttributesTarget,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comment" => {
                            if v.is_null() {
                                continue;
                            }
                            comment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = RerouteOnCallPageRequestDataAttributes {
                    comment,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RerouteOnCallPageRequestDataAttributesVisitor)
    }
}
