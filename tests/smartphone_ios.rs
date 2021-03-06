// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_smartphone_ios() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (iPhone; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "5.0.1".to_string());
                assert_eq!(result.version, "5.1".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B411 Safari/600.1.4"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B411 Safari/600.1.4""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "8.1".to_string());
                assert_eq!(result.version, "8.0".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Mobile/12B411"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Mobile/12B411""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Webview".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "8.1".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPad; U; CPU OS 4_3_2 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8H7 Safari/6533.18.5"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPad; U; CPU OS 4_3_2 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8H7 Safari/6533.18.5""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iPad".to_string());
                assert_eq!(result.os_version, "4.3.2".to_string());
                assert_eq!(result.version, "5.0.2".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPad; U; CPU OS 4_3_5 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8L1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPad; U; CPU OS 4_3_5 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8L1""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Webview".to_string());
                assert_eq!(result.os, "iPad".to_string());
                assert_eq!(result.os_version, "4.3.5".to_string());
                assert_eq!(result.version, "5.0.2".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPod; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPod; CPU iPhone OS 5_0_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A405 Safari/7534.48.3""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iPod".to_string());
                assert_eq!(result.os_version, "5.0.1".to_string());
                assert_eq!(result.version, "5.1".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPod touch; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPod touch; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iPod".to_string());
                assert_eq!(result.os_version, "7.0".to_string());
                assert_eq!(result.version, "7.0".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPod touch; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Mobile/12B466"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPod touch; CPU iPhone OS 8_1_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Mobile/12B466""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Webview".to_string());
                assert_eq!(result.os, "iPod".to_string());
                assert_eq!(result.os_version, "8.1.3".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPhone; U; CPU iPhone OS 5_1_1 like Mac OS X; ja-jp) AppleWebKit/534.46.0 (KHTML, like Gecko) CriOS/19.0.1084.60 Mobile/9B206 Safari/7534.48.3"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; U; CPU iPhone OS 5_1_1 like Mac OS X; ja-jp) AppleWebKit/534.46.0 (KHTML, like Gecko) CriOS/19.0.1084.60 Mobile/9B206 Safari/7534.48.3""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Chrome".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "5.1.1".to_string());
                assert_eq!(result.version, "19.0.1084.60".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPhone; CPU iPhone OS 8_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) FxiOS/1.0 Mobile/12F69 Safari/600.1.4"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; CPU iPhone OS 8_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) FxiOS/1.0 Mobile/12F69 Safari/600.1.4""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Firefox".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "8.3".to_string());
                assert_eq!(result.version, "1.0".to_string());
            }
        }
        match parser.parse(r#"Girls/2.0 (livedoor Co.,Ltd.; Peachy 2.1; iPhone; RSS Version 2.0; +http://girls.livedoor.com/)"#) {
            None => panic!(r#"invalid parse. "Girls/2.0 (livedoor Co.,Ltd.; Peachy 2.1; iPhone; RSS Version 2.0; +http://girls.livedoor.com/)""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "iPhone".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"MobileSafari/7534.48.3 CFNetwork/548.0.4 Darwin/11.0.0"#) {
            None => panic!(r#"invalid parse. "MobileSafari/7534.48.3 CFNetwork/548.0.4 Darwin/11.0.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "Safari".to_string());
                assert_eq!(result.os, "iOS".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Blogos/1.13 CFNetwork/548.0.4 Darwin/11.0.0"#) {
            None => panic!(r#"invalid parse. "Blogos/1.13 CFNetwork/548.0.4 Darwin/11.0.0""#),
            Some(result) => {
                assert_eq!(result.category, "smartphone".to_string());
                assert_eq!(result.name, "UNKNOWN".to_string());
                assert_eq!(result.os, "iOS".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
    }
}
