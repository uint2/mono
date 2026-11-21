Feature: Primary Git Repository

  These scenarios cover the case where we operate in the most normal state of a
  git repository, where there is a ".git" directory that contains the
  repository, and there is a worktree checked out at the current directory.

  Background:
    Given a repo is init'd at [.] with branch [main]
    And there is a non-trivial commit at [.]

  Scenario: Checking out another branch checks it out
    Given checkout at [.] a new branch [dev]
    And there is a non-trivial commit at [.]
    And at [.] we checkout [main]
    Then the current branch at [.] is [main]
    When we run [git checkout4 dev] at [.]
    Then stderr is
      """
      Branch dev is not mapped to any worktree.
      [1] ${{ test.dir }}
      Pick one >
      """
    When we feed keys "1" 
    Then stderr is
      """
      [gco] bypass.
      Switched to branch 'dev'
      """
    And the current branch at [.] is [dev]
    And at [.] the mapped worktree for branch [main] is [.]
    And at [.] the mapped worktree for branch [dev] is [.]

  Scenario: Checking out the default branch checks it out
    Given checkout at [.] a new branch [dev]
    And there is a non-trivial commit at [.]
    Then the current branch at [.] is [dev]
    When we run [git checkout4 main] at [.]
    Then stderr is
      """
      Branch main is not mapped to any worktree.
      [1] ${{ test.dir }}
      Pick one >
      """
    When we feed keys "1" 
    Then stderr is
      """
      [gco] bypass.
      Switched to branch 'main'
      """
    And the current branch at [.] is [main]
    And at [.] the mapped worktree for branch [main] is [.]
    And at [.] the mapped worktree for branch [dev] is [.]

  Scenario: Checking out a branch from a detached HEAD checks it out
    Given there is a non-trivial commit at [.]
    And at [.] we checkout the current commit
    When we run [git status] at [.]
    Then stdout is
      """
      HEAD detached at ${{ git.HEAD.short }}
      nothing to commit, working tree clean
      """
    When we run [git checkout4 main] at [.]
    Then stderr is
      """
      Branch main is not mapped to any worktree.
      [1] ${{ test.dir }}
      Pick one >
      """
    When we feed keys "1" 
    Then stderr is
      """
      [gco] bypass.
      Switched to branch 'main'
      """
    And the current branch at [.] is [main]
    And at [.] the mapped worktree for branch [main] is [.]

  Scenario: Checking out a branch in the git directory checks it out
    Given checkout at [.] a new branch [dev]
    And there is a non-trivial commit at [.]
    And at [.] we checkout [main]
    And a worktree is added at [./.git] called [dev]
    Then the current branch at [.] is [main]
    And the current branch at [./.git/dev] is [dev]
    When we run [git checkout4 dev] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/dev/
      """

  Scenario: Checking out a branch in the git directory checks it out
    Given checkout at [.] a new branch [dev]
    And there is a non-trivial commit at [.]
    And at [.] we checkout [main]
    And a worktree is added at [./.git] called [dev]
    Then the current branch at [.] is [main]
    And the current branch at [./.git/dev] is [dev]
    When we run [git checkout4 dev] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/dev/
      """
