use diesel::prelude::*;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::record::IntoNewRecord;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    /// The Clerk ID of the user, used as the primary identifier for this user entity.
    pub id: String,
    /// The username of the user, which must be unique.
    pub username: String,
    /// An optional first name for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// An optional last name for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Unix timestamp marking the creation of the user account.
    pub created_at: Timestamp,
    /// Unix timestamp marking the last update to the user account.
    pub updated_at: Timestamp,
}

#[derive(Serialize, Deserialize, Validate, Clone, Default)]
pub struct NewUser {
    /// The username to give to the user. It must be unique across your instance.
    pub username: String,
    /// The first name to assign to the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name to assign to the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The plaintext password to give the user. Must be at least 8 characters long, and can not be in any list of hacked passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Timestamp of when the user was created
    pub created_at: Option<Timestamp>,
}

/// User update input type
#[derive(Serialize, Deserialize, Validate, Clone, Default)]
pub struct UserUpdate {
    /// The username to give to the user. It must be unique across your instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The first name to assign to the user
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub first_name: Option<Option<String>>,
    /// The last name to assign to the user
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub last_name: Option<Option<String>>,
}

#[derive(Identifiable, Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRecord {
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UserUpdateRecord {
    pub username: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<Option<String>>,
    pub updated_at: Option<i64>,
}

// In general, this is going to be called when retrieving items from the DB,
// so we know they should be valid if they were inserted successfully,
// so panicking is the right approach here, avoiding the need to handle errors
// in the caller and enforcing the invariant of valid inserts.
impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        let created_at = Timestamp::from_millisecond(record.created_at).expect("Invalid timestamp");
        let updated_at = Timestamp::from_millisecond(record.updated_at).expect("Invalid timestamp");

        User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            updated_at,
            created_at,
        }
    }
}

impl IntoNewRecord for NewUser {
    type Record = UserRecord;

    fn into_new_record(self) -> UserRecord {
        let current_time = Timestamp::now();

        UserRecord {
            id: format!("dummy-user-{current_time}"),
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,

            created_at: current_time.as_millisecond(),
            updated_at: current_time.as_millisecond(),
        }
    }
}

impl From<UserUpdate> for UserUpdateRecord {
    fn from(user_update: UserUpdate) -> Self {
        let current_time = Timestamp::now();

        UserUpdateRecord {
            username: user_update.username,
            first_name: user_update.first_name,
            last_name: user_update.last_name,
            updated_at: Some(current_time.as_millisecond()),
        }
    }
}
