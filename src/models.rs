use schema::{gists, users};

#[derive(Debug, Identifiable, Queryable, Associations)]
#[has_many(gists)]
pub struct User {
    /// The Github username of the user.
    pub id: String, // TODO(DarinM223): add other user based statistics here.
}

#[derive(Debug, Identifiable, Queryable, Associations, Serialize)]
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
}

#[derive(Insertable)]
#[table_name = "gists"]
pub struct NewGist<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> NewGist<'a> {
    pub fn from<'b>(gist: &'b Gist) -> NewGist<'b> {
        NewGist {
            id: gist.id.as_str(),
            user_id: gist.user_id.as_str(),
            title: gist.title.as_str(),
            body: gist.body.as_str(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
}
