// ssh_auth_cargo_publish/src/crates_mod.rs

use crate::secrets_always_local_mod::crates_io_mod;
use crate::RED;
use crate::RESET;

/// publish to crates.io retrieving the secret_token from encrypted file
///
/// If the secret_token is not stored ask user to interactively input the secret_token, encrypt and store.
pub fn publish(registry: &str) {
    if registry != "crates.io" {
        panic!("{RED}Error: The argument 'registry' can only be `crates.io`.{RESET}")
    }
    let crates_io_client = crates_io_mod::CratesIoClient::new_with_stored_secret_token();
    crates_io_client.publish_to_crates_io();
}
