Feature: Git Baseline

  Check that git's outputs are as expected since gco-4 is basically a
  string-parsing project.

  Scenario: Git's porcelain output for worktrees is as expected
    Given a repo is init'd at [.] with branch [main]
    And there is an empty commit at [.]
    Then the output of listing worktrees matches regex
      """
      worktree [A-Za-z0-9/:_-]+
      HEAD [a-f0-9]{40}
      branch refs/heads/main
      """

  Scenario: Git's porcelain output for worktrees is as expected in a bare repo
    Given a bare repo is init'd at [.]
    And there is an orphan worktree at [./main]
    And there is an empty commit at [./main]
    Then the output of listing worktrees matches regex
      """
      worktree [A-Za-z0-9/:_-]+
      bare

      worktree [A-Za-z0-9/:_-]+/main
      HEAD [a-f0-9]{40}
      branch refs/heads/main
      """
