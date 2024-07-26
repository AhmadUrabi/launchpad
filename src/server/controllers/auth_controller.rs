use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    name: String,
    email: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(id: String, name: String, email: String, duration: u64) -> Self {
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
}
