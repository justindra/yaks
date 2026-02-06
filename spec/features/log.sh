# shellcheck shell=bash
Describe 'yx log'
  BeforeEach 'setup_isolated_repo'
  AfterEach 'teardown_isolated_repo'

  It 'shows empty log when no events exist'
    When run yx log
    The output should equal ""
    The status should be success
  End

  It 'displays add events'
    When run sh -c "
      yx add 'test yak'
      yx log
    "
    The output should include "add test yak"
    The status should be success
  End

  It 'displays events in chronological order'
    When run sh -c "
      yx add 'first yak'
      yx add 'second yak'
      yx log
    "
    The line 1 of output should include "add first yak"
    The line 2 of output should include "add second yak"
    The status should be success
  End

  It 'displays done events'
    When run sh -c "
      yx add 'test yak'
      yx done 'test yak'
      yx log
    "
    The line 1 of output should include "add test yak"
    The line 2 of output should include "done test yak"
    The status should be success
  End

  It 'displays done --undo events'
    When run sh -c "
      yx add 'test yak'
      yx done 'test yak'
      yx done --undo 'test yak'
      yx log
    "
    The line 1 of output should include "add test yak"
    The line 2 of output should include "done test yak"
    The line 3 of output should include "done --undo test yak"
    The status should be success
  End

  It 'displays remove events'
    When run sh -c "
      yx add 'test yak'
      yx rm 'test yak'
      yx log
    "
    The line 1 of output should include "add test yak"
    The line 2 of output should include "rm test yak"
    The status should be success
  End

  It 'displays context events'
    When run sh -c "
      unset YX_IGNORE_STDIN
      yx add 'test yak'
      echo 'Some context' | yx context 'test yak'
      yx log
    "
    The line 1 of output should include "add test yak"
    The line 2 of output should include "context test yak"
    The status should be success
  End

  It 'includes timestamp in output'
    When run sh -c "
      yx add 'test yak'
      yx log
    "
    # Check for timestamp format YYYY-MM-DD HH:MM:SS
    The output should match pattern "20*-*-* *:*:* * add test yak"
    The status should be success
  End

  It 'includes author in output'
    When run sh -c "
      yx add 'test yak'
      yx log
    "
    # Check that author is present (test@example.com from setup)
    The output should include "test@example.com"
    The status should be success
  End
End
