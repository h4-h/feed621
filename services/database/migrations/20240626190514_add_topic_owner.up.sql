-- WARN: this migration breaks app logic

ALTER TABLE topics RENAME TO topics_old;

CREATE TABLE IF NOT EXISTS topics (
    id             BIGINT     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_id       BIGINT     NOT NULL,
    name           TEXT       NOT NULL,
    query          TEXT       NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES users (tg_user_id) ON DELETE CASCADE
);

INSERT INTO users (tg_user_id, tg_chat_id) VALUES (0, 0) ON CONFLICT DO NOTHING;

INSERT INTO topics (id, owner_id, name, query)
    OVERRIDING SYSTEM VALUE
    SELECT id, 0, name, query FROM topics_old;

DROP TABLE topics_old CASCADE;
