// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GroupStyleRule {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "match")]
    pub match_: Option<crate::datadogV1_20270101::model::GroupStyleRuleMatch>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette_index")]
    pub palette_index: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GroupStyleRule {
    pub fn new(palette: String) -> GroupStyleRule {
        GroupStyleRule {
            match_: None,
            palette,
            palette_index: None,
            _unparsed: false,
        }
    }
    
    pub fn match_(mut self, value: crate::datadogV1_20270101::model::GroupStyleRuleMatch) -> Self {
        self.match_ = Some(value);
        self
    }
    
    pub fn palette_index(mut self, value: f64) -> Self {
        self.palette_index = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GroupStyleRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GroupStyleRuleVisitor;
        impl<'a> Visitor<'a> for GroupStyleRuleVisitor {
            type Value = GroupStyleRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_: Option<crate::datadogV1_20270101::model::GroupStyleRuleMatch> = None;
                let mut palette: Option<String> = None;
                let mut palette_index: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match" => {
                            if v.is_null() {
                                continue;
                            }
                            match_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette" => {
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette_index" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            palette_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let palette = palette.ok_or_else(|| M::Error::missing_field("palette"))?;

                let content = GroupStyleRule {
                    match_,
                    palette,
                    palette_index,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GroupStyleRuleVisitor)
    }
}
