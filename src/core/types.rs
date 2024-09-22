use std::{time::Duration, u64};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Task {
    actions: Vec<Action>,
    dalay: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    randomize_delay: Option<Randomization>,
    repeat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    randomize_repeat: Option<Randomization>,
    termination_conditions: Vec<TerminationConditions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    Click(ClickAction),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct ClickAction {
    pub button: ButtonType,
    pub position: ClickPosition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_position: Option<Randomization>,
    pub delay: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_delay: Option<Randomization>,
    pub repeat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_repeat: Option<Randomization>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClickPosition {
    #[default]
    Unspecified,
    Fixed {
        x: i64,
        y: i64,
    },
    Image {
        image: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum ButtonType {
    #[default]
    Left,
    Right,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Randomization {
    pub range: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TerminationConditions {
    Timeout {
        #[serde(with = "super::dur_serde")]
        timeout: Duration,
    },
    Image {
        image: String,
    },
}
