use diesel::prelude::*;

use crate::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_credentials(
    user: models::LoginUser,
    conn: &MysqlConnection,
) -> Result<Option<String>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .select(nome)
        .filter(login.eq(user.login))
        .filter(psswd.eq(user.psswd))
        .first::<String>(conn)
        .optional()?;
    
    
    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    nm: &str,
    login_nm: &str,
    psswd_nm: &str,
    conn: &MysqlConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: 0,
        nome: nm.to_owned(),
        login: login_nm.to_owned(),
        psswd: psswd_nm.to_owned(),
        date_inserted: None,
        date_updated: None,

    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}