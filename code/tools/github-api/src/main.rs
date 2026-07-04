macro_rules! github {
    ($($e:expr),* $(,)?) => {
        concat!("https://api.github.com", $($e)*)
    };
}

mod api;

use reqwest::ClientBuilder;
use reqwest::header::{self, HeaderMap, HeaderValue};

/// Use the GITHUB_TOKEN environment variable.
fn get_token() -> String {
    std::env::var("GITHUB_TOKEN").unwrap()
}

fn get_headers() -> HeaderMap {
    use HeaderValue as HV;

    let mut headers = HeaderMap::new();

    // Get the token into "Authorization: Bearer YOUR-TOKEN".
    let bearer_token = format!("Bearer {}", get_token());
    let mut token = HV::from_str(&bearer_token).unwrap();
    token.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, token);

    // Set "X-GitHub-Api-Version: 2026-03-10"
    headers.insert("X-GitHub-Api-Version", HV::from_static("2026-03-10"));

    headers.insert(header::ACCEPT, HV::from_static("application/vnd.github+json"));

    // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
    headers.insert(header::USER_AGENT, HV::from_static("curl/8.14.1"));

    headers
}

#[tokio::main]
async fn main() {
    let headers = get_headers();
    let client = ClientBuilder::new().default_headers(headers).build().unwrap();

    let mut repos = api::list_user_repos(&client).await;
    repos.sort_by(|a, b| {
        let a = a["full_name"].as_str().unwrap();
        let b = b["full_name"].as_str().unwrap();
        a.cmp(&b)
    });
    for repo in &repos {
        println!("{}", repo["full_name"]);
    }
    println!("{} repos", repos.len());
}
