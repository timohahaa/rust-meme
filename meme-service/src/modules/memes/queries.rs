pub static list_memes_query: &str = "
SELECT
    meme_id
    , name
    , description
    , s3_path
    , created_at
    , updated_at
FROM memes.memes
WHERE deleted_at IS NULL
";
