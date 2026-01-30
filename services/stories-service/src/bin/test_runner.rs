/// Test runner binary for Islamic Stories Service
/// This runs unit tests for Requirements 4.4 and 4.5 without database dependencies

use stories_service::test_runner;

fn main() {
    test_runner::run_all_tests();
}