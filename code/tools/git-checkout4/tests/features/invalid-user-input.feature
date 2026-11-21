Feature: Invalid User Input

  These scenarios handle the case when the user enters invalid inputs when
  prompted for the worktree to map a branch to.

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

  Scenario: User keys in a letter
    When we run [git checkout4 release] at [.git/release]
    Then stderr is
      """
      Branch release-1.2 is not mapped to any worktree.
      [1] ${{ test.dir }}
      [2] ${{ test.dir }}/.git/develop
      [3] ${{ test.dir }}/.git/release
      Pick one >
      """
    When we feed keys "a"
    Then stderr is
      """
      Invalid choice for mapping worktree - cannot be parsed as integer
      """

  Scenario: User keys in a number that is out of range
    When we run [git checkout4 release] at [.git/release]
    Then stderr is
      """
      Branch release-1.2 is not mapped to any worktree.
      [1] ${{ test.dir }}
      [2] ${{ test.dir }}/.git/develop
      [3] ${{ test.dir }}/.git/release
      Pick one >
      """
    When we feed keys "100"
    Then stderr is
      """
      Invalid choice for mapping worktree - index out of range
      """
