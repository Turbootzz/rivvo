-- Add NOT NULL to foreign key columns that must always reference a parent

ALTER TABLE org_members
    ALTER COLUMN org_id SET NOT NULL,
    ALTER COLUMN user_id SET NOT NULL;

ALTER TABLE boards
    ALTER COLUMN org_id SET NOT NULL;

ALTER TABLE posts
    ALTER COLUMN board_id SET NOT NULL;

ALTER TABLE votes
    ALTER COLUMN post_id SET NOT NULL,
    ALTER COLUMN user_id SET NOT NULL;

ALTER TABLE comments
    ALTER COLUMN post_id SET NOT NULL;

ALTER TABLE changelog_entries
    ALTER COLUMN org_id SET NOT NULL;

ALTER TABLE changelog_posts
    ALTER COLUMN changelog_id SET NOT NULL,
    ALTER COLUMN post_id SET NOT NULL;

ALTER TABLE tags
    ALTER COLUMN board_id SET NOT NULL;

ALTER TABLE post_tags
    ALTER COLUMN post_id SET NOT NULL,
    ALTER COLUMN tag_id SET NOT NULL;

ALTER TABLE subscriptions
    ALTER COLUMN user_id SET NOT NULL,
    ALTER COLUMN post_id SET NOT NULL;

-- Change author_id to ON DELETE SET NULL (preserve content when user is deleted)
ALTER TABLE posts
    DROP CONSTRAINT posts_author_id_fkey,
    ADD CONSTRAINT posts_author_id_fkey
        FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE comments
    DROP CONSTRAINT comments_author_id_fkey,
    ADD CONSTRAINT comments_author_id_fkey
        FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE SET NULL;
