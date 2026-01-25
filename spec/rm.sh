Describe 'yx rm'
  BeforeEach 'export YAK_PATH=$(mktemp -d)'
  AfterEach 'rm -rf "$YAK_PATH"'

  It 'removes a yak by name'
    When run sh -c 'yx add "Fix the bug" && yx add "Write docs" && yx rm "Fix the bug" && yx list'
    The output should include "- [ ] Write docs"
    The output should not include "- [ ] Fix the bug"
  End

  It 'shows error when yak not found'
    When run yx rm "Nonexistent yak"
    The status should be failure
    The error should include "not found"
  End

  It 'handles removing the only yak'
    When run sh -c 'yx add "Only yak" && yx rm "Only yak" && yx list'
    The output should equal "You have no yaks. Are you done?"
  End
End
