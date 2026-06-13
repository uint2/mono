use std::fs;
use std::path::Path;

use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct GithubWorkflowStep<'a> {
    pub name: Option<&'a str>,
    pub uses: Option<&'a str>,
    pub working_directory: Option<&'a str>,
}

impl<'a> From<&'a Yaml> for GithubWorkflowStep<'a> {
    fn from(value: &'a Yaml) -> Self {
        Self {
            name: value["name"].as_str(),
            uses: value["uses"].as_str(),
            working_directory: value["working-directory"].as_str(),
        }
    }
}

#[derive(Debug)]
pub struct GithubWorkflowJob<'a> {
    pub yml_key: &'a str,
    pub name: Option<&'a str>,
    pub steps: Vec<GithubWorkflowStep<'a>>,
}

/// Represents the data in a YAML file of a GitHub Workflow.
#[derive(Debug)]
pub struct GithubWorkflow<'a> {
    pub name: Option<&'a str>,
    pub needs: Option<Vec<&'a str>>,
    pub jobs: Option<Vec<GithubWorkflowJob<'a>>>,
    pub on: &'a Yaml,
}

impl<'a> From<&'a Yaml> for GithubWorkflow<'a> {
    fn from(value: &'a Yaml) -> Self {
        let mut z = Self {
            name: value["name"].as_str(),
            needs: None,
            jobs: None,
            on: &value["on"],
        };
        if let Some(yml_jobs) = value["jobs"].as_hash() {
            let mut jobs = Vec::with_capacity(yml_jobs.len());
            for (key, value) in yml_jobs {
                let yml_steps = value["steps"].as_vec().unwrap();
                jobs.push(GithubWorkflowJob {
                    yml_key: key.as_str().unwrap(),
                    name: value["name"].as_str(),
                    steps: yml_steps.into_iter().map(GithubWorkflowStep::from).collect(),
                });
            }
            z.jobs = Some(jobs);
        }
        if let Some(yml_need) = value["needs"].as_str() {
            z.needs = Some(vec![yml_need])
        } else if let Some(yml_needs) = value["needs"].as_vec() {
            let mut needs = Vec::with_capacity(yml_needs.len());
            for i in 0..needs.len() {
                needs.push(yml_needs[i].as_str().unwrap());
            }
            z.needs = Some(needs)
        }
        z
    }
}

impl GithubWorkflow<'_> {
    pub fn jobs(&self) -> core::slice::Iter<'_, GithubWorkflowJob<'_>> {
        let Some(jobs) = &self.jobs else { return [].iter() };
        jobs.iter()
    }

    /// Check that if path filters are used, there are only two: one to
    /// filter for sub-projects, and one to point to itself.
    ///
    /// Passing this assertion signals that this workflow is meant to be a
    /// replacement for a sub-project's own GitHub workflow.
    pub fn assert_subproject_link(&self, workflow_path: &Path) {
        // Obtain the path filters. If there are none, then this workflow runs
        // conditionally on other conditions. We have nothing to do here.
        let Some(path_filters) = self.on["push"]["paths"].as_vec() else { return };

        assert_eq!(
            path_filters.len(),
            2,
            "If path filters are used at all, use only 2. See docs for more."
        );

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
            f2, workflow_path,
            "The second path filter should be the path to the workflow itself."
        );
    }

    /// Checks that all steps with `uses` pointing to a github repo has is of
    /// a particular tagged version.
    pub fn assert_uses_version(&self, name: &'static str, version: &'static str) {
        let ender = format!("@{version}");
        for job in self.jobs() {
            for step in &job.steps {
                let Some(uses) = step.uses else { continue };
                let uses = uses.trim_start();
                if uses.starts_with(name) {
                    assert!(uses.ends_with(&ender), "Use \"{version}\" of {name}.");
                }
            }
        }
    }
}
