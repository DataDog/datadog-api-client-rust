// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsMobileStepType {
    ASSERTELEMENTCONTENT,
    ASSERTSCREENCONTAINS,
    ASSERTSCREENLACKS,
    DOUBLETAP,
    EXTRACTVARIABLE,
    FLICK,
    OPENDEEPLINK,
    PLAYSUBTEST,
    PRESSBACK,
    RESTARTAPPLICATION,
    ROTATE,
    SCROLL,
    SCROLLTOELEMENT,
    TAP,
    TOGGLEWIFI,
    TYPETEXT,
    WAIT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsMobileStepType {
    fn to_string(&self) -> String {
        match self {
            Self::ASSERTELEMENTCONTENT => String::from("assertElementContent"),
            Self::ASSERTSCREENCONTAINS => String::from("assertScreenContains"),
            Self::ASSERTSCREENLACKS => String::from("assertScreenLacks"),
            Self::DOUBLETAP => String::from("doubleTap"),
            Self::EXTRACTVARIABLE => String::from("extractVariable"),
            Self::FLICK => String::from("flick"),
            Self::OPENDEEPLINK => String::from("openDeeplink"),
            Self::PLAYSUBTEST => String::from("playSubTest"),
            Self::PRESSBACK => String::from("pressBack"),
            Self::RESTARTAPPLICATION => String::from("restartApplication"),
            Self::ROTATE => String::from("rotate"),
            Self::SCROLL => String::from("scroll"),
            Self::SCROLLTOELEMENT => String::from("scrollToElement"),
            Self::TAP => String::from("tap"),
            Self::TOGGLEWIFI => String::from("toggleWiFi"),
            Self::TYPETEXT => String::from("typeText"),
            Self::WAIT => String::from("wait"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsMobileStepType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "assertElementContent" => Self::ASSERTELEMENTCONTENT,
            "assertScreenContains" => Self::ASSERTSCREENCONTAINS,
            "assertScreenLacks" => Self::ASSERTSCREENLACKS,
            "doubleTap" => Self::DOUBLETAP,
            "extractVariable" => Self::EXTRACTVARIABLE,
            "flick" => Self::FLICK,
            "openDeeplink" => Self::OPENDEEPLINK,
            "playSubTest" => Self::PLAYSUBTEST,
            "pressBack" => Self::PRESSBACK,
            "restartApplication" => Self::RESTARTAPPLICATION,
            "rotate" => Self::ROTATE,
            "scroll" => Self::SCROLL,
            "scrollToElement" => Self::SCROLLTOELEMENT,
            "tap" => Self::TAP,
            "toggleWiFi" => Self::TOGGLEWIFI,
            "typeText" => Self::TYPETEXT,
            "wait" => Self::WAIT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
