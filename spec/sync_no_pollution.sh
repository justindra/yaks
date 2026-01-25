Describe 'yx sync does not pollute working tree or index'
  It 'does not add files to git index'
    ORIGIN=$(mktemp -d)
    REPO=$(mktemp -d)
    YX_BIN="$(pwd)/bin/yx"

    # Set up bare origin and clone
    git -C "$ORIGIN" init --bare --quiet
    git -C "$REPO" init --quiet
    git -C "$REPO" remote add origin "$ORIGIN"
    git -C "$REPO" config user.email "test@example.com"
    git -C "$REPO" config user.name "Test"
    echo "test" > "$REPO/README.md"
    git -C "$REPO" add README.md
    git -C "$REPO" commit -m "init" --quiet
    git -C "$REPO" push -u origin main --quiet

    # Add a yak and sync
    YAK_PATH="$REPO/.yaks" "$YX_BIN" add "test yak"
    cd "$REPO"
    YAK_PATH="$REPO/.yaks" "$YX_BIN" sync 2>&1

    # Check that git status is clean (no staged or unstaged changes)
    When call git -C "$REPO" status --porcelain
    The output should equal ""
    The status should be success

    rm -rf "$ORIGIN" "$REPO"
  End

  It 'does not leave files in working directory'
    ORIGIN=$(mktemp -d)
    REPO=$(mktemp -d)
    YX_BIN="$(pwd)/bin/yx"

    # Set up bare origin and clone
    git -C "$ORIGIN" init --bare --quiet
    git -C "$REPO" init --quiet
    git -C "$REPO" remote add origin "$ORIGIN"
    git -C "$REPO" config user.email "test@example.com"
    git -C "$REPO" config user.name "Test"
    echo "test" > "$REPO/README.md"
    git -C "$REPO" add README.md
    git -C "$REPO" commit -m "init" --quiet
    git -C "$REPO" push -u origin main --quiet

    # Add a yak and sync
    YAK_PATH="$REPO/.yaks" "$YX_BIN" add "test yak"
    cd "$REPO"
    YAK_PATH="$REPO/.yaks" "$YX_BIN" sync 2>&1

    # Check that only .yaks and README.md exist
    When call sh -c "ls -A '$REPO' | grep -v '^\.git$' | sort"
    The output should equal ".yaks
README.md"

    rm -rf "$ORIGIN" "$REPO"
  End
End
