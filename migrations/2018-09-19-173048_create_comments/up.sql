CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  page_id INTEGER NOT NULL REFERENCES pages (id),
  name VARCHAR,
  email VARCHAR,
  access_code VARCHAR NOT NULL,
  comment TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
)
