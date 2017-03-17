use diesel::types::Timestamptz;
use schema::{gists, users};

#[derive(Identifiable, Queryable, Associations)]
#[has_many(gists)]
pub struct User {
    /// The Github username of the user.
    pub id: String, // TODO(DarinM223): add other user based statistics here.
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Gist {
    /// The id of the Github gist.
    pub id: String,
    /// The Github username of the user.
    pub user_id: String,
    /// The title of the gist.
    pub title: String,
    /// The text of the gist.
    pub body: String,
    /// The date and time when the gist was created.
    pub created: Timestamptz,
}

#[derive(Insertable)]
#[table_name="gists"]
pub struct NewGist<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: &'a str,
}
