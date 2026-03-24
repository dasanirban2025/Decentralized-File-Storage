#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct FileData {
    pub owner: Address,
    pub file_hash: String,
    pub file_name: String,
    pub timestamp: u64,
}

#[contract]
pub struct FileStorageContract;

#[contractimpl]
impl FileStorageContract {

    // Store file metadata
    pub fn store_file(
        env: Env,
        user: Address,
        file_id: Symbol,
        file_hash: String,
        file_name: String,
    ) {
        user.require_auth();

        let data = FileData {
            owner: user.clone(),
            file_hash,
            file_name,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&file_id, &data);
    }

    // Retrieve file metadata
    pub fn get_file(env: Env, file_id: Symbol) -> Option<FileData> {
        env.storage().instance().get(&file_id)
    }

    // Delete file (only owner can delete)
    pub fn delete_file(env: Env, user: Address, file_id: Symbol) {
        user.require_auth();

        let stored: Option<FileData> = env.storage().instance().get(&file_id);

        if let Some(file) = stored {
            if file.owner == user {
                env.storage().instance().remove(&file_id);
            }
        }
    }
}