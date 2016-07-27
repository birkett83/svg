//! The elements.

use std::fmt;

use Node;

mod value;

pub use self::value::Value;

/// An element.
#[derive(Debug)]
pub struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Box<Node>>,
}

impl Element {
    /// Create an element.
    #[inline]
    pub fn new<T: Into<String>>(name: T) -> Self {
        Element { name: name.into(), attributes: vec![], children: vec![] }
    }

    /// Append a node.
    #[inline]
    pub fn append<T: Node>(&mut self, node: T) {
        self.children.push(Box::new(node));
    }

    /// Assign an attribute.
    #[inline]
    pub fn assign<T: Into<String>, U: Value>(&mut self, name: T, value: U) {
        self.attributes.push((name.into(), value.into()));
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "<{}", self.name));
        for &(ref name, ref value) in self.attributes.iter() {
            try!(write!(formatter, " {}='{}'", name, value));
        }
        if self.children.is_empty() {
            write!(formatter, "/>")
        } else {
            try!(write!(formatter, ">"));
            for child in self.children.iter() {
                try!(write!(formatter, "\n{}", child));
            }
            write!(formatter, "\n</{}>", self.name)
        }
    }
}

impl Node for Element {
}

macro_rules! element {
    ($(#[$attribute:meta])* struct $struct_name:ident($name:expr)) => (
        $(#[$attribute])*
        #[derive(Debug)]
        pub struct $struct_name(pub ::element::Element);

        impl $struct_name {
            /// Create an element.
            #[inline]
            pub fn new() -> Self {
                $struct_name(::element::Element::new($name))
            }

            /// Append a node.
            pub fn add<T: ::Node>(mut self, node: T) -> Self {
                self.0.append(node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T: Into<String>, U: ::element::Value>(mut self, name: T, value: U) -> Self {
                self.0.assign(name, value);
                self
            }
        }

        impl ::std::ops::Deref for $struct_name {
            type Target = ::element::Element;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl ::Node for $struct_name {
        }
    );
}

pub mod path;

pub use self::path::Path;

element! {
    #[doc = "
    The [svg][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement"]
    struct SVG("svg")
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn display() {
        let mut element = Element::new("foo");
        element.assign("x", -15);
        element.assign("y", "10px");
        element.assign("size", (42.5, 69.0));
        element.assign("color", "green");
        element.append(Element::new("bar"));
        assert_eq!(element.to_string(), "\
            <foo x='-15' y='10px' size='42.5 69' color='green'>\n\
            <bar/>\n\
            </foo>\
        ");
    }
}
