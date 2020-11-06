use std::{env, fmt};

use hubcaps::releases::Release as HubcapsRelease;
use serde::{Deserialize, Serialize};

use crate::result::Result;
use serde::export::fmt::Display;
use serde::export::Formatter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub description: Option<String>,
    pub source: PackageSource,
    pub targets: Vec<PackageTargetType>,
    pub detail: Option<PackageDetailType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageSummary {
    pub name: String,
    pub description: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageSource {
    Github { owner: String, repo: String },
    Helm { registry: String, repo: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageDetailType {
    Github { package: GithubPackage },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageTargetType {
    LinuxAmd64(PackageManagement),
    LinuxArm64(PackageManagement),
    MacOS(PackageManagement),
    Windows(PackageManagement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageManagement {
    pub artifact_templates: Vec<String>,
    pub checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstall_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_commands: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubPackage {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: u64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubAsset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageIndex {
    pub name: String,
    pub owner: String,
    pub source: String,
}

impl PackageSource {
    pub fn url(&self) -> String {
        match self {
            PackageSource::Github { owner, repo } => {
                format!("https://github.com/{}/{}", owner, repo)
            }

            _ => "".to_string(),
        }
    }

    pub fn owner(&self) -> String {
        match self {
            PackageSource::Github { owner, repo: _ } => format!("{}", owner),
            PackageSource::Helm { registry, repo: _ } => format!("{}", registry),
        }
    }
}

impl Package {
    pub fn target(&self) -> Result<PackageManagement> {
        // https://doc.rust-lang.org/std/env/consts/index.html
        let os = env::consts::OS;
        let arch = env::consts::ARCH;

        let e = anyhow!("Unsupported OS {} or ARCH {}", os, arch);

        if os == "linux" {
            return match arch {
                "x86_64" => Ok(self
                    .targets
                    .iter()
                    .find_map(|it| {
                        if let PackageTargetType::LinuxAmd64(m) = it {
                            Some(m.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap()),
                "aarch64" => Ok(self
                    .targets
                    .iter()
                    .find_map(|it| {
                        if let PackageTargetType::LinuxArm64(m) = it {
                            Some(m.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap()
                    .clone()),
                _ => Err(e),
            };
        }

        if os == "macos" {
            return Ok(self
                .targets
                .iter()
                .find_map(|it| {
                    if let PackageTargetType::MacOS(m) = it {
                        Some(m.clone())
                    } else {
                        None
                    }
                })
                .unwrap()
                .clone());
        }

        if os == "windows" {
            return Ok(self
                .targets
                .iter()
                .find_map(|it| {
                    if let PackageTargetType::Windows(m) = it {
                        Some(m.clone())
                    } else {
                        None
                    }
                })
                .unwrap()
                .clone());
        }

        Err(e)
    }
}

impl From<HubcapsRelease> for GithubPackage {
    fn from(r: HubcapsRelease) -> Self {
        Self {
            url: r.url,
            html_url: r.html_url,
            assets_url: r.assets_url,
            upload_url: r.upload_url,
            tarball_url: r.tarball_url,
            zipball_url: r.zipball_url,
            id: r.id,
            tag_name: r.tag_name,
            target_commitish: r.target_commitish,
            name: r.name,
            body: r.body,
            draft: r.draft,
            prerelease: r.prerelease,
            created_at: r.created_at,
            published_at: r.published_at,
            assets: r
                .assets
                .into_iter()
                .map(|it| GithubAsset::from(it))
                .collect(),
        }
    }
}

impl From<hubcaps::releases::Asset> for GithubAsset {
    fn from(a: hubcaps::releases::Asset) -> Self {
        GithubAsset {
            url: a.url,
            browser_download_url: a.browser_download_url,
            id: a.id,
            name: a.name,
            label: a.label,
            state: a.state,
            content_type: a.content_type,
            size: a.size,
            download_count: a.download_count,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Display for PackageSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PackageSource::Github { .. } => write!(f, "{}", "github"),
            PackageSource::Helm { .. } => write!(f, "{}", "helm"),
        }
    }
}

impl From<Package> for PackageSummary {
    fn from(p: Package) -> Self {
        PackageSummary {
            name: p.name.clone(),
            description: p.description.clone(),
            source: Some(p.source.url()),
        }
    }
}