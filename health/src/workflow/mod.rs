//! This script serves to check that all those workflows that use path filters
//! filter for sub-projects as well as itself.
//!
//! In the future if the complexity grows and we use path filters for other
//! things, this script needs to be updated. (This script currently will also
//! assert that we _only_ use path filters for the known purpose.)

mod types;

use types::GithubWorkflow;

use std::fs;
use std::path::{Path, PathBuf};

use yaml_rust2::YamlLoader;

const WORKFLOW_DIR: &str = ".github/workflows";
const MAIN_CI_YML: &str = "ci.yml";

fn get_yml_files_in_dir<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let files = fs::read_dir(dir).unwrap();
    files
        .into_iter()
        .filter_map(|v| v.ok())
        .filter_map(|v| {
            let path = v.path();
            let extension = path.extension()?;
            if extension == "yml" || extension == "yaml" { Some(path) } else { None }
        })
        .collect()
}

fn check_main_ci_yml() {
    let yml_path = Path::new(WORKFLOW_DIR).join(MAIN_CI_YML);
    let raw_yml = match fs::read_to_string(&yml_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            panic!("Failed to read workflow file: {}", yml_path.display());
        }
    };
    let docs = match YamlLoader::load_from_str(&raw_yml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            panic!("Failed to parse workflow: {}", yml_path.display());
        }
    };
    let wf = GithubWorkflow::from(&docs[0]);
    for job in wf.jobs() {
        println!("{:?}", job);
    }
}

pub fn main() {
    let workflow_yml_paths = get_yml_files_in_dir(WORKFLOW_DIR);

    match workflow_yml_paths.len() {
        0 => println!("WARNING: no workflows found."),
        n => println!("Found {n} workflow(s). Validating..."),
    }

    for workflow_yml_path in &workflow_yml_paths {
        let raw_yml = match fs::read_to_string(workflow_yml_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e:?}");
                panic!("Failed to read workflow file: {}", workflow_yml_path.display());
            }
        };
        let docs = match YamlLoader::load_from_str(&raw_yml) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e:?}");
                panic!("Failed to parse workflow: {}", workflow_yml_path.display());
            }
        };
        let wf = GithubWorkflow::from(&docs[0]);
        wf.assert_subproject_link(workflow_yml_path);
        wf.assert_uses_version("actions/checkout", "v6");
        wf.assert_uses_version("actions/setup-go", "v6");
        wf.assert_uses_version("cloudflare/wrangler-action", "v4");
        wf.assert_uses_version("mlugg/setup-zig", "v2");
        wf.assert_uses_version("actions/upload-artifact", "v7");
        wf.assert_uses_version("softprops/action-gh-release", "v3");
        wf.assert_uses_version("dorny/paths-filter", "v4");

        // Pretty-print the workflow
        let mut path_display =
            format!("*> \x1b[36m{}\x1b[m ", workflow_yml_path.display());
        if let Some(name) = wf.name {
            path_display.push_str(&format!("({name}) "));
        }
        println!("{}", path_display);
        for j in wf.jobs() {
            match j.name {
                Some(name) => println!("  - {} ({name})", j.yml_key),
                None => println!("  - {}", j.yml_key),
            }
        }
    }

    println!("All {n} workflow(s) validated.", n = workflow_yml_paths.len());

    check_main_ci_yml();
}
