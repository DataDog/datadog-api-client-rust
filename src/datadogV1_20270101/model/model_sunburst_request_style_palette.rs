// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SunburstRequestStylePalette {
	SUCCESS,
	RED,
	GREEN,
	ORANGE,
	GRAY,
	DOG_CLASSIC,
	DOG_CLASSIC_AREA,
	DOG_CLASSIC_LINE,
	DOG_CLASSIC_BARS,
	DOG_HIGHLIGHT,
	MONITOR_STATE,
	YELLOW_TO_GREEN,
	GREEN_TO_ORANGE,
	HOSTMAP_BLUES,
	HOSTMAP_GREEN_RED,
	HOSTMAP_GREEN_ORANGE_RED,
	HOSTMAP_BLUE_RED,
	DARK_GREY,
	PRIDE,
	GNBU,
	YLORRD,
	PURPLES,
	BLUES,
	ORANGES,
	REDS,
	GREENS,
	GREYS,
	STATUS_ERRORS,
	STATUS_WARNINGS,
	STATUS_SUCCESSES,
	PLASMA,
	VIRIDIS,
	DOGCAT,
	DATADOG16,
	PA_CLASSIC_LINE,
	PA_CLASSIC_AREA,
	PA_CLASSIC_BARS,
	PA_DATADOG16,
	CLASSIC,
	COOL_HM,
	WARM_HM,
	YELLOW___GREEN,
	GREEN___ORANGE,
	GREEN___RED,
	GREEN___ORANGE___RED,
	BLUE___RED,
	GOOD___BAD,
	COOL,
	OCEAN,
	WARM,
	FIRE,
	WARN,
	PURPLE,
	GREY,
	BLUE,
	DD20,
	SEMANTIC,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SunburstRequestStylePalette {
    fn to_string(&self) -> String {
        match self {
            Self::SUCCESS => String::from("success"),
            Self::RED => String::from("red"),
            Self::GREEN => String::from("green"),
            Self::ORANGE => String::from("orange"),
            Self::GRAY => String::from("gray"),
            Self::DOG_CLASSIC => String::from("dog_classic"),
            Self::DOG_CLASSIC_AREA => String::from("dog_classic_area"),
            Self::DOG_CLASSIC_LINE => String::from("dog_classic_line"),
            Self::DOG_CLASSIC_BARS => String::from("dog_classic_bars"),
            Self::DOG_HIGHLIGHT => String::from("dog_highlight"),
            Self::MONITOR_STATE => String::from("monitor_state"),
            Self::YELLOW_TO_GREEN => String::from("yellow_to_green"),
            Self::GREEN_TO_ORANGE => String::from("green_to_orange"),
            Self::HOSTMAP_BLUES => String::from("hostmap_blues"),
            Self::HOSTMAP_GREEN_RED => String::from("hostmap_green_red"),
            Self::HOSTMAP_GREEN_ORANGE_RED => String::from("hostmap_green_orange_red"),
            Self::HOSTMAP_BLUE_RED => String::from("hostmap_blue_red"),
            Self::DARK_GREY => String::from("dark_grey"),
            Self::PRIDE => String::from("pride"),
            Self::GNBU => String::from("GnBu"),
            Self::YLORRD => String::from("YlOrRd"),
            Self::PURPLES => String::from("Purples"),
            Self::BLUES => String::from("Blues"),
            Self::ORANGES => String::from("Oranges"),
            Self::REDS => String::from("Reds"),
            Self::GREENS => String::from("Greens"),
            Self::GREYS => String::from("Greys"),
            Self::STATUS_ERRORS => String::from("status_errors"),
            Self::STATUS_WARNINGS => String::from("status_warnings"),
            Self::STATUS_SUCCESSES => String::from("status_successes"),
            Self::PLASMA => String::from("Plasma"),
            Self::VIRIDIS => String::from("Viridis"),
            Self::DOGCAT => String::from("dogcat"),
            Self::DATADOG16 => String::from("datadog16"),
            Self::PA_CLASSIC_LINE => String::from("pa_classic_line"),
            Self::PA_CLASSIC_AREA => String::from("pa_classic_area"),
            Self::PA_CLASSIC_BARS => String::from("pa_classic_bars"),
            Self::PA_DATADOG16 => String::from("pa_datadog16"),
            Self::CLASSIC => String::from("classic"),
            Self::COOL_HM => String::from("Cool_hm"),
            Self::WARM_HM => String::from("Warm_hm"),
            Self::YELLOW___GREEN => String::from("Yellow / Green"),
            Self::GREEN___ORANGE => String::from("Green / Orange"),
            Self::GREEN___RED => String::from("Green / Red"),
            Self::GREEN___ORANGE___RED => String::from("Green / Orange / Red"),
            Self::BLUE___RED => String::from("Blue / Red"),
            Self::GOOD___BAD => String::from("Good - bad"),
            Self::COOL => String::from("cool"),
            Self::OCEAN => String::from("ocean"),
            Self::WARM => String::from("warm"),
            Self::FIRE => String::from("fire"),
            Self::WARN => String::from("warn"),
            Self::PURPLE => String::from("purple"),
            Self::GREY => String::from("grey"),
            Self::BLUE => String::from("blue"),
            Self::DD20 => String::from("dd20"),
            Self::SEMANTIC => String::from("semantic"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SunburstRequestStylePalette {
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

impl<'de> Deserialize<'de> for SunburstRequestStylePalette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"success" => Self::SUCCESS,"red" => Self::RED,"green" => Self::GREEN,"orange" => Self::ORANGE,"gray" => Self::GRAY,"dog_classic" => Self::DOG_CLASSIC,"dog_classic_area" => Self::DOG_CLASSIC_AREA,"dog_classic_line" => Self::DOG_CLASSIC_LINE,"dog_classic_bars" => Self::DOG_CLASSIC_BARS,"dog_highlight" => Self::DOG_HIGHLIGHT,"monitor_state" => Self::MONITOR_STATE,"yellow_to_green" => Self::YELLOW_TO_GREEN,"green_to_orange" => Self::GREEN_TO_ORANGE,"hostmap_blues" => Self::HOSTMAP_BLUES,"hostmap_green_red" => Self::HOSTMAP_GREEN_RED,"hostmap_green_orange_red" => Self::HOSTMAP_GREEN_ORANGE_RED,"hostmap_blue_red" => Self::HOSTMAP_BLUE_RED,"dark_grey" => Self::DARK_GREY,"pride" => Self::PRIDE,"GnBu" => Self::GNBU,"YlOrRd" => Self::YLORRD,"Purples" => Self::PURPLES,"Blues" => Self::BLUES,"Oranges" => Self::ORANGES,"Reds" => Self::REDS,"Greens" => Self::GREENS,"Greys" => Self::GREYS,"status_errors" => Self::STATUS_ERRORS,"status_warnings" => Self::STATUS_WARNINGS,"status_successes" => Self::STATUS_SUCCESSES,"Plasma" => Self::PLASMA,"Viridis" => Self::VIRIDIS,"dogcat" => Self::DOGCAT,"datadog16" => Self::DATADOG16,"pa_classic_line" => Self::PA_CLASSIC_LINE,"pa_classic_area" => Self::PA_CLASSIC_AREA,"pa_classic_bars" => Self::PA_CLASSIC_BARS,"pa_datadog16" => Self::PA_DATADOG16,"classic" => Self::CLASSIC,"Cool_hm" => Self::COOL_HM,"Warm_hm" => Self::WARM_HM,"Yellow / Green" => Self::YELLOW___GREEN,"Green / Orange" => Self::GREEN___ORANGE,"Green / Red" => Self::GREEN___RED,"Green / Orange / Red" => Self::GREEN___ORANGE___RED,"Blue / Red" => Self::BLUE___RED,"Good - bad" => Self::GOOD___BAD,"cool" => Self::COOL,"ocean" => Self::OCEAN,"warm" => Self::WARM,"fire" => Self::FIRE,"warn" => Self::WARN,"purple" => Self::PURPLE,"grey" => Self::GREY,"blue" => Self::BLUE,"dd20" => Self::DD20,"semantic" => Self::SEMANTIC,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
