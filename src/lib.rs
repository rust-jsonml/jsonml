#![doc = include_str!("../README.md")]

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

use void::Void;

use html_escape::encode_unquoted_attribute;

#[cfg(test)]
use serde_test::{assert_de_tokens, assert_tokens, Token};

// `Eq` and `Hash` cannot be derived since neither can `AttributeValue`.
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Tag(Tag),
    String(String),
}

/// WARNING: The fields will be hidden in the future for validation.
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub element_list: Vec<Element>,
}

impl Default for Element {
    fn default() -> Self {
        Element::String("".to_string())
    }
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Element::Tag(Tag {
                name,
                attributes,
                element_list,
            }) => {
                let mut seq =
                    serializer.serialize_seq(Some(1 + attributes.len() + element_list.len()))?;
                seq.serialize_element(&name)?;
                if !attributes.is_empty() {
                    seq.serialize_element(attributes)?;
                }
                if !element_list.is_empty() {
                    for element in element_list {
                        seq.serialize_element(&element)?;
                    }
                }
                seq.end()
            }
            Element::String(string) => serializer.serialize_str(string),
        }
    }
}

struct ElementVisitor;

impl<'de> Visitor<'de> for ElementVisitor {
    type Value = Element;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("JsonML element, which is tag or string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Element::String(v.to_string()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        if let Some(name) = seq.next_element::<String>()? {
            let mut attributes = HashMap::default();
            let mut element_list = vec![];

            #[derive(Deserialize)]
            #[serde(untagged)]
            enum AttributesOrElement {
                Attributes(HashMap<String, AttributeValue>),
                Element(Element),
            }

            if let Some(attributes_or_element) = seq.next_element::<AttributesOrElement>()? {
                match attributes_or_element {
                    AttributesOrElement::Attributes(attrs) => attributes = attrs,
                    AttributesOrElement::Element(element) => element_list.push(element),
                }
            }
            while let Some(element) = seq.next_element::<Element>()? {
                element_list.push(element);
            }
            Ok(Element::Tag(Tag {
                name,
                attributes,
                element_list,
            }))
        } else {
            Err(de::Error::missing_field("name"))
        }
    }
}

impl FromStr for Element {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Element::String(s.to_string()))
    }
}

/// Display in HTML
///
/// Panics when
///
/// * the element is a tag and the tag name contains an invalid character
/// * at least one attribute name contains an invalid character
impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Tag(Tag {
                    name,
                    attributes,
                    element_list,
                }) => {
                    // HTML tag name must consists of ASCII alphanumerics.
                    // https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
                    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
                        return Err(fmt::Error);
                    }

                    let attrs = if attributes.is_empty() {
                        "".to_string()
                    } else {
                        let mut attrs = String::default();
                        for (key, value) in attributes {
                            // Validate attribute name
                            // https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
                            if !key.chars().all(|c| {
                                c != '\u{0020}'
                                    && c != '\u{0022}'
                                    && c != '\u{0027}'
                                    && c != '\u{003E}'
                                    && c != '\u{002F}'
                                    && c != '\u{003D}'
                                    && !('\u{FDD0}' <= c && c <= '\u{FDEF}')
                                    && c != '\u{FFFE}'
                                    && c != '\u{FFFF}'
                                    && c != '\u{1FFFE}'
                                    && c != '\u{1FFFF}'
                                    && c != '\u{2FFFE}'
                                    && c != '\u{2FFFF}'
                                    && c != '\u{3FFFE}'
                                    && c != '\u{3FFFF}'
                                    && c != '\u{4FFFE}'
                                    && c != '\u{4FFFF}'
                                    && c != '\u{5FFFE}'
                                    && c != '\u{5FFFF}'
                                    && c != '\u{6FFFE}'
                                    && c != '\u{6FFFF}'
                                    && c != '\u{7FFFE}'
                                    && c != '\u{7FFFF}'
                                    && c != '\u{8FFFE}'
                                    && c != '\u{8FFFF}'
                                    && c != '\u{9FFFE}'
                                    && c != '\u{9FFFF}'
                                    && c != '\u{AFFFE}'
                                    && c != '\u{AFFFF}'
                                    && c != '\u{BFFFE}'
                                    && c != '\u{BFFFF}'
                                    && c != '\u{CFFFE}'
                                    && c != '\u{CFFFF}'
                                    && c != '\u{DFFFE}'
                                    && c != '\u{DFFFF}'
                                    && c != '\u{EFFFE}'
                                    && c != '\u{EFFFF}'
                                    && c != '\u{FFFFE}'
                                    && c != '\u{FFFFF}'
                                    && c != '\u{10FFFE}'
                                    && c != '\u{10FFFF}'
                            }) {
                                return Err(fmt::Error);
                            }
                            let pair = format!(r#" {key}="{value}""#);
                            attrs += &pair;
                        }
                        attrs
                    };
                    let elms = element_list
                        .iter()
                        .map(|elm| elm.to_string())
                        .collect::<Vec<_>>()
                        .join("");
                    format!("<{name}{attrs}>{elms}</{name}>")
                }
                Element::String(s) => s.to_owned(),
            }
        )
    }
}

#[test]
fn test_display_element() {
    assert_eq!(
        Element::Tag(Tag {
            name: "div".to_string(),
            attributes: HashMap::from([(
                "id".to_string(),
                AttributeValue::String("aaa".to_string())
            ),]),
            element_list: vec![
                Element::String("bbb".to_string()),
                Element::Tag(Tag {
                    name: "span".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![Element::String("ccc".to_string())]
                })
            ]
        })
        .to_string(),
        r#"<div id="aaa">bbb<span>ccc</span></div>"#.to_string()
    );
}

#[test]
#[should_panic]
fn test_display_element_invalid_tag_name() {
    Element::Tag(Tag {
        name: "あ".to_string(),
        attributes: HashMap::default(),
        element_list: vec![],
    })
    .to_string();
}

#[test]
#[should_panic]
fn test_display_element_invalid_attribute_name() {
    Element::Tag(Tag {
        name: "a".to_string(),
        attributes: HashMap::from([(" ".to_string(), AttributeValue::Null)]),
        element_list: vec![],
    })
    .to_string();
}

#[test]
fn test_display_element_encode_attribute_value() {
    assert_eq!(
        Element::Tag(Tag {
            name: "a".to_string(),
            attributes: HashMap::from(
                [("b".to_string(), AttributeValue::String("=".to_string())),]
            ),
            element_list: vec![]
        })
        .to_string(),
        r#"<a b="&#x3D;"></a>"#.to_string()
    );
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Element, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ElementVisitor)
    }
}

#[test]
fn test_element_tag() {
    let element = Element::Tag(Tag {
        name: "li".to_string(),
        attributes: HashMap::from([(
            "style".to_string(),
            AttributeValue::String("color:red".to_string()),
        )]),
        element_list: vec![Element::String("First Item".to_string())],
    });
    assert_tokens(
        &element,
        &[
            Token::Seq { len: Some(3) },
            Token::Str("li"),
            Token::Map { len: Some(1) },
            Token::Str("style"),
            Token::Str("color:red"),
            Token::MapEnd,
            Token::Str("First Item"),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_element_tag_name_only() {
    let element = Element::Tag(Tag {
        name: "li".to_string(),
        attributes: HashMap::default(),
        element_list: vec![],
    });
    assert_tokens(
        &element,
        &[Token::Seq { len: Some(1) }, Token::Str("li"), Token::SeqEnd],
    );
}

#[test]
fn test_element_tag_without_element_list() {
    let element = Element::Tag(Tag {
        name: "li".to_string(),
        attributes: HashMap::from([(
            "style".to_string(),
            AttributeValue::String("color:red".to_string()),
        )]),
        element_list: vec![],
    });
    assert_tokens(
        &element,
        &[
            Token::Seq { len: Some(2) },
            Token::Str("li"),
            Token::Map { len: Some(1) },
            Token::Str("style"),
            Token::Str("color:red"),
            Token::MapEnd,
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_element_string() {
    let element = Element::String("First Item".to_string());
    assert_de_tokens(&element, &[Token::Str("First Item")]);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Number(f32),
    Bool(bool),

    #[default]
    Null,
}

/// Display in HTML
impl Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::String(s) => write!(f, "{}", encode_unquoted_attribute(s)),
            AttributeValue::Number(n) => write!(f, "{}", n.to_string()),
            AttributeValue::Bool(b) => write!(f, "{}", b.to_string()),
            AttributeValue::Null => write!(f, "null"),
        }
    }
}

#[test]
fn test_attribute_value_string() {
    let value = AttributeValue::String("color:red".to_string());
    assert_tokens(&value, &[Token::Str("color:red")]);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_attribute_value_number() {
    let value = AttributeValue::Number(3.14);
    assert_tokens(&value, &[Token::F32(3.14)]);
}

#[test]
fn test_attribute_value_bool() {
    let value = AttributeValue::Bool(false);
    assert_tokens(&value, &[Token::Bool(false)]);
}

#[test]
fn test_attribute_value_null() {
    let value = AttributeValue::Null;
    assert_tokens(&value, &[Token::Unit]);
}
