Describe 'yak directories'
  BeforeEach 'setup_isolated_repo'
  AfterEach 'teardown_isolated_repo'

  It 'have context.md file by default'
    yx add "test yak"

    When call sh -c "find '$TEST_REPO/.yaks/test yak' -type f -name 'context.md' | wc -l"
    The output should equal "1"
  End
End
