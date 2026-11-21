Feature: Special Scenarios

  Special, compound scenarios that occur in daily use.

  Scenario: Work #1
    Given a repo is init'd at [google/xz] with branch [main]
    And there is a non-trivial commit on file [./lib/xz.c] at [google/xz]

    Given a repo is init'd at [delta] with branch [main]
    And a submodule is added at [delta] with repo [./google/xz] and directory [deps/xz]
    And we commit at [delta] with "Add submodule"
    And we create a .git file at [delta/workspace/xz] with gitdir = [./google/xz/.git]
    Then the git dir of [delta/workspace/xz] is [./google/xz/.git]
    Given a worktree is added at [delta/workspace/xz] called [dev]
    When we run [git checkout4 dev] at [delta/workspace/xz]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/delta/workspace/xz/dev
      """
