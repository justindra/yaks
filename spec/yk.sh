Describe 'yk'
  It 'shows help when run with --help'
    When run yk --help
    The output should include "Usage:"
    The status should be success
  End

  It 'shows help when run with no arguments'
    When run yk
    The output should include "Usage:"
    The status should be success
  End
End
