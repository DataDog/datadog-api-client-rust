// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team update attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamUpdateAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option")]
    pub banner: Option<Option<i64>>,
    /// An identifier for the color representing the team
    #[serde(rename = "color")]
    pub color: Option<i32>,
    /// Free-form markdown description/content for the team's homepage
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: String,
    /// Collection of hidden modules for the team
    #[serde(rename = "hidden_modules")]
    pub hidden_modules: Option<Vec<String>>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: String,
    /// Collection of visible modules for the team
    #[serde(rename = "visible_modules")]
    pub visible_modules: Option<Vec<String>>,
}

impl TeamUpdateAttributes {
    pub fn new(handle: String, name: String) -> TeamUpdateAttributes {
        TeamUpdateAttributes {
            avatar: None,
            banner: None,
            color: None,
            description: None,
            handle,
            hidden_modules: None,
            name,
            visible_modules: None,
        }
    }

    pub fn avatar(&mut self, value: Option<String>) -> &mut Self {
        self.avatar = Some(value);
        self
    }

    pub fn banner(&mut self, value: Option<i64>) -> &mut Self {
        self.banner = Some(value);
        self
    }

    pub fn color(&mut self, value: i32) -> &mut Self {
        self.color = Some(value);
        self
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn hidden_modules(&mut self, value: Vec<String>) -> &mut Self {
        self.hidden_modules = Some(value);
        self
    }

    pub fn visible_modules(&mut self, value: Vec<String>) -> &mut Self {
        self.visible_modules = Some(value);
        self
    }
}
