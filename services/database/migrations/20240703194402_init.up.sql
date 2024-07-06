CREATE type role AS enum ('user', 'admin', 'owner');

CREATE TABLE IF NOT EXISTS users (
    id           BIGINT       PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name         VARCHAR(64)  NOT NULL,
    role         ROLE         DEFAULT 'user'
);

CREATE TABLE IF NOT EXISTS topics (
    id           BIGINT       PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_id     BIGINT       NOT NULL,
    label        VARCHAR(32)  NOT NULL,
    query        VARCHAR(512) NOT NULL,
    sub_counter  BIGINT       DEFAULT 0,

    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS subscriptions (
    user_id      BIGINT       NOT NULL,
    topic_id     BIGINT       NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);
