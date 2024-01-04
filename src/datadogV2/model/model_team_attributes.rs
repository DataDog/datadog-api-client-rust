// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamAttributes {
    /// Unicode representation of the avatar for the team, limited to a single grapheme
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option")]
    pub avatar: Option<Option<String>>,
    /// Banner selection for the team
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option")]
    pub banner: Option<Option<i64>>,
    /// Creation date of the team
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Free-form markdown description/content for the team's homepage
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The team's identifier
    #[serde(rename = "handle")]
    pub handle: String,
    /// Collection of hidden modules for the team
    #[serde(rename = "hidden_modules")]
    pub hidden_modules: Option<Vec<String>>,
    /// The number of links belonging to the team
    #[serde(rename = "link_count")]
    pub link_count: Option<i32>,
    /// Modification date of the team
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// The name of the team
    #[serde(rename = "name")]
    pub name: String,
    /// A brief summary of the team, derived from the `description`
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub summary: Option<Option<String>>,
    /// The number of users belonging to the team
    #[serde(rename = "user_count")]
    pub user_count: Option<i32>,
    /// Collection of visible modules for the team
    #[serde(rename = "visible_modules")]
    pub visible_modules: Option<Vec<String>>,
}

impl TeamAttributes {
    pub fn new(handle: String, name: String) -> TeamAttributes {
        TeamAttributes {
            avatar: None,
            banner: None,
            created_at: None,
            description: None,
            handle,
            hidden_modules: None,
            link_count: None,
            modified_at: None,
            name,
            summary: None,
            user_count: None,
            visible_modules: None,
        }
    }
}