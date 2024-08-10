CREATE TABLE IF NOT EXISTS users (
    id             BIGINT        PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name           VARCHAR(20)   NOT NULL CHECK(LENGTH(name) > 4),
    email          VARCHAR(256)  NOT NULL UNIQUE,
    password_hash  TEXT          NOT NULL,
    password_salt  TEXT          NOT NULL
);

CREATE TABLE IF NOT EXISTS topics (
    id             BIGINT        PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_id       BIGINT        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name           VARCHAR(20)   NOT NULL CHECK(LENGTH(name) > 4),
    query          VARCHAR(256)  NOT NULL CHECK(LENGTH(query) > 2)
);

CREATE TABLE IF NOT EXISTS subscriptions (
    id             BIGINT        PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id        BIGINT        REFERENCES users(id) ON DELETE CASCADE,
    topic_id       BIGINT        REFERENCES topics(id) ON DELETE CASCADE
);
