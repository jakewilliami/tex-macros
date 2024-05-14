// Fetch resource remotely

#[path = "config.rs"] mod config;
use config::*;

use reqwest;
use serde_json;

// use "master" for tag
pub fn get_remote_resource(resource: &str, tag: &str) -> String {
    let uri = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        GITHUB_USER,
        GITHUB_REPO_NAME,
        tag,
        resource,
    );
    reqwest::blocking::get(uri)
        .expect("Cannot get remote resource")
        .text().expect("Cannot get text from remote response")
}

/// Get latest commit hash (SHA1 ID) from the remote repository.
///
/// This method uses GitHub's repo API to fetch HEAD information at
/// the main branch (in this case, master).
pub fn get_latest_commit_hash() -> String {
    // https://docs.github.com/en/rest/git/refs?apiVersion=2022-11-28
    let uri = format!(
        "https://api.github.com/repos/{}/{}/git/ref/heads/{}",
        GITHUB_USER,
        GITHUB_REPO_NAME,
        MAIN_BRANCH,
    );

    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
    headers.append("accept", "application/json".parse().unwrap());
    headers.append("user-agent", "mktex.rs".parse().unwrap());

    let request = client
        .get(uri)
        .headers(headers);

    let body = request.send().expect("Cannot get response from GitHub API")
        .text().expect("Cannot get text from remote response");

    let commit_data_raw: serde_json::Value = serde_json::from_str(&body)
        .expect("The JSON response was not well defined");
    let latest_commit = commit_data_raw.as_object().expect("Cannot parse response as object")
        .get("object").expect("Cannot get latest object information from remote ref")
        .get("sha").expect("Cannot get commit hash from response");

    latest_commit.to_string()
}

/*
/// Get latest commit hash (SHA1 ID) from the remote repository.
///
/// Like the main get_latest_commit_hash method above, this method uses
/// GitHub's API, however, using the commits endpoint, and specifying a page
/// size of 1.  From this response, we can glean information about the
/// latest commit.  However, note that this endpoint always returns the
/// _topological_ latest commits, but it might just return the one with
/// the latest dates (which is unrelated).  Use this method with caution.
/// The repos endpoint/method (above) is desirable for accuracy, as well as
/// the fact that it is a smaller response/cheaper process.
pub fn get_latest_commit_hash_commits_api() -> String {
    // https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28
    let uri = format!(
        "https://api.github.com/repos/{}/{}/commits",
        GITHUB_USER,
        GITHUB_REPO_NAME
    );

    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
    headers.append("accept", "application/json".parse().unwrap());
    headers.append("user-agent", "mktex.rs".parse().unwrap());

    let request = client
        .get(uri)
        .headers(headers);

    let body = request.send().expect("Cannot get response from GitHub API")
        .text().expect("Cannot get text from remote response");

    let commit_data_raw: serde_json::Value = serde_json::from_str(&body)
        .expect("The JSON response was not well defined");
    let latest_commit = commit_data_raw.as_array().expect("Cannot parse response as array")
        .first().expect("Cannot get first element from the array")
        .as_object().expect("Cannot parse element of response array as object")
        .get("sha").expect("Cannot get commit hash from response");

    latest_commit.to_string()
}

/// Get latest commit hash (SHA1 ID) from the remote repository.
///
/// This method uses libgit2 bindings to pull this information.
pub fn get_latest_commit_hash_git2() -> String {
    let uri = format!("https://github.com/{}/{}.git", GITHUB_USER, GITHUB_REPO_NAME);
    let mut repo = git2::Remote::create_detached(uri).expect("Cannot create remote repo instance");
    repo.connect(git2::Direction::Fetch).expect("Cannot connect to remote repo");

    // Create map of hashes
    let remote_heads = repo.list().expect("Cannot get remote head list from remote repo");
    let commit_map: HashMap<String, String> = remote_heads
        .iter()
        .map(|rh| (rh.name().to_string(), rh.oid().to_string()))
        .collect();

    repo.disconnect().expect("Cannot disconnect to remote repo");

    // Try to get refs/heads/<branch>
    let preferred_head_name = format!("refs/heads/{}", MAIN_BRANCH);
    if commit_map.contains_key(&preferred_head_name) {
        return commit_map.get(&preferred_head_name)
            .expect("Cannot get refs head name from commit map").to_owned();
    }

    // Default to HEAD
    if commit_map.contains_key("HEAD") {
        return commit_map.get("HEAD")
            .expect("Cannot get HEAD commit from commit map").to_owned();
    }

    panic!("Cannot find latest commit hash using git2");
}

/// Get latest commit hash (SHA1 ID) from the remote repository.
///
/// This method uses a GET request to `{repo}/info/refs` with query
/// `parameter service=git-upload-pack`.  Note that this method manually
/// decodes the packet line document/response for the latest commit ID.
/// This will return a list of all the refs (branches, tags, etc.), and
/// the hashes they map to.  The benefit of this method is that it works
/// on everything, not just GitHub (like the GitHub APIs I am using).
pub fn get_latest_commit_hash_pkt_line() -> String {
    let uri = format!(
        "https://github.com/{}/{}/info/refs",
        GITHUB_USER,
        GITHUB_REPO_NAME,
    );

    let client = reqwest::blocking::Client::new();

    let request = client
        .get(uri)
        .query(&[("service", "git-upload-pack")]);

    let body = request.send().expect("Cannot get response from GitHub API")
        .text().expect("Cannot get text from remote response");

    // Here, we parse the packet lines manually.  However, see:
    // https://docs.rs/gix-packetline/
    let lines: Vec<&str> = body.lines().collect();
    let head = format!("refs/heads/{}", MAIN_BRANCH);
    let latest_commit_line = lines.iter().find(|&&line| line.ends_with(head.as_str()));
    if let Some(latest_commit_line) = latest_commit_line {
        let commit_hash = latest_commit_line.split_whitespace().next().expect("Cannot get commit hash from ref line");
        return commit_hash[4..].to_string()
    }

    panic!("Cannot find latest commit hash using HTTP/git-upload-pack");
}

/// Get latest commit hash (SHA1 ID) from the remote repository.
///
/// This method uses gitoxide.
pub fn get_latest_commit_hash_gitoxide() -> String {
    // https://github.com/Byron/gitoxide/discussions/930
    unimplemented!()
}
*/
