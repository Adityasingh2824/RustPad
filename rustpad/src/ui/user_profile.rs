use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub email: Option<String>,
    pub theme: String,  // Dark mode, light mode, etc.
    // Add other preferences here
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>, theme: String) -> Self {
        UserProfile { username, email, theme }
    }

    pub fn update(&mut self, username: Option<String>, email: Option<String>, theme: Option<String>) {
        if let Some(new_username) = username {
            self.username = new_username;
        }
        if let Some(new_email) = email {
            self.email = new_email;
        }
        if let Some(new_theme) = theme {
            self.theme = new_theme;
        }
    }
}

// Shared state to manage user profiles
type UserProfileStore = Arc<Mutex<UserProfile>>;

/// Get the current user profile
pub async fn get_user_profile(profile_store: UserProfileStore) -> Result<impl Reply, Rejection> {
    let profile = profile_store.lock().unwrap();
    Ok(warp::reply::json(&*profile))
}

/// Update the user profile
pub async fn update_user_profile(
    profile_store: UserProfileStore,
    updated_profile: UserProfile,
) -> Result<impl Reply, Rejection> {
    let mut profile = profile_store.lock().unwrap();
    profile.update(Some(updated_profile.username), updated_profile.email, Some(updated_profile.theme));
    Ok(warp::reply::json(&"Profile updated successfully"))
}

/// User Profile UI
pub fn user_profile_ui(profile_store: UserProfileStore) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("profile")
        .and(warp::get())
        .and(warp::any().map(move || profile_store.clone()))
        .and_then(get_user_profile)
        .or(
            warp::path("profile")
                .and(warp::put())
                .and(warp::body::json())
                .and(warp::any().map(move || profile_store.clone()))
                .and_then(update_user_profile)
        )
}
