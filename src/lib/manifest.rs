use std::{collections::HashMap, fmt::Debug};

use mlua::{DeserializeOptions, Lua, LuaSerdeExt, Table};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use smart_default::SmartDefault;

use crate::{self as krait};

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct Manifest {
    pub repo: String,
    pub latest_commit: String,
    pub timestamp: u64,

    // HashMap<name, HashMap<version, Vec<ManifestPackage>>>
    pub packages: HashMap<String, HashMap<String, Vec<ManifestPackage>>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct ManifestPackage {
    pub commit: String,
    pub path: String,
    pub timestamp: u64,
    pub contents: Vec<ManifestPackageContent>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct ManifestPackageContent {
    pub name: String,
    pub path: String,

    // used for consistency sake because git still uses sha1
    pub sha1: String,
    pub url: String,
}

impl mlua::UserData for Manifest {}
impl mlua::UserData for ManifestPackage {}
impl mlua::UserData for ManifestPackageContent {}

impl Manifest {
    pub fn parse(s: String) -> Self {
        let lua = Lua::new();
        let globals = lua.globals();

        let krait_table = lua.create_table().expect("Failed to create krait table");
        let manifest_table = lua.create_table().expect("Failed to create manifest table");

        krait_table
            .set("manifest", manifest_table)
            .expect("Failed to set manifest table");

        globals
            .set("krait", krait_table)
            .expect("Failed to set krait table");

        // load the manifest
        let result = lua.load(&s).exec();

        if let Err(e) = result {
            eprintln!("Error parsing manifest: {}", e);
            krait::exit!(1);
        }

        // get the config as a table
        let krait_table: Table = globals.get("krait").expect("failed to get krait table");
        let manifest_table: Table = krait_table
            .get("manifest")
            .expect("failed to get manifest table");

        let options = DeserializeOptions::new()
            .deny_unsupported_types(false)
            .deny_recursive_tables(false);

        let manifest: Manifest =
            match lua.from_value_with(mlua::Value::Table(manifest_table), options) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing manifest: {}", e);
                    krait::exit!(1);
                }
            };

        dbg!(&manifest);

        manifest
    }

    pub fn gen_manifest() {
        // check if the current directory is a git repo
        // if not, exit

        let cd = match std::env::current_dir() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
                krait::exit!(1);
            }
        };

        let repo = match git2::Repository::discover(&cd) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: Current directory is not a git repository: {}", e);
                krait::exit!(1);
            }
        };

        // get the manifest.lua file
        let manifest_path = cd.join("manifest.lua");

        // check if the manifest.lua file exists and if it does, read it
        let mut manifest = match std::fs::read_to_string(&manifest_path) {
            Ok(m) => Manifest::parse(m),
            Err(_) => {
                // create a new empty manifest.lua file
                match std::fs::File::create(&manifest_path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating manifest.lua file: {}", e);
                        krait::exit!(1);
                    }
                };

                Manifest::default()
            }
        };

        // get the current branch
        let branch = match repo.head() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error getting current branch: {}", e);
                krait::exit!(1);
            }
        };

        let branch_name = match branch.shorthand() {
            Some(b) => b,
            None => {
                eprintln!("Error getting current branch name");
                krait::exit!(1);
            }
        };

        // get the latest commit
        let latest_commit = match repo.head() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error getting latest commit: {}", e);
                krait::exit!(1);
            }
        };

        let latest_commit = match latest_commit.peel_to_commit() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error getting latest commit: {}", e);
                krait::exit!(1);
            }
        };

        manifest.latest_commit = latest_commit.id().to_string();

        // get the last update time
        manifest.timestamp = latest_commit.time().seconds() as u64;

        // get the repo url
        // if manifest.repo.is_empty() {
        //     let remotes = match repo.remotes() {
        //         Ok(r) => r,
        //         Err(e) => {
        //             eprintln!("Error getting repo url: {}", e);
        //             krait::exit!(1);
        //         }
        //     };

        //     let mut valid = false;
        //     let mut repo_url = String::new();
        //     remotes.into_iter().for_each(|remote| {
        //         if let Some(remote) = remote {
        //             if remote.contains("github") {
        //                 valid = true;
        //                 repo_url = remote.to_string();
        //             }
        //         }
        //     });

        //     if !valid {
        //         eprintln!("Error: No valid remote found");
        //         krait::exit!(1);
        //     }

        //     manifest.repo = repo_url;
        // }

        // ask the user for remote url
        if manifest.repo.is_empty() {
            let mut repo_url;
            loop {
                repo_url = krait::scanln!("Enter the remote location for the repo (user/repo): ");

                if repo_url.is_empty() {
                    eprintln!("Error: No remote url entered");
                    krait::exit!(1);
                }

                // enter name of the org
                let org = krait::scanln!("Enter the name of the org: ");

                if org.is_empty() || org.contains("github") {
                    break;
                }

                eprintln!("Error: Invalid remote url");
            }

            manifest.repo = repo_url;
        }

        // read the packages directory and get the packages and their path relative to the repo root
        let packages_dir = cd.join("packages");

        if !packages_dir.exists() {
            eprintln!("Error: No packages directory found");
            krait::exit!(1);
        }

        let mut package_dirs = Vec::new();
        match std::fs::read_dir(&packages_dir) {
            Ok(p) => {
                p.into_iter().for_each(|p| {
                    if let Ok(p) = p {
                        if p.path().is_dir() {
                            package_dirs.push(p.path());
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error reading packages directory: {}", e);
                krait::exit!(1);
            }
        }

        for package_dir in package_dirs {
            let package_name = package_dir
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let package_manifest_path = package_dir.join("manifest.lua");

            if !package_manifest_path.exists() {
                eprintln!(
                    "Error: No manifest.lua file found in package {}",
                    package_name
                );
                krait::exit!(1);
            }

            let package_manifest_str = match std::fs::read_to_string(&package_manifest_path) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error reading package manifest: {}", e);
                    krait::exit!(1);
                }
            };

            let package_manifest = krait::pkg::PkgInfo::parse(package_manifest_str);

            // get the last commit for the package
            // let package_commit =
            //     match repo.revparse_single(&format!("HEAD:packages/{}", package_name)) {
            //         Ok(c) => c,
            //         Err(e) => {
            //             eprintln!(
            //                 "Error getting last commit for package {}: {}",
            //                 package_name, e
            //             );
            //             krait::exit!(1);
            //         }
            //     };

            // let package_commit = match package_commit.peel_to_commit() {
            //     Ok(c) => c,
            //     Err(e) => {
            //         eprintln!(
            //             "Error getting last commit for package {}: {}",
            //             package_name, e
            //         );
            //         krait::exit!(1);
            //     }
            // };

            // package path relative to the repo root
            let package_path = format!("packages/{}", package_name);

            // check for contents of the package
            let package_contents = match std::fs::read_dir(&package_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading package contents: {}", e);
                    krait::exit!(1);
                }
            };

            let mut contents: Vec<ManifestPackageContent> = Vec::new();

            for content in package_contents.flatten() {
                let content_path = content.path();

                if content_path.is_dir() {
                    eprintln!("Error: Package {} contains a directory", package_name);
                    eprintln!("Directories are not currently supported");
                    krait::exit!(1);
                }

                let content_name = content_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let content_path = format!("{}/{}", package_path, content_name);

                // hash the file using sha1
                let mut hasher = Sha1::new();
                let mut file = match std::fs::File::open(&content_path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error opening file {}: {}", content_path, e);
                        krait::exit!(1);
                    }
                };

                match std::io::copy(&mut file, &mut hasher) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Error hashing file {}: {}", content_path, e);
                        krait::exit!(1);
                    }
                };

                let hash_bytes = hasher.finalize();

                let hash = format!("{:x}", hash_bytes);

                // check if the branch name is main or master
                let branch_name = if branch_name == "main" || branch_name == "master" {
                    // package_commit as branch name
                    manifest.latest_commit.clone()
                } else {
                    // branch name
                    branch_name.to_string()
                };

                // get the download url
                let download_url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}",
                    manifest.repo, branch_name, content_path
                );

                // TODO: add support non-github repos

                contents.push(ManifestPackageContent {
                    name: content_name,
                    path: content_path,
                    sha1: hash,
                    url: download_url,
                });
            }

            let package = ManifestPackage {
                path: package_path,
                commit: manifest.latest_commit.clone(),
                timestamp: manifest.timestamp,
                contents,
            };

            // get the version from the package manifest
            let version = package_manifest.ver;

            let mut packages = manifest.packages.clone();

            if packages.contains_key(&package_name) {
                // check if the version is already in the manifest
                if packages[&package_name].contains_key(&version) {
                    // append the package to the existing version
                    packages
                        .get_mut(&package_name)
                        .unwrap()
                        .get_mut(&version)
                        .unwrap()
                        .push(package);
                } else {
                    // add the version to the package
                    packages
                        .get_mut(&package_name)
                        .unwrap()
                        .insert(version.clone(), vec![package]);
                }
            } else {
                // add the package to the manifest
                let mut hashmap = HashMap::new();
                hashmap.insert(version.clone(), vec![package]);

                packages.insert(package_name.clone(), hashmap);
            }

            manifest.packages = packages;
        }

        println!("{:#?}", manifest);

        todo!("generate the script for the manifest");
    }
}
