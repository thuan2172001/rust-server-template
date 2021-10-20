use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    // models::person::{Person},
    models::user::{LoginDTO, User, UserDTO, UserEdit},
    models::user_token::UserToken,
    models::login_history::LoginHistory,
    utils::token_utils,
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &pool.get().unwrap()) {
        Some(logged_user) => {
            match serde_json::from_value(
                json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer" }),
            ) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::new(
                            StatusCode::UNAUTHORIZED,
                            constants::MESSAGE_LOGIN_FAILED.to_string(),
                        ))
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                )),
            }
        }
        None => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            constants::MESSAGE_USER_NOT_FOUND.to_string(),
        )),
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
                        User::logout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    ))
}

pub fn get_user(pool: &web::Data<Pool>) -> Result<Vec<User>, ServiceError> {
    match User::find_all(&pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn get_user_by_id(id:i32, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
    match User::find_by_id(id, &pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn update(id: i32, updated_user: UserEdit, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match User::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match User::update(id, updated_user, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("User with id {} not found", id))),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match User::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match LoginHistory::delete_user(id, &pool.get().unwrap()) {
            Ok(_) => match User::delete(id, &pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
            },
            Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("User with id {} not found", id))),
        }
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Something went wrong with userId:  {}", id))),
    }
}
