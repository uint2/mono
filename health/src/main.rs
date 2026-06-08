//! This script serves to check that all those workflows that use path filters
//! filter for sub-projects as well as itself.
//!
//! In the future if the complexity grows and we use path filters for other
//! things, this script needs to be updated. (This script currently will also
//! assert that we _only_ use path filters for the known purpose.)

use std::fs;
use std::path::{Path, PathBuf};

use yaml_rust2::YamlLoader;

const WORKFLOW_DIR: &str = ".github/workflows";

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

fn main() {
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
        let tree = &docs[0];

        // First, we filter for the workflows with a on.push.paths value.
        let path_filters = &tree["on"]["push"]["paths"];
        if path_filters.is_badvalue() {
            continue;
        }

        // Check that there are only two filters: one for the sub-project
        // directory, and one for itself.
        let path_filters = path_filters.as_vec().unwrap();
        assert_eq!(path_filters.len(), 2);

        let f1 = path_filters[0].as_str().unwrap();
        let f2 = path_filters[1].as_str().unwrap();

        assert!(f1.ends_with("**/*"), "The first path filter should be all-capturing.");
        let project_subdir = {
            let mut f1 = f1.strip_suffix("**/*").unwrap();
            while f1.ends_with('/') {
                f1 = &f1[..f1.len() - 1];
            }
            Path::new(f1)
        };

        let stat = fs::metadata(project_subdir).unwrap();
        assert!(
            stat.is_dir(),
            "Subproject directory does not exist. Has it moved/been deleted?"
        );

        assert_eq!(
            f2, workflow_yml_path,
            "The second path filter should be the path to the workflow itself."
        );

        println!("{:?}", workflow_yml_path);
        println!("name: {:?}", tree["name"].as_str());
        if let Some(jobs) = tree["jobs"].as_vec() {
            println!("jobs:");
            for j in jobs {
                println!("{:?}", j);
            }
        }
        println!()
    }

    println!("All {n} workflow(s) validated.", n = workflow_yml_paths.len());
}
