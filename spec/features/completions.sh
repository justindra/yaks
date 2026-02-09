# shellcheck shell=bash
# shellcheck disable=SC1010
Describe 'Completions'
  BeforeEach 'setup_isolated_repo'
  AfterEach 'teardown_isolated_repo'

  Describe 'All commands have completions'
    It 'includes all main commands in bash completions'
      When run grep -E "^[[:space:]]*COMPREPLY.*compgen.*add list ls done finish remove rm move mv prune context sync" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "add list ls done finish remove rm move mv prune context sync"
    End

    It 'includes done alias (finish) in bash completions case statement'
      When run grep -E "done\|finish" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "done|finish"
    End

    It 'includes remove command in bash completions case statement'
      When run grep -E "remove\|rm" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "remove|rm"
    End

    It 'includes sync command in bash completions'
      When run grep "sync" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "sync"
    End

    It 'includes sync command in zsh completions'
      When run grep "'sync:Sync yaks with git refs'" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "sync:Sync yaks with git refs"
    End

    It 'includes finish alias in zsh completions'
      When run grep "'finish:Mark a yak as done (alias)'" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "finish:Mark a yak as done"
    End

    It 'includes remove command in zsh completions'
      When run grep "'remove:Remove a yak'" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "remove:Remove a yak"
    End

    It 'includes done alias (finish) in zsh case statement'
      When run grep -E "done\|finish" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "done|finish"
    End

    It 'includes remove command in zsh case statement'
      When run grep -E "remove\|rm" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "remove|rm"
    End
  End

  Describe 'Flags are supported'
    It 'includes --recursive flag for done command in bash'
      When run grep "recursive" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "--recursive"
    End

    It 'includes --recursive flag for done command in zsh'
      When run grep "recursive" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "--recursive"
    End

    It 'includes --undo flag for done command in bash'
      When run grep "undo" "$TEST_PROJECT_DIR/completions/yx.bash"
      The status should be success
      The output should include "--undo"
    End

    It 'includes --undo flag for done command in zsh'
      When run grep "undo" "$TEST_PROJECT_DIR/completions/yx.zsh"
      The status should be success
      The output should include "--undo"
    End
  End
End
