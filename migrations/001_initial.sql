-- Organizations/workspaces (for cloud multi-tenancy)
CREATE TABLE organizations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(255) NOT NULL,
    slug            VARCHAR(100) UNIQUE NOT NULL,
    logo_url        TEXT,
    custom_domain   VARCHAR(255),
    plan            VARCHAR(20) DEFAULT 'free',
    settings        JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);

-- Users
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           VARCHAR(255) UNIQUE NOT NULL,
    name            VARCHAR(255) NOT NULL,
    avatar_url      TEXT,
    password_hash   TEXT,
    provider        VARCHAR(50),
    provider_id     VARCHAR(255),
    created_at      TIMESTAMPTZ DEFAULT now()
);

-- Organization members
CREATE TABLE org_members (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          UUID REFERENCES organizations(id) ON DELETE CASCADE,
    user_id         UUID REFERENCES users(id) ON DELETE CASCADE,
    role            VARCHAR(20) DEFAULT 'member',
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(org_id, user_id)
);

-- Boards
CREATE TABLE boards (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          UUID REFERENCES organizations(id) ON DELETE CASCADE,
    name            VARCHAR(255) NOT NULL,
    slug            VARCHAR(100) NOT NULL,
    description     TEXT,
    is_private      BOOLEAN DEFAULT FALSE,
    settings        JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(org_id, slug)
);

-- Feedback posts
CREATE TABLE posts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    board_id        UUID REFERENCES boards(id) ON DELETE CASCADE,
    author_id       UUID REFERENCES users(id),
    title           VARCHAR(500) NOT NULL,
    description     TEXT,
    status          VARCHAR(30) DEFAULT 'open',
    category        VARCHAR(100),
    vote_count      INT DEFAULT 0,
    comment_count   INT DEFAULT 0,
    pinned          BOOLEAN DEFAULT FALSE,
    merged_into_id  UUID REFERENCES posts(id),
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);

-- Votes
CREATE TABLE votes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
    user_id         UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(post_id, user_id)
);

-- Comments
CREATE TABLE comments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
    author_id       UUID REFERENCES users(id),
    body            TEXT NOT NULL,
    is_admin_reply  BOOLEAN DEFAULT FALSE,
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);

-- Changelog entries
CREATE TABLE changelog_entries (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id          UUID REFERENCES organizations(id) ON DELETE CASCADE,
    title           VARCHAR(500) NOT NULL,
    body            TEXT NOT NULL,
    published_at    TIMESTAMPTZ,
    is_draft        BOOLEAN DEFAULT TRUE,
    created_at      TIMESTAMPTZ DEFAULT now(),
    updated_at      TIMESTAMPTZ DEFAULT now()
);

-- Changelog <-> Posts link
CREATE TABLE changelog_posts (
    changelog_id    UUID REFERENCES changelog_entries(id) ON DELETE CASCADE,
    post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
    PRIMARY KEY (changelog_id, post_id)
);

-- Tags
CREATE TABLE tags (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    board_id        UUID REFERENCES boards(id) ON DELETE CASCADE,
    name            VARCHAR(100) NOT NULL,
    color           VARCHAR(7) DEFAULT '#6366f1',
    UNIQUE(board_id, name)
);

CREATE TABLE post_tags (
    post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
    tag_id          UUID REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);

-- Notification subscriptions
CREATE TABLE subscriptions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id) ON DELETE CASCADE,
    post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
    notify_on       VARCHAR(20) DEFAULT 'status',
    created_at      TIMESTAMPTZ DEFAULT now(),
    UNIQUE(user_id, post_id)
);

-- Indexes
CREATE INDEX idx_posts_board_id ON posts(board_id);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_posts_vote_count ON posts(vote_count DESC);
CREATE INDEX idx_votes_post_id ON votes(post_id);
CREATE INDEX idx_votes_user_id ON votes(user_id);
CREATE INDEX idx_comments_post_id ON comments(post_id);
CREATE INDEX idx_org_slug ON organizations(slug);
