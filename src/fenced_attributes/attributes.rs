use anyhow::Result;
use hocon::HoconLoader;
use serde::Deserialize;
use uuid::Uuid;

fn serde_default_true() -> bool {
    true
}

/// Code chunk attributes, can be used to defined filters, path, arguments and etc
#[derive(Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Attributes {
    /// optional id, to use for TODO:dependencies
    pub id: Option<String>,

    /// system filter, e.g `linux`, `macos`, `windows` and etc.
    /// derived from https://doc.rust-lang.org/std/env/consts/constant.OS.html
    pub sys: Option<Vec<String>>,

    /// linux distro filter, e.g. `arch`, `debian` and etc.
    /// derived from release ID_LIKE
    pub linux_distro: Option<Vec<String>>,

    /// system architecture filter, e.g.`x86_64`, `arm` and etc.
    /// derived from https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
    pub arch: Option<Vec<String>>,

    /// Code chunk executor name or path, e.g. `sh`, `node` and etc.
    pub cmd: Option<String>,

    /// Code chunk executor arguments
    pub args: Option<Vec<String>>,

    /// `PATH` env variable for the commands
    pub path: Option<String>,

    /// determines if gem should executed  the Code chunk as file, default `true`
    #[serde(default = "serde_default_true")]
    pub as_file: bool,

    /// determines if gem should display stdout of the Code chunk, default `true`
    #[serde(default = "serde_default_true")]
    pub stdout: bool,

    /// determines if gem should allow warnings, default `true`
    #[serde(default = "serde_default_true")]
    pub allow_warnings: bool,

    /// determines if gem should TODO: allow errors, default `true`
    #[serde(default)]
    pub allow_errors: bool,

    /// tells gem to run the Code chunk in sudo
    #[serde(default)]
    pub with_sudo: bool,
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

    #[allow(dead_code)]
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
