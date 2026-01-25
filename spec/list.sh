Describe 'yk list'
  BeforeEach 'export YAK_PATH=$(mktemp -d)'
  AfterEach 'rm -rf "$YAK_PATH"'

  It 'shows message when no yaks exist'
    When run yk list
    The output should equal 'You have no yaks. Are you done?'
  End

  It 'lists added yaks'
    When run sh -c 'yk add "Fix the bug" && yk list'
    The output should equal "Fix the bug"
  End
End
