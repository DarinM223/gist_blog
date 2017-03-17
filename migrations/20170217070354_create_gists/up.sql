CREATE TABLE gists (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR REFERENCES users(id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created TIMESTAMP WITH TIME ZONE
)
