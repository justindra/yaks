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
