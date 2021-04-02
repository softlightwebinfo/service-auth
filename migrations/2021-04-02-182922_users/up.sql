-- Your SQL goes here
CREATE TABLE users
(
    id       serial       NOT NULL PRIMARY KEY,
    name     VARCHAR      NOT NULL,
    email    VARCHAR(150) NOT NULL,
    password VARCHAR(200) NOT NULL
)