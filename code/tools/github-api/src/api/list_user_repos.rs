use reqwest::{Client, header::HeaderMap};
use serde_json::Value;

// {
//   "type": "array",
//   "items": {
//     "title": "Repository",
//     "description": "A repository on GitHub.",
//     "type": "object",
//     "properties": {
//       "id": {
//         "description": "Unique identifier of the repository",
//         "type": "integer",
//         "format": "int64"
//       },
//       "node_id": {
//         "type": "string"
//       },
//       "name": {
//         "description": "The name of the repository.",
//         "type": "string"
//       },
//       "full_name": {
//         "type": "string"
//       },
//       "license": {
//         "anyOf": [
//           {
//             "type": "null"
//           },
//           {
//             "title": "License Simple",
//             "description": "License Simple",
//             "type": "object",
//             "properties": {
//               "key": {
//                 "type": "string"
//               },
//               "name": {
//                 "type": "string"
//               },
//               "url": {
//                 "type": [
//                   "string",
//                   "null"
//                 ],
//                 "format": "uri"
//               },
//               "spdx_id": {
//                 "type": [
//                   "string",
//                   "null"
//                 ]
//               },
//               "node_id": {
//                 "type": "string"
//               },
//               "html_url": {
//                 "type": "string",
//                 "format": "uri"
//               }
//             },
//             "required": [
//               "key",
//               "name",
//               "url",
//               "spdx_id",
//               "node_id"
//             ]
//           }
//         ]
//       },
//       "forks": {
//         "type": "integer"
//       },
//       "permissions": {
//         "type": "object",
//         "properties": {
//           "admin": {
//             "type": "boolean"
//           },
//           "pull": {
//             "type": "boolean"
//           },
//           "triage": {
//             "type": "boolean"
//           },
//           "push": {
//             "type": "boolean"
//           },
//           "maintain": {
//             "type": "boolean"
//           }
//         },
//         "required": [
//           "admin",
//           "pull",
//           "push"
//         ]
//       },
//       "owner": {
//         "title": "Simple User",
//         "description": "A GitHub user.",
//         "type": "object",
//         "properties": {
//           "name": {
//             "type": [
//               "string",
//               "null"
//             ]
//           },
//           "email": {
//             "type": [
//               "string",
//               "null"
//             ]
//           },
//           "login": {
//             "type": "string"
//           },
//           "id": {
//             "type": "integer",
//             "format": "int64"
//           },
//           "node_id": {
//             "type": "string"
//           },
//           "avatar_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "gravatar_id": {
//             "type": [
//               "string",
//               "null"
//             ]
//           },
//           "url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "html_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "followers_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "following_url": {
//             "type": "string"
//           },
//           "gists_url": {
//             "type": "string"
//           },
//           "starred_url": {
//             "type": "string"
//           },
//           "subscriptions_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "organizations_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "repos_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "events_url": {
//             "type": "string"
//           },
//           "received_events_url": {
//             "type": "string",
//             "format": "uri"
//           },
//           "type": {
//             "type": "string"
//           },
//           "site_admin": {
//             "type": "boolean"
//           },
//           "starred_at": {
//             "type": "string"
//           },
//           "user_view_type": {
//             "type": "string"
//           }
//         },
//         "required": [
//           "avatar_url",
//           "events_url",
//           "followers_url",
//           "following_url",
//           "gists_url",
//           "gravatar_id",
//           "html_url",
//           "id",
//           "node_id",
//           "login",
//           "organizations_url",
//           "received_events_url",
//           "repos_url",
//           "site_admin",
//           "starred_url",
//           "subscriptions_url",
//           "type",
//           "url"
//         ]
//       },
//       "private": {
//         "description": "Whether the repository is private or public.",
//         "default": false,
//         "type": "boolean"
//       },
//       "html_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "description": {
//         "type": [
//           "string",
//           "null"
//         ]
//       },
//       "fork": {
//         "type": "boolean"
//       },
//       "url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "archive_url": {
//         "type": "string"
//       },
//       "assignees_url": {
//         "type": "string"
//       },
//       "blobs_url": {
//         "type": "string"
//       },
//       "branches_url": {
//         "type": "string"
//       },
//       "collaborators_url": {
//         "type": "string"
//       },
//       "comments_url": {
//         "type": "string"
//       },
//       "commits_url": {
//         "type": "string"
//       },
//       "compare_url": {
//         "type": "string"
//       },
//       "contents_url": {
//         "type": "string"
//       },
//       "contributors_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "deployments_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "downloads_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "events_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "forks_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "git_commits_url": {
//         "type": "string"
//       },
//       "git_refs_url": {
//         "type": "string"
//       },
//       "git_tags_url": {
//         "type": "string"
//       },
//       "git_url": {
//         "type": "string"
//       },
//       "issue_comment_url": {
//         "type": "string"
//       },
//       "issue_events_url": {
//         "type": "string"
//       },
//       "issues_url": {
//         "type": "string"
//       },
//       "keys_url": {
//         "type": "string"
//       },
//       "labels_url": {
//         "type": "string"
//       },
//       "languages_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "merges_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "milestones_url": {
//         "type": "string"
//       },
//       "notifications_url": {
//         "type": "string"
//       },
//       "pulls_url": {
//         "type": "string"
//       },
//       "releases_url": {
//         "type": "string"
//       },
//       "ssh_url": {
//         "type": "string"
//       },
//       "stargazers_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "statuses_url": {
//         "type": "string"
//       },
//       "subscribers_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "subscription_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "tags_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "teams_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "trees_url": {
//         "type": "string"
//       },
//       "clone_url": {
//         "type": "string"
//       },
//       "mirror_url": {
//         "type": [
//           "string",
//           "null"
//         ],
//         "format": "uri"
//       },
//       "hooks_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "svn_url": {
//         "type": "string",
//         "format": "uri"
//       },
//       "homepage": {
//         "type": [
//           "string",
//           "null"
//         ],
//         "format": "uri"
//       },
//       "language": {
//         "type": [
//           "string",
//           "null"
//         ]
//       },
//       "forks_count": {
//         "type": "integer"
//       },
//       "stargazers_count": {
//         "type": "integer"
//       },
//       "watchers_count": {
//         "type": "integer"
//       },
//       "size": {
//         "description": "The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0.",
//         "type": "integer"
//       },
//       "default_branch": {
//         "description": "The default branch of the repository.",
//         "type": "string"
//       },
//       "open_issues_count": {
//         "type": "integer"
//       },
//       "is_template": {
//         "description": "Whether this repository acts as a template that can be used to generate new repositories.",
//         "default": false,
//         "type": "boolean"
//       },
//       "topics": {
//         "type": "array",
//         "items": {
//           "type": "string"
//         }
//       },
//       "has_issues": {
//         "description": "Whether issues are enabled.",
//         "default": true,
//         "type": "boolean"
//       },
//       "has_projects": {
//         "description": "Whether projects are enabled.",
//         "default": true,
//         "type": "boolean"
//       },
//       "has_wiki": {
//         "description": "Whether the wiki is enabled.",
//         "default": true,
//         "type": "boolean"
//       },
//       "has_pages": {
//         "type": "boolean"
//       },
//       "has_discussions": {
//         "description": "Whether discussions are enabled.",
//         "default": false,
//         "type": "boolean"
//       },
//       "has_pull_requests": {
//         "description": "Whether pull requests are enabled.",
//         "default": true,
//         "type": "boolean"
//       },
//       "pull_request_creation_policy": {
//         "description": "The policy controlling who can create pull requests: all or collaborators_only.",
//         "type": "string",
//         "enum": [
//           "all",
//           "collaborators_only"
//         ]
//       },
//       "archived": {
//         "description": "Whether the repository is archived.",
//         "default": false,
//         "type": "boolean"
//       },
//       "disabled": {
//         "type": "boolean",
//         "description": "Returns whether or not this repository disabled."
//       },
//       "visibility": {
//         "description": "The repository visibility: public, private, or internal.",
//         "default": "public",
//         "type": "string"
//       },
//       "pushed_at": {
//         "type": [
//           "string",
//           "null"
//         ],
//         "format": "date-time"
//       },
//       "created_at": {
//         "type": [
//           "string",
//           "null"
//         ],
//         "format": "date-time"
//       },
//       "updated_at": {
//         "type": [
//           "string",
//           "null"
//         ],
//         "format": "date-time"
//       },
//       "allow_rebase_merge": {
//         "description": "Whether to allow rebase merges for pull requests.",
//         "default": true,
//         "type": "boolean"
//       },
//       "temp_clone_token": {
//         "type": "string"
//       },
//       "allow_squash_merge": {
//         "description": "Whether to allow squash merges for pull requests.",
//         "default": true,
//         "type": "boolean"
//       },
//       "allow_auto_merge": {
//         "description": "Whether to allow Auto-merge to be used on pull requests.",
//         "default": false,
//         "type": "boolean"
//       },
//       "delete_branch_on_merge": {
//         "description": "Whether to delete head branches when pull requests are merged",
//         "default": false,
//         "type": "boolean"
//       },
//       "allow_update_branch": {
//         "description": "Whether or not a pull request head branch that is behind its base branch can always be updated even if it is not required to be up to date before merging.",
//         "default": false,
//         "type": "boolean"
//       },
//       "squash_merge_commit_title": {
//         "type": "string",
//         "enum": [
//           "PR_TITLE",
//           "COMMIT_OR_PR_TITLE"
//         ],
//         "description": "The default value for a squash merge commit title:\n\n- `PR_TITLE` - default to the pull request's title.\n- `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit)."
//       },
//       "squash_merge_commit_message": {
//         "type": "string",
//         "enum": [
//           "PR_BODY",
//           "COMMIT_MESSAGES",
//           "BLANK"
//         ],
//         "description": "The default value for a squash merge commit message:\n\n- `PR_BODY` - default to the pull request's body.\n- `COMMIT_MESSAGES` - default to the branch's commit messages.\n- `BLANK` - default to a blank commit message."
//       },
//       "merge_commit_title": {
//         "type": "string",
//         "enum": [
//           "PR_TITLE",
//           "MERGE_MESSAGE"
//         ],
//         "description": "The default value for a merge commit title.\n\n- `PR_TITLE` - default to the pull request's title.\n- `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name)."
//       },
//       "merge_commit_message": {
//         "type": "string",
//         "enum": [
//           "PR_BODY",
//           "PR_TITLE",
//           "BLANK"
//         ],
//         "description": "The default value for a merge commit message.\n\n- `PR_TITLE` - default to the pull request's title.\n- `PR_BODY` - default to the pull request's body.\n- `BLANK` - default to a blank commit message."
//       },
//       "allow_merge_commit": {
//         "description": "Whether to allow merge commits for pull requests.",
//         "default": true,
//         "type": "boolean"
//       },
//       "allow_forking": {
//         "description": "Whether to allow forking this repo",
//         "type": "boolean"
//       },
//       "web_commit_signoff_required": {
//         "description": "Whether to require contributors to sign off on web-based commits",
//         "default": false,
//         "type": "boolean"
//       },
//       "open_issues": {
//         "type": "integer"
//       },
//       "watchers": {
//         "type": "integer"
//       },
//       "starred_at": {
//         "type": "string"
//       },
//       "anonymous_access_enabled": {
//         "type": "boolean",
//         "description": "Whether anonymous git access is enabled for this repository"
//       },
//       "code_search_index_status": {
//         "type": "object",
//         "description": "The status of the code search index for this repository",
//         "properties": {
//           "lexical_search_ok": {
//             "type": "boolean"
//           },
//           "lexical_commit_sha": {
//             "type": "string"
//           }
//         }
//       }
//     },
//     "required": [
//       "archive_url",
//       "assignees_url",
//       "blobs_url",
//       "branches_url",
//       "collaborators_url",
//       "comments_url",
//       "commits_url",
//       "compare_url",
//       "contents_url",
//       "contributors_url",
//       "deployments_url",
//       "description",
//       "downloads_url",
//       "events_url",
//       "fork",
//       "forks_url",
//       "full_name",
//       "git_commits_url",
//       "git_refs_url",
//       "git_tags_url",
//       "hooks_url",
//       "html_url",
//       "id",
//       "node_id",
//       "issue_comment_url",
//       "issue_events_url",
//       "issues_url",
//       "keys_url",
//       "labels_url",
//       "languages_url",
//       "merges_url",
//       "milestones_url",
//       "name",
//       "notifications_url",
//       "owner",
//       "private",
//       "pulls_url",
//       "releases_url",
//       "stargazers_url",
//       "statuses_url",
//       "subscribers_url",
//       "subscription_url",
//       "tags_url",
//       "teams_url",
//       "trees_url",
//       "url",
//       "clone_url",
//       "default_branch",
//       "forks",
//       "forks_count",
//       "git_url",
//       "has_issues",
//       "has_projects",
//       "has_wiki",
//       "has_pages",
//       "homepage",
//       "language",
//       "archived",
//       "disabled",
//       "mirror_url",
//       "open_issues",
//       "open_issues_count",
//       "license",
//       "pushed_at",
//       "size",
//       "ssh_url",
//       "stargazers_count",
//       "svn_url",
//       "watchers",
//       "watchers_count",
//       "created_at",
//       "updated_at"
//     ]
//   }
// }

fn get_next_url(headers: &HeaderMap) -> Option<&str> {
    let links = headers.get("link")?;
    let text = links.to_str().unwrap();
    let rel_next = text.find("rel=\"next\"")?;
    let text = &text[..rel_next];
    // At this point, by GitHub's documentation, there must be some link behind
    // the rel="next" marker, hence the unwraps.
    let (_, text) = text.rsplit_once('<').unwrap();
    let (text, _) = text.split_once('>').unwrap();
    Some(text)
}

// Note: when unauthenticated, we will have parsing issues.
pub async fn list_user_repos(client: &Client) -> Vec<Value> {
    let url = format!("{}/user/repos", github!());
    let mut vec: Vec<Value> = vec![];
    let mut next_req = Some(client.get(url));
    while let Some(req) = next_req.take() {
        let res = req.send().await.unwrap();
        if let Some(url) = get_next_url(res.headers()) {
            next_req = Some(client.get(url))
        }
        vec.extend(res.json::<Vec<Value>>().await.unwrap());
    }
    vec
}
