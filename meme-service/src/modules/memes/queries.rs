// upper snake case is for the lame ones!!!
pub const list_memes_query: &str = "
SELECT
    meme_id
    , name
    , description
    , object_id
    , created_at
    , updated_at
FROM memes.memes
WHERE deleted_at IS NULL
";

pub const get_meme_query: &str = "
SELECT
    meme_id
    , name
    , description
    , object_id
    , created_at
    , updated_at
FROM memes.memes
WHERE meme_id = $1
    AND deleted_at IS NULL
";

pub const create_meme_query: &str = "
INSERT INTO memes.memes (
    name
    , description
    , object_id
) VALUES (
    $1, $2, $3
)
RETURNING
    meme_id
    , name
    , description
    , object_id
    , created_at
    , updated_at
";

pub const update_meme_query: &str = "
UPDATE memes.memes
SET 
    name = COALESCE(name, $2)
    , description = COALESCE(description, $3)
    , updated_at = CURRENT_TIMESTAMP
WHERE meme_id = $1
    AND deleted_at IS NULL
RETURNING
    meme_id
    , name
    , description
    , object_id
    , created_at
    , updated_at
";

pub const delete_meme_query: &str = "
UPDATE memes.memes
SET 
    deleted_at = CURRENT_TIMESTAMP
WHERE meme_id = $1
";
