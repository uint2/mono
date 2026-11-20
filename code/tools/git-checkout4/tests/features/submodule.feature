Feature: Git Submodule

  These scenarios cover the case where we operate in a git submodule.

  Scenario: Bypass if we detect we're in a submodule
    Given a repo is init'd at [./internet/xz] with branch [main]
    And there is a non-trivial commit at [./internet/xz]
    And a repo is init'd at [./local] with branch [main]
    And a submodule is added at [./local] with repo [./internet/xz/.git] and directory [xz]
    Then the file [.gitmodules] exists at [./local]
    And running [git-checkout4 main] at [./local/xz] results in a bypass
