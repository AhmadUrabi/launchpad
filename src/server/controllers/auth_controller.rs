use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: usize,
    name: String,
    email: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(id: usize, name: String, email: String, duration: u64) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = iat + (duration * 3600);

        // Convert iat to usize
        let iat = iat as usize;
        let exp = exp as usize;

        Self {
            id,
            name,
            email,
            iat,
            exp,
        }
    }

    pub fn generate_from_user(user: User) -> Self {
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // TODO: Add expiration time for eact user
        let exp = iat + (24 * 3600);
        Self {
            id: user.id as usize,
            name: user.name,
            email: user.email,
            iat,
            exp,
        }
    }
}
