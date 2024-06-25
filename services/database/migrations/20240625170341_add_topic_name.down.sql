ALTER TABLE topics RENAME TO topics_old;

CREATE TABLE IF NOT EXISTS topics (
    id             BIGINT     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    query          TEXT       NOT NULL
);

INSERT INTO topics (id, query)
    OVERRIDING SYSTEM VALUE
    SELECT id, query FROM topics_old;

DROP TABLE topics_old CASCADE;
