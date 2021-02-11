/*!
Common `Error`, `ErrorKind`, and `Result` types.
*/

#![allow(missing_docs)]

error_chain! {
    errors {
        #[doc = "An empty string is not allowed here."]
        MustNotBeEmpty {
            description("An empty string is not allowed here.")
            display("An empty string is not allowed here.")
        }
        #[doc = "An illegal character was found parsing a content string."]
        IllegalCharacter {
            description("An illegal character was found parsing a content string.")
            display("An illegal character was found parsing a content string.")
        }
        #[doc = "The item you are adding would conflict with an existing item."]
        Conflict {
            description("The item you are adding would conflict with an existing item.")
            display("The item you are adding would conflict with an existing item.")
        }
        #[doc = "The inner content you are adding would conflict with an existing content item."]
        UnexpectedBlock {
            description("The inner content you are adding would conflict with an existing content item.")
            display("The inner content you are adding would conflict with an existing content item.")
        }
        #[doc = "The inner content you are adding would conflict with an existing content item."]
        UnexpectedInline {
            description("The inner content you are adding would conflict with an existing content item.")
            display("The inner content you are adding would conflict with an existing content item.")
        }
        #[doc = "The provided value is not a known document format."]
        UnknownFormat {
            description("The provided value is not a known document format.")
            display("The provided value is not a known document format.")
        }
    }
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}
