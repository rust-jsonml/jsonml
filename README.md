# JsonML crate

[JsonML](http://www.jsonml.org/) deserialization and serialization.

* [GitHub](https://github.com/rust-jsonml/jsonml)
  (former repository was at [GitLab](https://gitlab.com/gemmaro/rust-jsonml))
* [documentation](https://docs.rs/jsonml/latest/jsonml/)

Deserialization example:

```rust
# use std::collections::HashMap;
use jsonml::{Element, AttributeValue};

let element: Element =
    serde_json::from_str(r#"[ "li", { "style": "color:red" }, "First Item" ]"#)
        .expect("deserialize element tag");
assert_eq!(
    element,
    Element::Tag {
        name: "li".to_string(),
        attributes: HashMap::from([(
            "style".to_string(),
            AttributeValue::String("color:red".to_string())
        )]),
        element_list: vec![Element::String("First Item".to_string())]
    }
);
```

Serialization example:

```rust
# use std::collections::HashMap;
use jsonml::{Element, AttributeValue};

let element = Element::Tag {
    name: "li".to_string(),
    attributes: HashMap::from([(
        "style".to_string(),
        AttributeValue::String("color:red".to_string()))]
    ),
    element_list: vec![Element::String("First Item".to_string())]
};
assert_eq!(
    serde_json::to_string(&element).expect("serialize element tag"),
    r#"["li",{"style":"color:red"},"First Item"]"#
);
```

## License

Licensed under either of the following at your option:

* [`LICENSE-APACHE.txt`](LICENSE-APACHE.txt) or [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [`LICENSE-MIT.txt`](LICENSE-MIT.txt) or [The MIT License](https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
