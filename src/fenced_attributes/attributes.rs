use anyhow::Result;
use hocon::HoconLoader;
use serde::Deserialize;
use uuid::Uuid;

fn serde_default_true() -> bool {
    true
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Attributes {
    pub id: Option<String>,
    pub sys: Option<Vec<String>>,
    pub arch: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub cmd: Option<String>,
    pub path: Option<String>,

    #[serde(default = "serde_default_true")]
    pub as_file: bool,
    #[serde(default = "serde_default_true")]
    pub stdout: bool,
    #[serde(default = "serde_default_true")]
    pub allow_warnings: bool,
    #[serde(default)]
    pub allow_errors: bool,
}

impl Attributes {
    /**
     * Parses block attributes
     * @param text e.g. {#identifier .class1 .class2 key1=value1 key2=value2}
     */
    pub fn parse(text: &str) -> Result<Self> {
        let hocon = HoconLoader::new().load_str(text)?.hocon()?;
        let attributes: Attributes = hocon.resolve()?;
        if attributes.id.is_some() {
            Ok(attributes)
        } else {
            Ok(Attributes {
                id: Some(Uuid::new_v4().to_string()),
                ..attributes
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cmd.is_none() && self.arch.is_none() && self.args.is_none() && self.sys.is_none()
    }
}

/// Tests of fences parsing

#[test]
fn parse_attributes_test() {
    let attributes = Attributes::parse("");
    assert_eq!(attributes.is_ok(), true);

    let attributes = attributes.unwrap();
    assert_eq!(attributes.id.is_some(), true);
    assert_eq!(attributes.is_empty(), true);

    let attributes = Attributes::parse("{id: test, sys=[macos], args=[test1, test2]}}");
    assert_eq!(attributes.is_ok(), true);

    let attributes = attributes.unwrap();
    assert_eq!(attributes.is_empty(), false);
    assert_eq!(attributes.id.unwrap(), "test");
    assert_eq!(attributes.sys, Some(vec!["macos".to_string()]));
    assert_eq!(attributes.arch.is_none(), true);
    assert_eq!(attributes.cmd.is_none(), true);
    assert_eq!(
        attributes.args,
        Some(vec!["test1".to_string(), "test2".to_string()])
    );
}
