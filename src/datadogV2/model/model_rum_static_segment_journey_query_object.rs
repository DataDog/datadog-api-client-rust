// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The journey query object used to compute the static segment user list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumStaticSegmentJourneyQueryObject {
    /// The list of journey nodes defining the query.
    #[serde(rename = "nodes")]
    pub nodes: Vec<crate::datadogV2::model::RumStaticSegmentJourneyNode>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumStaticSegmentJourneyQueryObject {
    pub fn new(
        nodes: Vec<crate::datadogV2::model::RumStaticSegmentJourneyNode>,
    ) -> RumStaticSegmentJourneyQueryObject {
        RumStaticSegmentJourneyQueryObject {
            nodes,
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

impl<'de> Deserialize<'de> for RumStaticSegmentJourneyQueryObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumStaticSegmentJourneyQueryObjectVisitor;
        impl<'a> Visitor<'a> for RumStaticSegmentJourneyQueryObjectVisitor {
            type Value = RumStaticSegmentJourneyQueryObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nodes: Option<Vec<crate::datadogV2::model::RumStaticSegmentJourneyNode>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "nodes" => {
                            nodes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let nodes = nodes.ok_or_else(|| M::Error::missing_field("nodes"))?;

                let content = RumStaticSegmentJourneyQueryObject {
                    nodes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumStaticSegmentJourneyQueryObjectVisitor)
    }
}
