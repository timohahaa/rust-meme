// upper snake case is for the lame ones!!!
pub const LIST_MEMES_QUERY: &str = "
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

pub const GET_MEME_QUERY: &str = "
SELECT
    meme_id
    , name
    , description
    , s3_path
    , created_at
    , updated_at
FROM memes.memes
WHERE meme_id = $1
    AND deleted_at IS NULL
";

pub const CREATE_MEME_QUERY: &str = "
INSERT INTO memes.memes (
    meme_id
    , name
    , description
    , s3_path
    , status
) VALUES (
    $1, $2, $3, $4, 'uploading'
)
RETURNING
    meme_id
    , name
    , description
    , s3_path
    , created_at
    , updated_at
";

pub const MARK_AS_DONE_QUERY: &str = "
UPDATE memes.memes
SET 
    status = 'done'
WHERE meme_id = $1
";

pub const UPDATE_MEME_QUERY: &str = "
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
    , s3_path
    , created_at
    , updated_at
";

pub const DELETE_MEME_QUERY: &str = "
UPDATE memes.memes
SET 
    deleted_at = CURRENT_TIMESTAMP
WHERE meme_id = $1
";
