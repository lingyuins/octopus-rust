use serde::{Deserialize, Serialize};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "admin".to_string()
}

/// User login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub expire: i32,
}

/// User change password request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChangePassword {
    pub old_password: String,
    pub new_password: String,
}

/// User change username request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChangeUsername {
    pub new_username: String,
}

/// User bootstrap create request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBootstrapCreate {
    pub username: String,
    pub password: String,
}

/// User create request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

/// User login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub token: String,
    pub expire_at: String,
}

/// User info response (without password)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub role: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username,
            role: user.role,
        }
    }
}

impl User {
    /// Hash the password using argon2
    pub fn hash_password(&mut self) -> Result<(), crate::error::AppError> {
        use argon2::{
            password_hash::{PasswordHasher, SaltString},
            Argon2,
        };
        use rand::rngs::OsRng;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(self.password.as_bytes(), &salt)
            .map_err(|e| crate::error::AppError::Internal(format!("failed to hash password: {}", e)))?;

        self.password = hash.to_string();
        Ok(())
    }

    /// Verify password against stored hash
    pub fn verify_password(&self, password: &str) -> Result<bool, crate::error::AppError> {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };

        let parsed_hash = PasswordHash::new(&self.password)
            .map_err(|e| crate::error::AppError::Internal(format!("invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

// User role constants
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_EDITOR: &str = "editor";
pub const ROLE_VIEWER: &str = "viewer";