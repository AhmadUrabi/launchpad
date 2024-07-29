-- Your SQL goes here
CREATE TABLE IF NOT EXISTS access_tokens (
    id bigint unsigned NOT NULL PRIMARY KEY AUTO_INCREMENT,
    user_id bigint unsigned NOT NULL,
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
)
