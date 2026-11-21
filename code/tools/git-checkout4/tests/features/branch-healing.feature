Feature: Branch Healing

  When there is a branch that is mapped to a particular worktree directory, then
  checking it out using git-checkout4, will first jump to that worktree
  directory, and then ensure that it is checked out.

  Background:
    Given a repo is init'd at [.] with branch [main]
    And there is a non-trivial commit at [.]
    And a worktree is added at [.] called [.git/develop]
    And there is a non-trivial commit at [.git/develop]
    And a worktree is added at [.] called [.git/release]
    And there is a non-trivial commit at [.git/release]
    And checkout at [.git/release] a new branch [release-1.2]
    And checkout at [.git/release] a new branch [release-1.3]
    And at [.git/release] we checkout [release]
    Then the current branch at [.git/develop] is [develop]
    And the current branch at [.git/release] is [release]
    When we run [git checkout4 release] at [.git/release]
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
      Already on 'release'
      """

  Scenario: Checkout mapped branch from the primary repository
    Then the current branch at [.git/release] is [release]
    When we run [git checkout4 release-1.2] at [.]
    Then stderr is
      """
      [gco] jump to worktree, and checkout release-1.2.
      """

  Scenario: Checkout mapped branch from the git directory
    Then the current branch at [.git/release] is [release]
    When we run [git checkout4 release-1.2] at [.git]
    Then stderr is
      """
      [gco] jump to worktree, and checkout release-1.2.
      """

  Scenario: Checkout mapped branch from the mapped worktree directory
    Then the current branch at [.git/release] is [release]
    When we run [git checkout4 release-1.2] at [.git/release]
    Then stderr is
      """
      [gco] bypass.
      Switched to branch 'release-1.2'
      """

  Scenario: Checkout mapped branch from the primary repository, and the mapped
    branch is already checked out
    Given at [.git/release] we checkout [release-1.2]
    Then the current branch at [.git/release] is [release-1.2]
    When we run [git checkout4 release-1.2] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """

  Scenario: Checkout mapped branch from the git directory, and the mapped branch
    is already checked out
    Given at [.git/release] we checkout [release-1.2]
    Then the current branch at [.git/release] is [release-1.2]
    When we run [git checkout4 release-1.2] at [.git]
    Then stderr is
      """
      [gco] jump to worktree
      """

  Scenario: Checkout mapped branch from the mapped worktree directory, and the
    mapped branch is already checked out
    Given at [.git/release] we checkout [release-1.2]
    Then the current branch at [.git/release] is [release-1.2]
    When we run [git checkout4 release-1.2] at [.git/release]
    Then stderr is
      """
      [gco] bypass.
      Already on 'release-1.2'
      """

  Scenario: Checkout invalid branch from the primary repository
    Then the current branch at [.git/release] is [release]
    When we run [git checkout4 zero] at [.]
    Then stderr is
      """
      [gco] bypass.
      error: pathspec 'zero' did not match any file(s) known to git
      """

