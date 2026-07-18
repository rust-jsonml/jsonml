#![doc = include_str!("../README.md")]

use core::convert::From;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
};

use void::Void;

use html_escape::encode_unquoted_attribute;

#[cfg(test)]
use serde_test::{Token, assert_de_tokens, assert_tokens};

// `Eq` and `Hash` cannot be derived since neither can `AttributeValue`.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Element {
    Tag {
        name: String,
        attributes: HashMap<String, AttributeValue>,
        element_list: Vec<Element>,
    },
    String(String),
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
            Element::Tag {
                name,
                attributes,
                element_list,
            } => {
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
            Ok(Element::Tag {
                name,
                attributes,
                element_list,
            })
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
                Element::Tag {
                    name,
                    attributes,
                    element_list,
                } => {
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
                                    && !('\u{FDD0}'..='\u{FDEF}').contains(&c)
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
        Element::Tag {
            name: "div".to_string(),
            attributes: HashMap::from([(
                "id".to_string(),
                AttributeValue::String("aaa".to_string())
            ),]),
            element_list: vec![
                Element::String("bbb".to_string()),
                Element::Tag {
                    name: "span".to_string(),
                    attributes: HashMap::default(),
                    element_list: vec![Element::String("ccc".to_string())]
                }
            ]
        }
        .to_string(),
        r#"<div id="aaa">bbb<span>ccc</span></div>"#.to_string()
    );
}

#[test]
#[should_panic]
fn test_display_element_invalid_tag_name() {
    Element::Tag {
        name: "あ".to_string(),
        attributes: HashMap::default(),
        element_list: vec![],
    }
    .to_string();
}

#[test]
#[should_panic]
fn test_display_element_invalid_attribute_name() {
    Element::Tag {
        name: "a".to_string(),
        attributes: HashMap::from([(" ".to_string(), AttributeValue::Null)]),
        element_list: vec![],
    }
    .to_string();
}

#[test]
fn test_display_element_encode_attribute_value() {
    assert_eq!(
        Element::Tag {
            name: "a".to_string(),
            attributes: HashMap::from(
                [("b".to_string(), AttributeValue::String("=".to_string())),]
            ),
            element_list: vec![]
        }
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
    let element = Element::Tag {
        name: "li".to_string(),
        attributes: HashMap::from([(
            "style".to_string(),
            AttributeValue::String("color:red".to_string()),
        )]),
        element_list: vec![Element::String("First Item".to_string())],
    };
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
    let element = Element::Tag {
        name: "li".to_string(),
        attributes: HashMap::default(),
        element_list: vec![],
    };
    assert_tokens(
        &element,
        &[Token::Seq { len: Some(1) }, Token::Str("li"), Token::SeqEnd],
    );
}

#[test]
fn test_element_tag_without_element_list() {
    let element = Element::Tag {
        name: "li".to_string(),
        attributes: HashMap::from([(
            "style".to_string(),
            AttributeValue::String("color:red".to_string()),
        )]),
        element_list: vec![],
    };
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

pub struct ElementPreIter<'a> {
    stack: Vec<&'a Element>,
}

impl<'a> Iterator for ElementPreIter<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Element::Tag { element_list, .. } = node {
            for child in element_list.iter().rev() {
                self.stack.push(child);
            }
        }
        Some(node)
    }
}

pub struct PreOrderElement(Element);

impl From<Element> for PreOrderElement {
    fn from(value: Element) -> PreOrderElement {
        PreOrderElement(value)
    }
}

impl<'a> IntoIterator for &'a PreOrderElement {
    type Item = &'a Element;
    type IntoIter = ElementPreIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let PreOrderElement(element) = self;
        ElementPreIter {
            stack: vec![element],
        }
    }
}

pub struct ElementPostIter<'a> {
    stack: Vec<&'a Element>,
    result: Vec<&'a Element>,
}

impl<'a> Iterator for ElementPostIter<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            self.result.push(node);
            if let Element::Tag { element_list, .. } = node {
                for child in element_list.iter() {
                    self.stack.push(child);
                }
            }
        }
        self.result.pop()
    }
}

pub struct PostOrderElement(Element);

impl From<Element> for PostOrderElement {
    fn from(value: Element) -> PostOrderElement {
        PostOrderElement(value)
    }
}

impl<'a> IntoIterator for &'a PostOrderElement {
    type Item = &'a Element;
    type IntoIter = ElementPostIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let PostOrderElement(element) = self;
        ElementPostIter {
            stack: vec![element],
            result: vec![],
        }
    }
}

#[test]
fn test_iters() {
    let text = Element::String("hello".to_string());
    let span = Element::Tag {
        name: "span".to_string(),
        attributes: HashMap::new(),
        element_list: vec![],
    };
    let tree = Element::Tag {
        name: "div".to_string(),
        attributes: HashMap::new(),
        element_list: vec![text.clone(), span.clone()],
    };

    let nodes: Vec<_> = PreOrderElement::from(tree.clone())
        .into_iter()
        .map(|node| node.to_owned())
        .collect();
    assert_eq!(nodes, vec![tree.clone(), text.clone(), span.clone()]);

    let nodes: Vec<_> = PostOrderElement::from(tree.clone())
        .into_iter()
        .map(|node| node.to_owned())
        .collect();
    assert_eq!(nodes, vec![text, span, tree]);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<AttributeValue>),
    Object(HashMap<String, AttributeValue>),

    #[default]
    Null,
}

impl AttributeValue {
    /// Renders the value as JSON. Object keys are emitted in sorted order so
    /// that the output does not vary with hash iteration order.
    fn write_json(&self, out: &mut String) {
        match self {
            AttributeValue::String(s) => write_json_string(s, out),
            AttributeValue::Number(n) => out.push_str(&n.to_string()),
            AttributeValue::Bool(b) => out.push_str(&b.to_string()),
            AttributeValue::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write_json(out);
                }
                out.push(']');
            }
            AttributeValue::Object(members) => {
                let mut keys: Vec<&String> = members.keys().collect();
                keys.sort();
                out.push('{');
                for (index, key) in keys.into_iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_json_string(key, out);
                    out.push(':');
                    members[key].write_json(out);
                }
                out.push('}');
            }
            AttributeValue::Null => out.push_str("null"),
        }
    }
}

fn write_json_string(value: &str, out: &mut String) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Display in HTML
impl Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::String(s) => write!(f, "{}", encode_unquoted_attribute(s)),
            AttributeValue::Number(n) => write!(f, "{}", n),
            AttributeValue::Bool(b) => write!(f, "{}", b),
            AttributeValue::Array(_) | AttributeValue::Object(_) => {
                let mut json = String::new();
                self.write_json(&mut json);
                write!(f, "{}", encode_unquoted_attribute(&json))
            }
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
fn test_attribute_value_number() {
    let value = AttributeValue::Number(3.14);
    assert_tokens(&value, &[Token::F64(3.14)]);
}

#[test]
fn test_attribute_value_number_keeps_double_precision() {
    let value: AttributeValue = serde_json::from_str("16777217").unwrap();
    assert_eq!(value, AttributeValue::Number(16777217.0));
    assert_eq!(serde_json::to_string(&value).unwrap(), "16777217.0");
}

#[test]
fn test_attribute_value_array() {
    let value: AttributeValue = serde_json::from_str(r#"["Length:1","Min:0"]"#).unwrap();
    assert_eq!(
        value,
        AttributeValue::Array(vec![
            AttributeValue::String("Length:1".to_string()),
            AttributeValue::String("Min:0".to_string()),
        ])
    );
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"["Length:1","Min:0"]"#
    );
}

#[test]
fn test_attribute_value_object() {
    let value: AttributeValue = serde_json::from_str(r#"{"fg":"Green","bold":true}"#).unwrap();
    let AttributeValue::Object(ref members) = value else {
        panic!("expected an object");
    };
    assert_eq!(
        members.get("fg"),
        Some(&AttributeValue::String("Green".to_string()))
    );
    assert_eq!(members.get("bold"), Some(&AttributeValue::Bool(true)));
}

#[test]
fn test_attribute_value_nested_round_trip() {
    let source = r#"{"a":[1.0,{"b":null}],"c":{"d":[true]}}"#;
    let value: AttributeValue = serde_json::from_str(source).unwrap();
    let round_tripped = serde_json::to_string(&value).unwrap();
    let reparsed: AttributeValue = serde_json::from_str(&round_tripped).unwrap();
    assert_eq!(value, reparsed);
}

#[test]
fn test_element_with_structured_attribute_values() {
    let source = r#"["Layout",{"constraints":["Length:1","Min:0"]},["Gauge",{"style":{"fg":"Green"}}]]"#;
    let element: Element = serde_json::from_str(source).unwrap();
    let Element::Tag {
        ref name,
        ref attributes,
        ref element_list,
    } = element
    else {
        panic!("expected a tag");
    };
    assert_eq!(name, "Layout");
    assert_eq!(
        attributes.get("constraints"),
        Some(&AttributeValue::Array(vec![
            AttributeValue::String("Length:1".to_string()),
            AttributeValue::String("Min:0".to_string()),
        ]))
    );
    assert_eq!(element_list.len(), 1);

    let reparsed: Element = serde_json::from_str(&serde_json::to_string(&element).unwrap()).unwrap();
    assert_eq!(element, reparsed);
}

#[test]
fn test_display_structured_attribute_value_as_json() {
    let value = AttributeValue::Array(vec![
        AttributeValue::Number(1.0),
        AttributeValue::String("two".to_string()),
    ]);
    let rendered = value.to_string();
    assert_eq!(
        html_escape::decode_html_entities(&rendered),
        r#"[1,"two"]"#
    );
}

#[test]
fn test_display_object_attribute_value_sorts_keys() {
    let value: AttributeValue = serde_json::from_str(r#"{"b":1,"a":2,"c":3}"#).unwrap();
    assert_eq!(
        html_escape::decode_html_entities(&value.to_string()),
        r#"{"a":2,"b":1,"c":3}"#
    );
}

#[test]
fn test_display_object_attribute_value_is_order_independent() {
    let one: AttributeValue = serde_json::from_str(r#"{"b":1,"a":2}"#).unwrap();
    let other: AttributeValue = serde_json::from_str(r#"{"a":2,"b":1}"#).unwrap();
    assert_eq!(one.to_string(), other.to_string());
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
