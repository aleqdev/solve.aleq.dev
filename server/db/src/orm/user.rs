use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;


#[derive(Debug, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub salt: Vec<u8>,
    pub password_hash: Vec<u8>
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub salt: &'a [u8],
  pub password_hash: &'a [u8]
}

impl User {
  pub async fn create(conn: &mut AsyncPgConnection, username: &str, salt: &[u8], password_hash: &[u8]) -> QueryResult<Self> {
    use crate::schema::user::dsl;

    let new_user = NewUser { username, salt, password_hash };

    let query = diesel::insert_into(dsl::user)
        .values(&new_user)
        .returning(Self::as_returning());

    diesel_async::RunQueryDsl::get_result(query, conn).await
  }

  pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<usize> {
    use crate::schema::user::dsl;
  
    let query = diesel::delete(dsl::user.filter(dsl::id.eq(id)));

    diesel_async::RunQueryDsl::execute(query, conn).await
  }

  pub async fn get(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Option<Self>> {
    use crate::schema::user::dsl;
  
    let query = dsl::user.filter(dsl::id.eq(id));

    diesel_async::RunQueryDsl::get_result(query, conn).await.optional()
  }

  pub async fn get_by_username(conn: &mut AsyncPgConnection, username: &str) -> QueryResult<Option<Self>> {
    use crate::schema::user::dsl;
  
    let query = dsl::user.filter(dsl::username.eq(username));

    diesel_async::RunQueryDsl::get_result(query, conn).await.optional()
  }

  pub async fn exists_with_username(conn: &mut AsyncPgConnection, username: &str) -> QueryResult<bool> {
    use crate::schema::user::dsl;
  
    let query = diesel::dsl::select(diesel::dsl::exists(dsl::user.filter(dsl::username.eq(username))));

    diesel_async::RunQueryDsl::get_result(query, conn).await
  }
}