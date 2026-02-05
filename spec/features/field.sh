# shellcheck shell=bash
Describe 'yx field'
  BeforeEach 'setup_isolated_repo'
  AfterEach 'teardown_isolated_repo'

  It 'writes a field from stdin'
    When run sh -c "
      yx add 'my yak'
      echo 'field content' | yx field 'my yak' notes
      yx field 'my yak' notes --show
    "
    The output should equal "my yak

field content"
  End

  It 'reads a field with --show'
    When run sh -c "
      yx add 'my yak'
      echo 'important info' | yx field 'my yak' priority
      yx field 'my yak' priority --show
    "
    The output should equal "my yak

important info"
  End

  It 'shows error for nonexistent yak'
    When run sh -c "
      echo 'content' | yx field 'nonexistent' notes
    "
    The status should be failure
    The error should include "Error: yak 'nonexistent' not found"
  End

  It 'shows error for invalid field name with slash'
    When run sh -c "
      yx add 'my yak'
      echo 'content' | yx field 'my yak' 'invalid/name'
    "
    The status should be failure
    The error should include "Error: Invalid field name 'invalid/name'"
  End

  It 'shows error for reserved field name context.md'
    When run sh -c "
      yx add 'my yak'
      echo 'content' | yx field 'my yak' context.md
    "
    The status should be failure
    The error should include "Error: Field name 'context.md' is reserved"
  End

  It 'shows error for reserved field name state'
    When run sh -c "
      yx add 'my yak'
      echo 'content' | yx field 'my yak' state
    "
    The status should be failure
    The error should include "Error: Field name 'state' is reserved"
  End

  It 'works with nested yaks'
    When run sh -c "
      yx add 'parent'
      yx add 'parent/child'
      echo 'child notes' | yx field 'parent/child' notes
      yx field 'parent/child' notes --show
    "
    The output should equal "parent/child

child notes"
  End

  It 'allows field names with dots'
    When run sh -c "
      yx add 'my yak'
      echo 'text file content' | yx field 'my yak' notes.txt
      yx field 'my yak' notes.txt --show
    "
    The output should equal "my yak

text file content"
  End

  It 'replaces existing field content'
    When run sh -c "
      yx add 'my yak'
      echo 'old content' | yx field 'my yak' notes
      echo 'new content' | yx field 'my yak' notes
      yx field 'my yak' notes --show
    "
    The output should equal "my yak

new content"
  End

  It 'shows error when reading nonexistent field'
    When run sh -c "
      yx add 'my yak'
      yx field 'my yak' nonexistent --show
    "
    The status should be failure
    The error should include "Error: Failed to read field 'nonexistent' for 'my yak'"
  End
End
