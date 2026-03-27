/* These tests (Bulleted List Example and Colorful Table Example)
 * are cited from [the JsonML site](http://www.jsonml.org/).
 */

use std::collections::HashMap;

use jsonml::{AttributeValue, Element};

#[test]
fn test_bullet_list_example() {
    let element: Element =
        serde_json::from_str(include_str!("fixtures/bullet-list.json")).expect("deserialize JSON");
    assert_eq!(
        element,
        Element::Tag {
            name: "ul".to_string(),
            attributes: HashMap::default(),
            element_list: vec![
                Element::Tag {
                    name: "li".to_string(),
                    attributes: HashMap::from([(
                        "style".to_string(),
                        AttributeValue::String("color:red".to_string())
                    )]),
                    element_list: vec![Element::String("First Item".to_string())]
                },
                Element::Tag {
                    name: "li".to_string(),
                    attributes: HashMap::from([
                        (
                            "title".to_string(),
                            AttributeValue::String("Some hover text.".to_string())
                        ),
                        (
                            "style".to_string(),
                            AttributeValue::String("color:green".to_string())
                        )
                    ]),
                    element_list: vec![Element::String("Second Item".to_string())]
                },
                Element::Tag {
                    name: "li".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![
                        Element::Tag {
                            name: "span".to_string(),
                            attributes: HashMap::from([(
                                "class".to_string(),
                                AttributeValue::String("code-example-third".to_string())
                            )]),
                            element_list: vec![Element::String("Third".to_string())]
                        },
                        Element::String(" Item".to_string())
                    ]
                }
            ]
        }
    );
}

#[test]
fn test_colorful_table_example() {
    let element: Element = serde_json::from_str(include_str!("fixtures/colorful-table.json"))
        .expect("deserialize JSON");
    assert_eq!(
        element,
        Element::Tag {
            name: "table".to_string(),
            attributes: HashMap::from([
                (
                    "class".to_string(),
                    AttributeValue::String("MyTable".to_string())
                ),
                (
                    "style".to_string(),
                    AttributeValue::String("background-color:yellow".to_string())
                )
            ]),
            element_list: vec![
                Element::Tag {
                    name: "tr".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("border:1px solid black".to_string())
                                )
                            ]),
                            element_list: vec![Element::String("#550758".to_string())]
                        },
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("background-color:red".to_string())
                                )
                            ]),
                            element_list: vec![Element::String("Example text here".to_string())]
                        }
                    ]
                },
                Element::Tag {
                    name: "tr".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("border:1px solid black".to_string())
                                )
                            ]),
                            element_list: vec![Element::String("#993101".to_string())]
                        },
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("background-color:green".to_string())
                                )
                            ]),
                            element_list: vec![Element::String("127624015".to_string())]
                        }
                    ]
                },
                Element::Tag {
                    name: "tr".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("border:1px solid black".to_string())
                                )
                            ]),
                            element_list: vec![Element::String("#E33D87".to_string())]
                        },
                        Element::Tag {
                            name: "td".to_string(),
                            attributes: HashMap::from([
                                (
                                    "class".to_string(),
                                    AttributeValue::String("MyTD".to_string())
                                ),
                                (
                                    "style".to_string(),
                                    AttributeValue::String("background-color:blue".to_string())
                                )
                            ]),
                            element_list: vec![
                                Element::String("\u{00A0}".to_string()),
                                Element::Tag {
                                    name: "span".to_string(),
                                    attributes: HashMap::from([(
                                        "style".to_string(),
                                        AttributeValue::String(
                                            "background-color:maroon".to_string()
                                        )
                                    )]),
                                    element_list: vec![Element::String("\u{00A9}".to_string())]
                                },
                                Element::String("\u{00A0}".to_string())
                            ]
                        }
                    ]
                }
            ]
        }
    );
}
