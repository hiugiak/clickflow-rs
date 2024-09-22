use std::time::Duration;

use serde_test::{assert_tokens, Token};

use super::ClickAction;

#[test]
fn test_term_conds_ser_de() {
    let timeout = super::TerminationConditions::Timeout {
        timeout: Duration::from_millis(1000),
    };
    assert_tokens(
        &timeout,
        &[
            Token::Struct {
                name: "TerminationConditions",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("timeout"),
            Token::Str("timeout"),
            Token::U64(1000),
            Token::StructEnd,
        ],
    );

    let image = super::TerminationConditions::Image {
        image: String::from("./image.png"),
    };
    assert_tokens(
        &image,
        &[
            Token::Struct {
                name: "TerminationConditions",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("image"),
            Token::Str("image"),
            Token::Str("./image.png"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_click_action_ser_de() {
    let action = super::Action::Click(ClickAction {
        button: super::ButtonType::Left,
        position: super::ClickPosition::Fixed { x: 100, y: 100 },
        repeat: 2,
        randomize_repeat: Some(super::Randomization { range: 4 }),
        ..Default::default()
    });
    assert_tokens(
        &action,
        &[
            Token::Struct {
                name: "ClickAction",
                len: 6,
            },
            Token::Str("type"),
            Token::Str("click"),
            Token::Str("button"),
            Token::Enum { name: "ButtonType" },
            Token::Str("left"),
            Token::Unit,
            Token::Str("position"),
            Token::Struct {
                name: "ClickPosition",
                len: 3,
            },
            Token::Str("type"),
            Token::Str("fixed"),
            Token::Str("x"),
            Token::I64(100),
            Token::Str("y"),
            Token::I64(100),
            Token::StructEnd,
            Token::Str("delay"),
            Token::I64(0),
            Token::Str("repeat"),
            Token::I64(2),
            Token::Str("randomizeRepeat"),
            Token::Some,
            Token::Struct {
                name: "Randomization",
                len: 1,
            },
            Token::Str("range"),
            Token::I64(4),
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}
