use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::{self, Write},
    path::Path,
    str::FromStr,
};

use serde::Serialize;

use handlebars::Handlebars;

use crate::ExportStrings;

static PACKAGE_FILE_TEMPLATE: &[u8] = include_bytes!("gen_src/package-template.json");

/// Builds a npm packet from create language binding strings.
///
/// # Example
/// ```ignore
/// #[derive(Serialize, PostcardBindings)]
/// struct Test {
///     field: u8
/// }
///
/// let parent_dir = std::env::current_dir().unwrap().as_path();
/// build_npm_packet(parent_dir, generate_bindings!(Test))
/// ```
pub fn build_npm_package(
    parent_dir: &Path,
    packet_info: PacketInfo,
    bindings: ExportStrings,
) -> io::Result<()> {
    let mut dir = parent_dir.to_path_buf();
    dir.push(packet_info.name.as_str());

    std::fs::create_dir_all(&dir)?;

    let package_json =
        package_file_src(packet_info.name.to_owned(), packet_info.version.to_string()).unwrap();

    let mut package_json_path = dir.to_owned();
    package_json_path.push("package.json");
    File::create(package_json_path.as_path())?.write_all(package_json.as_bytes())?;

    let mut js_export_path = dir.to_owned();
    js_export_path.push("index.js");
    File::create(js_export_path.as_path())?.write_all(bindings.js_file.as_bytes())?;

    let mut js_export_path = dir;
    js_export_path.push("index.d.ts");
    File::create(js_export_path.as_path())?.write_all(bindings.ts_file.as_bytes())?;

    Ok(())
}

fn package_file_src(
    package_name: String,
    package_version: String,
) -> Result<String, handlebars::RenderError> {
    #[derive(Serialize)]
    struct TemplateData {
        package_name: String,
        package_version: String,
    }

    let template_data = TemplateData {
        package_name,
        package_version,
    };
    Handlebars::new().render_template(
        String::from_utf8(PACKAGE_FILE_TEMPLATE.into())
            .unwrap()
            .as_str(),
        &template_data,
    )
}

/// Defines a package version with major, minor, patch version numbers.
///
/// # Examples
/// ```
/// # use postcard_bindgen_core::Version;
/// let version = Version::from_array([2, 10, 2]);
/// assert_eq!(version.to_string(), String::from("2.10.2"))
/// ```
///
/// ```
/// # use std::str::FromStr;
/// # use postcard_bindgen_core::Version;
/// let version = Version::from_str("2.10.2").unwrap();
/// assert_eq!(version.to_string(), String::from("2.10.2"))
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Wraps more infos for the npm package.
pub struct PacketInfo {
    pub name: String,
    pub version: Version,
}

impl Version {
    pub fn from_array(parts: [u32; 3]) -> Self {
        Self {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }
}

/// Error type indicates that supplied string is not a version.
pub struct VersionFromStrError;

impl Debug for VersionFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "supplied string not a version format - <major.minor.patch>"
        )
    }
}

impl Display for VersionFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VersionFromStrError {}

impl FromStr for Version {
    type Err = VersionFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            Err(VersionFromStrError)
        } else {
            Ok(Self {
                major: u32::from_str(parts[0]).map_err(|_| VersionFromStrError)?,
                minor: u32::from_str(parts[1]).map_err(|_| VersionFromStrError)?,
                patch: u32::from_str(parts[2]).map_err(|_| VersionFromStrError)?,
            })
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = VersionFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
