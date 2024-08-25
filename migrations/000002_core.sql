-- +migrate Up
CREATE SCHEMA IF NOT EXISTS memes;

CREATE TABLE IF NOT EXISTS memes.memes (
    meme_id       UUID      NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY
    , name        TEXT      NOT NULL 
    , description TEXT
    , s3_path     TEXT      NOT NULL
    , created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    , updated_at  TIMESTAMP
    , deleted_at  TIMESTAMP
);

-- +migrate Down
DROP SCHEMA IF EXISTS memes CASCADE;
