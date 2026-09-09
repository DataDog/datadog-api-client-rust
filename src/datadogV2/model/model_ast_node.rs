// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A node in the abstract syntax tree of the parsed source code.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AstNode {
    /// The tree-sitter node type of this AST node.
    #[serde(rename = "ast_type")]
    pub ast_type: String,
    /// The child nodes of this AST node, or null for a leaf node.
    #[serialize_always]
    #[serde(rename = "children")]
    pub children: Option<Vec<crate::datadogV2::model::AstNode>>,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "end")]
    pub end: crate::datadogV2::model::AnalysisPosition,
    /// The name of the field this node occupies within its parent node, when the parent addresses it by name.
    #[serde(rename = "field_name")]
    pub field_name: Option<String>,
    /// A position in source code, identified by line and column numbers.
    #[serde(rename = "start")]
    pub start: crate::datadogV2::model::AnalysisPosition,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AstNode {
    pub fn new(
        ast_type: String,
        children: Option<Vec<crate::datadogV2::model::AstNode>>,
        end: crate::datadogV2::model::AnalysisPosition,
        start: crate::datadogV2::model::AnalysisPosition,
    ) -> AstNode {
        AstNode {
            ast_type,
            children,
            end,
            field_name: None,
            start,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn field_name(mut self, value: String) -> Self {
        self.field_name = Some(value);
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

impl<'de> Deserialize<'de> for AstNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AstNodeVisitor;
        impl<'a> Visitor<'a> for AstNodeVisitor {
            type Value = AstNode;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ast_type: Option<String> = None;
                let mut children: Option<Option<Vec<crate::datadogV2::model::AstNode>>> = None;
                let mut end: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut field_name: Option<String> = None;
                let mut start: Option<crate::datadogV2::model::AnalysisPosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ast_type" => {
                            ast_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "children" => {
                            children = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_name" => {
                            if v.is_null() {
                                continue;
                            }
                            field_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let ast_type = ast_type.ok_or_else(|| M::Error::missing_field("ast_type"))?;
                let children = children.ok_or_else(|| M::Error::missing_field("children"))?;
                let end = end.ok_or_else(|| M::Error::missing_field("end"))?;
                let start = start.ok_or_else(|| M::Error::missing_field("start"))?;

                let content = AstNode {
                    ast_type,
                    children,
                    end,
                    field_name,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AstNodeVisitor)
    }
}
