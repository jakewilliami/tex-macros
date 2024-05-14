// Fetch resource!

// https://stackoverflow.com/a/73840814/12069968
#[path = "config.rs"] mod config;
// include!("remote.rs");
#[path = "remote.rs"] mod remote;

use std::{fs, path::Path};
use home;

#[derive(PartialEq)]
pub enum ResourceLocation {
    Local,
    Remote,
}

pub fn fetch_resource(resource: &str, loc: &ResourceLocation) -> String {
    match loc {
        ResourceLocation::Local => fetch_resource_local(resource),
        ResourceLocation::Remote => fetch_resource_remote(resource),
    }
}

fn fetch_resource_local(resource: &str) -> String {
    let resource_dir = home::home_dir().expect("Cannot get home directory")
        .join("projects").join(config::GITHUB_REPO_NAME);
    if !resource_dir.as_path().exists() {
        panic!("{}",
            format!(
                "No local resource path at ~/projects/{}/",
                config::GITHUB_REPO_NAME
            )
        )
    }

    // Adjoining an absolute path replaces the existing path
    // As such, we need to account for these in the resource
    let resource = Path::new(resource.trim_start_matches('/'));

    let resource_path = resource_dir.join(&resource);
    fs::read_to_string(resource_path).unwrap()
}

fn fetch_resource_remote(resource: &str) -> String {
    remote::get_remote_resource(resource, "master")
}
