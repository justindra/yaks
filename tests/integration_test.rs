use std::env;
use tempfile::TempDir;
use yx::ports::StoragePort;

/// Helper to run yx commands in a test environment
struct TestEnv {
    _temp_dir: TempDir,
    yak_path: String,
}

impl TestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let yak_path = temp_dir.path().to_str().unwrap().to_string();

        Self {
            _temp_dir: temp_dir,
            yak_path,
        }
    }

    fn yak_exists(&self, name: &str) -> bool {
        let path = format!("{}/{}", self.yak_path, name);
        std::path::Path::new(&path).exists()
    }
}

#[test]
fn test_add_yak_creates_directory() {
    let test_env = TestEnv::new();

    // Set YAK_PATH for the test
    env::set_var("YAK_PATH", &test_env.yak_path);

    // Create DirectoryStorage and ConsoleOutput
    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Execute AddYak use case
    let use_case = yx::application::AddYak::new(&storage, &output);
    use_case.execute("integration-test-yak").unwrap();

    // Verify the yak directory was created
    assert!(test_env.yak_exists("integration-test-yak"));
}

#[test]
fn test_add_yak_can_be_retrieved() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add a yak
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("test-retrieval").unwrap();

    // Retrieve it using the storage port
    let yak = storage.get_yak("test-retrieval").unwrap();
    assert_eq!(yak.name, "test-retrieval");
    assert!(!yak.done);
}

#[test]
fn test_list_empty_yaks() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // List should succeed even with no yaks
    let list_use_case = yx::application::ListYaks::new(&storage, &output);
    list_use_case.execute().unwrap();
}

#[test]
fn test_list_shows_added_yaks() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add some yaks
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("yak-one").unwrap();
    add_use_case.execute("yak-two").unwrap();

    // List them
    let list_use_case = yx::application::ListYaks::new(&storage, &output);
    list_use_case.execute().unwrap();

    // Verify both yaks exist
    let yaks = storage.list_yaks().unwrap();
    assert_eq!(yaks.len(), 2);
    assert!(yaks.iter().any(|y| y.name == "yak-one"));
    assert!(yaks.iter().any(|y| y.name == "yak-two"));
}

#[test]
fn test_done_yak_marks_as_done() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add a yak
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("test-yak").unwrap();

    // Mark it as done
    let done_use_case = yx::application::DoneYak::new(&storage, &output);
    done_use_case.execute("test-yak", false).unwrap();

    // Verify it's marked as done
    let yak = storage.get_yak("test-yak").unwrap();
    assert!(yak.done);
}

#[test]
fn test_done_yak_with_undo_marks_as_not_done() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add a yak and mark it done
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("test-yak").unwrap();
    let done_use_case = yx::application::DoneYak::new(&storage, &output);
    done_use_case.execute("test-yak", false).unwrap();

    // Verify it's marked as done
    let yak = storage.get_yak("test-yak").unwrap();
    assert!(yak.done);

    // Mark it as not done using undo flag
    done_use_case.execute("test-yak", true).unwrap();

    // Verify it's no longer marked as done
    let yak = storage.get_yak("test-yak").unwrap();
    assert!(!yak.done);
}

#[test]
fn test_done_yak_fails_for_nonexistent_yak() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Try to mark a non-existent yak as done
    let done_use_case = yx::application::DoneYak::new(&storage, &output);
    let result = done_use_case.execute("nonexistent", false);

    assert!(result.is_err());
}

#[test]
fn test_remove_yak_deletes_directory() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add a yak
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("test-yak").unwrap();

    // Verify it exists
    assert!(test_env.yak_exists("test-yak"));

    // Remove it
    let remove_use_case = yx::application::RemoveYak::new(&storage, &output);
    remove_use_case.execute("test-yak").unwrap();

    // Verify it no longer exists
    assert!(!test_env.yak_exists("test-yak"));
}

#[test]
fn test_remove_yak_fails_for_nonexistent_yak() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Try to remove a non-existent yak
    let remove_use_case = yx::application::RemoveYak::new(&storage, &output);
    let result = remove_use_case.execute("nonexistent");

    assert!(result.is_err());
}

#[test]
fn test_remove_yak_can_remove_done_yak() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add a yak and mark it done
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("done-yak").unwrap();
    let done_use_case = yx::application::DoneYak::new(&storage, &output);
    done_use_case.execute("done-yak", false).unwrap();

    // Remove the done yak
    let remove_use_case = yx::application::RemoveYak::new(&storage, &output);
    remove_use_case.execute("done-yak").unwrap();

    // Verify it's gone
    assert!(!test_env.yak_exists("done-yak"));
}

#[test]
fn test_prune_removes_all_done_yaks() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add multiple yaks
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("done-yak-1").unwrap();
    add_use_case.execute("done-yak-2").unwrap();
    add_use_case.execute("active-yak").unwrap();

    // Mark some as done
    let done_use_case = yx::application::DoneYak::new(&storage, &output);
    done_use_case.execute("done-yak-1", false).unwrap();
    done_use_case.execute("done-yak-2", false).unwrap();

    // Prune done yaks
    let prune_use_case = yx::application::PruneYaks::new(&storage, &output);
    prune_use_case.execute().unwrap();

    // Verify done yaks are removed
    assert!(!test_env.yak_exists("done-yak-1"));
    assert!(!test_env.yak_exists("done-yak-2"));

    // Verify active yak still exists
    assert!(test_env.yak_exists("active-yak"));
}

#[test]
fn test_prune_handles_no_done_yaks() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Add only active yaks
    let add_use_case = yx::application::AddYak::new(&storage, &output);
    add_use_case.execute("active-yak-1").unwrap();
    add_use_case.execute("active-yak-2").unwrap();

    // Prune (should handle gracefully)
    let prune_use_case = yx::application::PruneYaks::new(&storage, &output);
    prune_use_case.execute().unwrap();

    // Verify all yaks still exist
    assert!(test_env.yak_exists("active-yak-1"));
    assert!(test_env.yak_exists("active-yak-2"));
}

#[test]
fn test_prune_handles_empty_list() {
    let test_env = TestEnv::new();
    env::set_var("YAK_PATH", &test_env.yak_path);

    let storage = yx::adapters::storage::DirectoryStorage::new().unwrap();
    let output = yx::adapters::cli::ConsoleOutput;

    // Prune when no yaks exist (should handle gracefully)
    let prune_use_case = yx::application::PruneYaks::new(&storage, &output);
    prune_use_case.execute().unwrap();
}
