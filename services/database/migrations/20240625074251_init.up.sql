CREATE TABLE IF NOT EXISTS users (
    id             BIGINT     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    tg_user_id     BIGINT     NOT NULL,
    tg_chat_id     BIGINT     NOT NULL
);

CREATE TABLE IF NOT EXISTS topics (
    id             BIGINT     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    query          TEXT       NOT NULL
);

CREATE TABLE IF NOT EXISTS subscriptions (
    user_id        BIGINT     NOT NULL,
    topic_id       BIGINT     NOT NULL,
    last_post_id   INT        DEFAULT 0,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);
