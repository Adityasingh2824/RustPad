use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply, http::header::SET_COOKIE};
use uuid::Uuid;
use warp::http::HeaderValue;
use warp::reply::Response;

pub type Sessions = Arc<Mutex<HashMap<String, UserSession>>>;

/// Struct representing a user session.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub user_id: String,
    pub is_authenticated: bool,
}

impl UserSession {
    /// Creates a new user session.
    pub fn new(user_id: String) -> Self {
        UserSession {
            user_id,
            is_authenticated: true,
        }
    }
}

/// Generates a unique session ID using UUID.
fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

/// Verifies if a session exists in the session store.
pub async fn verify_session(sessions: &Sessions, session_id: &str) -> bool {
    let sessions = sessions.lock().unwrap();
    sessions.contains_key(session_id)
}

/// Filter to ensure a session exists, creating one if needed.
pub fn with_session(
    session_store: Sessions,
) -> impl Filter<Extract = (UserSession,), Error = Rejection> + Clone {
    warp::cookie::optional("session_id")
        .and(warp::any().map(move || session_store.clone()))
        .and_then(
            |session_id: Option<String>, session_store: Sessions| async move {
                let session_id = session_id.unwrap_or_else(generate_session_id);

                let mut sessions = session_store.lock().unwrap();

                // Retrieve existing session or create a new one.
                let session = sessions
                    .entry(session_id.clone())
                    .or_insert_with(|| UserSession::new("guest".to_string()))
                    .clone();

                Ok::<_, Rejection>(session)
            },
        )
}

/// Creates a new session for a user and sets a session ID cookie.
pub async fn create_session(
    user_id: String,
    session_store: Sessions,
) -> Result<impl Reply, Rejection> {
    let session_id = generate_session_id();

    // Create a new session with the provided user ID.
    let new_session = UserSession::new(user_id);

    // Store the session in the session store.
    session_store.lock().unwrap().insert(session_id.clone(), new_session);

    // Create a session cookie for the response.
    let cookie = HeaderValue::from_str(&format!("session_id={}; Path=/; HttpOnly", session_id))
        .expect("Failed to create cookie header value");

    // Prepare the response and include the session cookie.
    let mut response = warp::reply::json(&"Session Created").into_response();
    response.headers_mut().insert(SET_COOKIE, cookie);

    Ok(response)
}

/// Retrieves an existing session based on the session ID cookie.
pub fn get_session(
    session_store: Sessions,
) -> impl Filter<Extract = (Option<UserSession>,), Error = Rejection> + Clone {
    warp::cookie::optional("session_id")
        .and(warp::any().map(move || session_store.clone()))
        .and_then(|session_id: Option<String>, session_store: Sessions| async move {
            let sessions = session_store.lock().unwrap();
            let session = session_id.and_then(|id| sessions.get(&id).cloned());

            Ok::<_, Rejection>(session)
        })
}

/// Invalidates a session, removing it from the session store.
pub async fn invalidate_session(
    session_id: String,
    session_store: Sessions,
) -> Result<impl Reply, Rejection> {
    // Remove the session from the store.
    let mut sessions = session_store.lock().unwrap();
    sessions.remove(&session_id);

    let reply = warp::reply::json(&"Session Invalidated");
    Ok(reply)
}
