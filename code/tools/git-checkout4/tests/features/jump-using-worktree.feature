Feature: Jump using worktree

  When there is no branch match, we jump using the worktree's basename.

  Background:
    Given a repo is init'd at [.] with branch [main]
    And there is a non-trivial commit at [.]
    And a worktree is added at [.] called [.git/develop]
    And there is a non-trivial commit at [.git/develop]
    And a worktree is added at [.] called [.git/release]
    And there is a non-trivial commit at [.git/release]
    And checkout at [.git/release] a new branch [release-1.2]
    And checkout at [.git/release] a new branch [release-1.3]
    And delete at [.git/release] the branch [release]
    Then the current branch at [.git/develop] is [develop]
    And the current branch at [.git/release] is [release-1.3]
    When we run [git checkout4 .] at [.git/release]
    Then stderr is
      """
      Branch release-1.2 is not mapped to any worktree.
      [1] ${{ test.dir }}
      [2] ${{ test.dir }}/.git/develop
      [3] ${{ test.dir }}/.git/release
      Pick one >
      """
    When we feed keys "3"
    Then stderr is
      """
      Branch release-1.3 is not mapped to any worktree.
      [1] ${{ test.dir }}
      [2] ${{ test.dir }}/.git/develop
      [3] ${{ test.dir }}/.git/release
      Pick one >
      """
    When we feed keys "3"
    Then stderr is
      """
      [gco] bypass.
      Updated 0 paths from the index
      """

  Scenario: When the goal exists as a worktree but not a branch, jump to that
    worktree
    When we run [git checkout4 release] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/release
      """
    And at [.] the [release] branch does not exist

  Scenario: When the goal exists as a worktree but not a branch, jump to that
    worktree, even with a detached HEAD.
    Then the current branch at [.] is [main]
    Given at [.] we checkout the current commit
    When we run [git checkout4 release] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/release
      """
    And at [.] the [release] branch does not exist
