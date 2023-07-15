#[path = "remote.rs"] mod remote;

// https://stackoverflow.com/a/72397385/12069968
use crate::resource::{self, ResourceLocation};

use chrono::prelude::*;
use regex::{Captures, Regex};

pub fn expand_input_paths(contents_raw: String, loc: &ResourceLocation) -> String {
    let re = Regex::new(r"\\input\{(?P<path>.+)\}").unwrap();
    let expanded = re.replace_all(&contents_raw, |caps: &Captures| {
        let input_path = caps.name("path").unwrap().as_str();
        fetch_resource(input_path, loc)
    }).to_string();
    add_version_metadata(expanded, loc)
}

fn fetch_resource(input_path: &str, loc: &ResourceLocation) -> String {
    let resource_path = input_path.split("tex-macros/").last().expect("Cannot find tex-macros/ repo in input path");
    resource::fetch_resource(resource_path, loc)
}

fn add_version_metadata(contents_raw: String, loc: &ResourceLocation) -> String {
    let local_dt: DateTime<Local> = Local::now();
    let formatted_date = local_dt.format("%I:%M %p on %A, %e %B, %Y %Z").to_string();

    let mut contents = String::new();
    contents.push_str(format!("% Frozen version at {}\n\n", formatted_date).as_str());

    if loc == &ResourceLocation::Remote {
        let latest_commit = remote::get_latest_commit_hash();
        contents.pop();  // Remove other new line if remote info added
        contents.push_str(format!("% At commit version {} \n\n", latest_commit).as_str());
    }

    contents.push_str(&contents_raw);

    contents
}
