Feature: Lobby Jumps

  This just tests out a common scenario where we use a bare repo and jump around
  in its sister directories.

  Background:
    Given a bare repo is init'd at [.]
    And a worktree is added at [.] called [main]
    And there is a non-trivial commit at [main]
    And a worktree is added at [.] called [dev]
    And there is a non-trivial commit at [dev]

  Scenario: Checkout branch from the lobby
    When we run [git checkout4 main] at [.]
    Then stderr is
      """
      [gco] jump to worktree
      """
    And stdout is
      """
      ${{ test.dir }}/main
      """
    And at [.] the mapped worktree for branch [main] is [./main]
    And at [.] the mapped worktree for branch [dev] is [./dev]
