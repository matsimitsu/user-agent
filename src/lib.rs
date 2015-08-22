#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub version: Option<String>
}

impl PartialEq for Browser {
     fn eq(&self, other: &Browser) -> bool {
         self.name == other.name && self.version == other.version
     }
 }


#[derive(Debug)]
pub struct UserAgent {
    pub browser: Option<Browser>,
    pub platform: Option<String>,
    pub language: Option<String>
}

struct BrowserParser {
    name: &'static str,
    re: regex::Regex
}

struct PlatformParser {
    name: &'static str,
    re: regex::Regex
}


static BROWSER_VERSION_RE_ARRAY: [BrowserParser; 19] = [
    BrowserParser {name: "google", re: regex!(r"(?i:(?:googlebot)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "msn", re: regex!(r"(?i:(?:msnbot)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "yahoo", re: regex!(r"(?i:(?:yahoo)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "ask", re: regex!(r"(?i:(?:ask jeeves)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "aol", re: regex!(r"(?i:(?:aol|america\s+online\s+browser)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "opera", re: regex!(r"(?i:(?:opera)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "chrome", re: regex!(r"(?i:(?:chrome)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "firefox", re: regex!(r"(?i:(?:firefox|firebird|phoenix|iceweasel)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "galeon", re: regex!(r"(?i:(?:galeon)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "safari", re: regex!(r"(?i:(?:safari)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "webkit", re: regex!(r"(?i:(?:webkit)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "camino", re: regex!(r"(?i:(?:camino)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "konqueror", re: regex!(r"(?i:(?:konqueror)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "kmeleon", re: regex!(r"(?i:(?:k-meleon)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "netscape", re: regex!(r"(?i:(?:netscape)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "msie", re: regex!(r"(?i:(?:msie|microsoft\s+internet\s+explorer|trident/.+? rv:)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "lynx", re: regex!(r"(?i:(?:lynx)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "links", re: regex!(r"(?i:(?:links)[/\sa-z(]*(\d+[.\da-z]+)?)")},
    BrowserParser {name: "seamonkey", re: regex!(r"(?i:(?:seamonkey|mozilla)[/\sa-z(]*(\d+[.\da-z]+)?)")},
];
static PLATFORM_RE_ARRAY: [PlatformParser; 16] = [
    PlatformParser {name: "chromeos", re: regex!("(?i:cros)")},
    PlatformParser {name: "iphone", re: regex!("(?i:iphone|ios)")},
    PlatformParser {name: "ipad", re: regex!("(?i:ipad)")},
    PlatformParser {name: "macos", re: regex!(r"(?i:darwin|mac|os\s*x)")},
    PlatformParser {name: "windows", re: regex!("(?i:win)")},
    PlatformParser {name: "android", re: regex!(r"(?i:android)")},
    PlatformParser {name: "linux", re: regex!(r"(?i:x11|lin(\b|ux)?)")},
    PlatformParser {name: "solaris", re: regex!("(?i:(sun|i86)os)")},
    PlatformParser {name: "wii", re: regex!(r"(?i:nintendo\s+wii)")},
    PlatformParser {name: "irix", re: regex!("(?i:irix)")},
    PlatformParser {name: "hpux", re: regex!("(?i:hp-?ux)")},
    PlatformParser {name: "aix", re: regex!("(?i:aix)")},
    PlatformParser {name: "sco", re: regex!("(?i:sco|unix_sv)")},
    PlatformParser {name: "bsd", re: regex!("(?i:bsd)")},
    PlatformParser {name: "amiga", re: regex!("(?i:amiga)")},
    PlatformParser {name: "blackberry", re: regex!("(?i:blackberry|playbook)")},
];
static LANGUAGE_RE: regex::Regex = regex!(r"(?:;\s*|\s+)(\b\w{2}\b(?:-\b\w{2}\b)?)\s*;|(?:\(|\[|;)\s*(\b\w{2}\b(?:-\b\w{2}\b)?)\s*(?:\]|\)|;)");

pub fn parse_browser(string: &str) -> Option<Browser> {
    for browser in BROWSER_VERSION_RE_ARRAY.iter() {
        let captures = browser.re.captures(string);
        if captures.is_some() {
            return Some(Browser {
                name: browser.name.to_string(),
                version: match captures.unwrap().at(1) {
                    Some(s) => Some(s.to_string()),
                    None    => None
                }
            });
        }
    }
    return None;
}

pub fn parse_platform(string: &str) -> Option<String> {
    for platform in PLATFORM_RE_ARRAY.iter() {
        if platform.re.is_match(string) {
            return Some(platform.name.to_string());
        }
    }
    return None;
}

pub fn parse_language(string: &str) -> Option<String> {
    return match LANGUAGE_RE.captures(string) {
        Some(captures) => match captures.at(1) {
            None => {
                match captures.at(2) {
                    Some(x) => Some(x.to_string()),
                    None => None
                }
            },
            Some(x) => Some(x.to_string())
        },
        None => None
    };
}


pub fn parse_user_agent(user_agent: &str) -> UserAgent{
    return UserAgent {
        browser: parse_browser(user_agent),
        platform: parse_platform(user_agent),
        language: parse_language(user_agent)
    };

}


#[test]
fn test_firefox() {
    let ua = "Mozilla/5.0 (Macintosh; U; Intel Mac OS X; en-US; rv:1.8.1.11) Gecko/20071127 Firefox/2.0.0.11";
    assert_eq!(parse_browser(ua), Some(Browser {name: "firefox".to_string(), version: Some("2.0.0.11".to_string())}));
    assert_eq!(parse_platform(ua), Some("macos".to_string()));
    assert_eq!(parse_language(ua), Some("en-US".to_string()));
}

#[test]
fn test_opera() {
    let ua = "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; de-DE) Opera 8.54";
    assert_eq!(parse_browser(ua), Some(Browser {name: "opera".to_string(), version: Some("8.54".to_string())}));
    assert_eq!(parse_platform(ua), Some("windows".to_string()));
    assert_eq!(parse_language(ua), Some("de-DE".to_string()));
}

#[test]
fn test_iphone() {
    let ua = "Mozilla/5.0 (iPhone; U; CPU like Mac OS X; en) AppleWebKit/420 (KHTML, like Gecko) Version/3.0 Mobile/1A543a Safari/419.3";
    assert_eq!(parse_browser(ua), Some(Browser {name: "safari".to_string(), version: Some("419.3".to_string())}));
    assert_eq!(parse_platform(ua), Some("iphone".to_string()));
    assert_eq!(parse_language(ua), Some("en".to_string()));
}

#[test]
fn test_google() {
    let ua = "Bot Googlebot/2.1 ( http://www.googlebot.com/bot.html)";
    assert_eq!(parse_browser(ua), Some(Browser {name: "google".to_string(), version: Some("2.1".to_string())}));
    assert_eq!(parse_platform(ua), None);
    assert_eq!(parse_language(ua), None);
}

#[test]
fn test_chome() {
    let ua = "Mozilla/5.0 (X11; CrOS armv7l 3701.81.0) AppleWebKit/537.31 (KHTML, like Gecko) Chrome/26.0.1410.57 Safari/537.31";
    assert_eq!(parse_browser(ua), Some(Browser {name: "chrome".to_string(), version: Some("26.0.1410.57".to_string())}));
    assert_eq!(parse_platform(ua), Some("chromeos".to_string()));
    assert_eq!(parse_language(ua), None);
}

#[test]
fn test_msie() {
    let ua = "Mozilla/5.0 (Windows NT 6.3; Trident/7.0; .NET4.0E; rv:11.0) like Gecko";
    assert_eq!(parse_browser(ua), Some(Browser {name: "msie".to_string(), version: Some("11.0".to_string())}));
    assert_eq!(parse_platform(ua), Some("windows".to_string()));
    assert_eq!(parse_language(ua), None);
}
