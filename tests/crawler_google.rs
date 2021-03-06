// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_crawler_google() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Googlebot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Googlebot-Image/1.0"#) {
            None => panic!(r#"invalid parse. "Googlebot-Image/1.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Googlebot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 N905i(c100;TB;W24H16) (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Googlebot Mobile".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"SAMSUNG-SGH-E250/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0 (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "SAMSUNG-SGH-E250/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0 (compatible; Googlebot-Mobile/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Googlebot Mobile".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"DoCoMo/2.0 SH905i(c100;TB;W24H16) (compatible; Mediapartners-Google/2.1; +http://www.google.com/bot.html)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 SH905i(c100;TB;W24H16) (compatible; Mediapartners-Google/2.1; +http://www.google.com/bot.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google Mediapartners".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mediapartners-Google"#) {
            None => panic!(r#"invalid parse. "Mediapartners-Google""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google Mediapartners".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Feedfetcher-Google; (+http://www.google.com/feedfetcher.html; feed-id=000000000000000000)"#) {
            None => panic!(r#"invalid parse. "Feedfetcher-Google; (+http://www.google.com/feedfetcher.html; feed-id=000000000000000000)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google Feedfetcher".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"AppEngine-Google"#) {
            None => panic!(r#"invalid parse. "AppEngine-Google""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google AppEngine".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (en-us) AppleWebKit/525.13 (KHTML, like Gecko; Google Web Preview) Version/3.1 Safari/525.13"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (en-us) AppleWebKit/525.13 (KHTML, like Gecko; Google Web Preview) Version/3.1 Safari/525.13""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google Web Preview".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"FeedBurner/1.0 (http://www.FeedBurner.com)"#) {
            None => panic!(r#"invalid parse. "FeedBurner/1.0 (http://www.FeedBurner.com)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Google FeedBurner".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
    }
}
