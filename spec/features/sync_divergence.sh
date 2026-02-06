# shellcheck shell=bash
# shellcheck disable=SC1010,SC2034
Describe 'yx sync - handling divergence scenarios'
  setup_repos() {
    # Create origin repo
    ORIGIN=$(mktemp -d)
    setup_bare_repo "$ORIGIN"

    # Create clone1 repo
    CLONE1=$(mktemp -d)
    setup_test_repo "$CLONE1" "user1@example.com" "User 1" "$ORIGIN"
    echo "# Test Repo" > "$CLONE1/README.md"
    git -C "$CLONE1" add README.md
    git -C "$CLONE1" commit -m "Initial commit" --quiet
    git -C "$CLONE1" push -u origin main --quiet

    # Create clone2 repo (clone of origin)
    CLONE2=$(mktemp -d)
    git clone --quiet "$ORIGIN" "$CLONE2"
    git -C "$CLONE2" config user.email "user2@example.com"
    git -C "$CLONE2" config user.name "User 2"
  }

  cleanup_repos() {
    rm -rf "$ORIGIN" "$CLONE1" "$CLONE2"
  }

  BeforeEach 'setup_repos'
  AfterEach 'cleanup_repos'

  It 'syncs state changes when one clone falls behind'
    # Both clones start with shared yaks
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "shared yak"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs to get the yaks
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Verify both have the yak as todo
    result1=$(sh -c "GIT_WORK_TREE='$CLONE1' yx ls --format markdown")
    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")
    echo "$result1" | grep -q "\[todo\] shared yak" || exit 1
    echo "$result2" | grep -q "\[todo\] shared yak" || exit 1

    # Clone1 changes state to wip and syncs
    GIT_WORK_TREE="$CLONE1" "yx" state "shared yak" wip
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs and should get the state change
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Verify both show wip state
    result1=$(sh -c "GIT_WORK_TREE='$CLONE1' yx ls --format markdown")
    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "[wip] shared yak"
  End

  It 'handles one clone evolving while another stays static'
    # Clone1 creates initial yaks and syncs
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "yak-a"
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "yak-b"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs to get initial state
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone1 evolves: adds new yaks, changes states, removes some
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "yak-c"
    GIT_WORK_TREE="$CLONE1" "yx" state "yak-a" wip
    GIT_WORK_TREE="$CLONE1" "yx" done "yak-b"
    GIT_WORK_TREE="$CLONE1" "yx" rm "yak-b"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 stays at old state, then syncs
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone2 should now have all of Clone1's changes
    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "[wip] yak-a"
    The output should include "[todo] yak-c"
    The output should not include "yak-b"
  End

  It 'preserves local yaks when syncing with remote that has different yaks'
    # Clone1 adds yak-a and syncs
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "yak-a"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs to get yak-a
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone1 adds yak-b and syncs
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "yak-b"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 adds yak-c locally (before syncing)
    echo "" | GIT_WORK_TREE="$CLONE2" "yx" add "yak-c"

    # Clone2 syncs - should keep local yak-c AND get remote yak-b
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "yak-a"
    The output should include "yak-b"
    The output should include "yak-c"
  End

  It 'handles overlapping yak names with last-write-wins at yak level'
    # Clone1 adds yak-a with specific context and syncs
    echo "Clone1 context" | GIT_WORK_TREE="$CLONE1" "yx" add "shared"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs to get yak-a
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone1 modifies and syncs
    GIT_WORK_TREE="$CLONE1" "yx" state "shared" wip
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 modifies locally (different state)
    GIT_WORK_TREE="$CLONE2" "yx" done "shared"

    # Clone2 syncs - local wins (last-write-wins)
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone2 should have its own version (done state)
    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "[done] shared"
  End

  It 'fast-forwards when clone has no local changes'
    # Clone1 creates and evolves yaks
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "test"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs to get initial state
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone1 makes more changes
    GIT_WORK_TREE="$CLONE1" "yx" state "test" wip
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "another"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 has made NO local changes, just syncs (should fast-forward)
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "[wip] test"
    The output should include "another"
  End

  It 'syncs hierarchical yaks correctly'
    # Clone1 creates parent and child yaks
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "parent"
    echo "" | GIT_WORK_TREE="$CLONE1" "yx" add "parent/child"
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    # Clone1 changes parent state
    GIT_WORK_TREE="$CLONE1" "yx" state "parent" wip
    sh -c "cd '$CLONE1' && GIT_WORK_TREE='$CLONE1' yx sync" 2>&1

    # Clone2 syncs and should get parent state change
    sh -c "cd '$CLONE2' && GIT_WORK_TREE='$CLONE2' yx sync" 2>&1

    result2=$(sh -c "GIT_WORK_TREE='$CLONE2' yx ls --format markdown")

    When call echo "$result2"
    The output should include "[wip] parent"
    The output should include "child"
  End
End
