Feature: Primary Git Repository

  These scenarios cover the case where we operate in the most normal state of a
  git repository, where there is a ".git" directory that contains the
  repository, and there is a worktree checked out at the current directory.

  Background:
    Given a repo is init'd at [.] with branch [main]

  Scenario: Checking out another branches checks it out
    Given checkout at [.] a new branch [dev]
    And there is a non-trivial commit at [.]
    Then the current branch at [.] is [dev]
    And running [git-checkout4 main] at [.] results in a bypass
    And at [.] the mapped worktree for branch [dev] is [.]
    And at [.] there is no mapped worktree for branch [main]
