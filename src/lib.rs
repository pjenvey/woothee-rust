#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![recursion_limit="100"]
//! # Woothee
//!
//! Woothee is a user-agent strings parser.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! woothee = "*"
//! ```
//!
//! ```rust
//! extern crate woothee;
//!
//! use woothee::parser::Parser;
//!
//! fn main() {
//!     let parser = Parser::new();
//!     let result = parser.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
//!     println!("{:?}", result);
//! }
//! ```
//!

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod woothee;
pub mod parser;
pub mod dataset;

use parser::{Parser, WootheeResult};

pub fn is_crawler(agent: &str) -> bool {
    if agent.is_empty() || agent == "-" {
        return false;
    }

    let parser = Parser::new();
    let mut result = WootheeResult::new();
    parser.try_crawler(agent, &mut result)
}

#[cfg(test)]
mod tests {
    use super::{is_crawler, WootheeResult, Parser};

    fn get_woothee_result(agent: &str) -> WootheeResult {
        Parser::new().parse(agent).expect("fail parse()")
    }

    #[test]
    fn test_parser_lifetime() {
        let agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.7; rv:21.0) Gecko/20100101 Firefox/21.0";
        let result = get_woothee_result(agent);
        assert_eq!(result.name.as_str(), "Firefox");
    }

    #[test]
    fn test_is_crawler_smoke() {
        assert!(!is_crawler("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"));
        assert!(is_crawler("Mozilla/5.0 (compatible; Yahoo! Slurp; \
                            http://help.yahoo.com/help/us/ysearch/slurp)"));
    }
}
