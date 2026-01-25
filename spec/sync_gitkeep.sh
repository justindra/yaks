Describe 'empty yak directories'
  BeforeEach 'export YAK_PATH=$(mktemp -d)'
  AfterEach 'rm -rf "$YAK_PATH"'

  It 'have no files initially'
    yx add "empty yak"

    When call sh -c "find '$YAK_PATH/empty yak' -type f | wc -l"
    The output should equal "0"
  End
End
