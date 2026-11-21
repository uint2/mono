Feature: Relative Directory Preserving

  When we change directory as a result of git-checkout4's automatic worktree
  jumping, we would also like to preserve the relative path of the CWD relative
  to the root of the worktree.

  Background:
    Given a repo is init'd at [.] with branch [main]
    And there is a non-trivial commit at [.]
    And a worktree is added at [.] called [.git/develop]
    And there is a non-trivial commit on file [src/main/java/com/google/maps/Main.java] at [.git/develop]
    And a worktree is added at [.] called [.git/release]
    And there is a non-trivial commit on file [src/main/java/com/github/actions/Main.java] at [.git/release]
    Then the current branch at [.git/develop] is [develop]
    And the current branch at [.git/release] is [release]
    And at [.] there is no mapped worktree for branch [main]
    And at [.] there is no mapped worktree for branch [develop]
    And at [.] there is no mapped worktree for branch [release]
    When we run [git checkout4 .] at [.]
    Then stderr is
      """
      [gco] bypass.
      Updated 0 paths from the index
      """
    And at [.] the mapped worktree for branch [main] is [.]
    And at [.] the mapped worktree for branch [release] is [.git/release]
    And at [.] the mapped worktree for branch [develop] is [.git/develop]


  Scenario: Relative directory is preserved if possible
    When we run [git checkout4 release] at [.git/develop/src/main/java]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/release/src/main/java
      """

  Scenario: Relative directory is best-preserved
    When we run [git checkout4 release] at [.git/develop/src/main/java/com/google/maps]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/.git/release/src/main/java/com
      """
