ALTER TABLE subscriptions RENAME TO subscriptions_old;
ALTER TABLE users RENAME TO users_old;

CREATE TABLE IF NOT EXISTS users (
    tg_user_id     BIGINT    PRIMARY KEY,
    tg_chat_id     BIGINT    NOT NULL
);

INSERT INTO users (tg_user_id, tg_chat_id)
    SELECT tg_user_id, tg_chat_id
    FROM users_old;

CREATE TABLE IF NOT EXISTS subscriptions (
    user_id        BIGINT    NOT NULL,
    topic_id       BIGINT    NOT NULL,
    last_post_id   INT       DEFAULT 0,

    FOREIGN KEY (user_id) REFERENCES users (tg_user_id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES topics (id) ON DELETE CASCADE
);

INSERT INTO subscriptions (user_id, topic_id, last_post_id)
    SELECT uo.tg_user_id, so.topic_id, so.last_post_id FROM subscriptions_old so
    JOIN users_old uo ON uo.id = so.user_id;

DROP TABLE subscriptions_old;
DROP TABLE users_old;
