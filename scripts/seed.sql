-- Seed script: demo data for local development
-- All users have password: "password"
-- Run: psql $DATABASE_URL -f scripts/seed.sql

BEGIN;

-- Clean existing data (order matters for FK constraints)
TRUNCATE post_tags, tags, subscriptions, changelog_posts, changelog_entries,
         comments, votes, posts, boards, org_members, organizations, users
         CASCADE;

-- ============================================================
-- Users
-- ============================================================
INSERT INTO users (id, email, name, avatar_url, password_hash, provider) VALUES
  ('aaaaaaaa-0000-0000-0000-000000000001', 'admin@demo.com',  'Alice Admin',  NULL, '$argon2id$v=19$m=19456,t=2,p=1$qOOPfjSVFW/XMN03eJQjYg$d1Zl7+kUCnfoXmxfVVuk/lTPgcNz8zeEyEeHiJExct8', 'email'),
  ('aaaaaaaa-0000-0000-0000-000000000002', 'user@demo.com',   'Bob User',     NULL, '$argon2id$v=19$m=19456,t=2,p=1$qOOPfjSVFW/XMN03eJQjYg$d1Zl7+kUCnfoXmxfVVuk/lTPgcNz8zeEyEeHiJExct8', 'email'),
  ('aaaaaaaa-0000-0000-0000-000000000003', 'member@demo.com', 'Carol Member', NULL, '$argon2id$v=19$m=19456,t=2,p=1$qOOPfjSVFW/XMN03eJQjYg$d1Zl7+kUCnfoXmxfVVuk/lTPgcNz8zeEyEeHiJExct8', 'email');

-- ============================================================
-- Organization
-- ============================================================
INSERT INTO organizations (id, name, slug) VALUES
  ('bbbbbbbb-0000-0000-0000-000000000001', 'Acme Corp', 'acme-corp');

INSERT INTO org_members (org_id, user_id, role) VALUES
  ('bbbbbbbb-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000001', 'admin'),
  ('bbbbbbbb-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000002', 'member'),
  ('bbbbbbbb-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000003', 'member');

-- ============================================================
-- Boards
-- ============================================================
INSERT INTO boards (id, org_id, name, slug, description) VALUES
  ('cccccccc-0000-0000-0000-000000000001', 'bbbbbbbb-0000-0000-0000-000000000001', 'Feature Requests', 'feature-requests', 'Vote on features you want to see next'),
  ('cccccccc-0000-0000-0000-000000000002', 'bbbbbbbb-0000-0000-0000-000000000001', 'Bug Reports',      'bug-reports',      'Report bugs and track fixes'),
  ('cccccccc-0000-0000-0000-000000000003', 'bbbbbbbb-0000-0000-0000-000000000001', 'General Feedback',  'general-feedback',  'Share any ideas or suggestions');

-- ============================================================
-- Tags
-- ============================================================
INSERT INTO tags (id, board_id, name, color) VALUES
  -- Feature Requests tags
  ('dddddddd-0000-0000-0000-000000000001', 'cccccccc-0000-0000-0000-000000000001', 'UX',       '#8b5cf6'),
  ('dddddddd-0000-0000-0000-000000000002', 'cccccccc-0000-0000-0000-000000000001', 'Backend',  '#3b82f6'),
  ('dddddddd-0000-0000-0000-000000000003', 'cccccccc-0000-0000-0000-000000000001', 'Mobile',   '#10b981'),
  -- Bug Reports tags
  ('dddddddd-0000-0000-0000-000000000004', 'cccccccc-0000-0000-0000-000000000002', 'Critical',   '#ef4444'),
  ('dddddddd-0000-0000-0000-000000000005', 'cccccccc-0000-0000-0000-000000000002', 'Minor',      '#f59e0b'),
  ('dddddddd-0000-0000-0000-000000000006', 'cccccccc-0000-0000-0000-000000000002', 'Regression', '#ec4899');

-- ============================================================
-- Posts — Feature Requests board
-- ============================================================
INSERT INTO posts (id, board_id, author_id, title, description, status, vote_count, comment_count, pinned, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000001', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Dark mode support',
   'It would be great to have a dark mode option. Many of us work late and the bright interface is hard on the eyes.',
   'planned', 12, 3, FALSE, now() - interval '10 days'),

  ('eeeeeeee-0000-0000-0000-000000000002', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Keyboard shortcuts',
   'Add vim-style keyboard shortcuts for power users. At minimum j/k navigation and Enter to open.',
   'open', 8, 1, FALSE, now() - interval '7 days'),

  ('eeeeeeee-0000-0000-0000-000000000003', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000001',
   'API access for integrations',
   'Expose a public REST API so teams can build custom integrations with Slack, Linear, Jira, etc.',
   'in_progress', 21, 4, TRUE, now() - interval '14 days'),

  ('eeeeeeee-0000-0000-0000-000000000004', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Export feedback to CSV',
   'Allow admins to export all posts and votes to CSV for reporting.',
   'done', 5, 0, FALSE, now() - interval '20 days'),

  ('eeeeeeee-0000-0000-0000-000000000005', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Email notifications for status changes',
   'Send an email to voters when a post they voted on changes status.',
   'open', 15, 2, FALSE, now() - interval '3 days'),

  ('eeeeeeee-0000-0000-0000-000000000006', 'cccccccc-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Single sign-on (SSO)',
   'Support SAML/OIDC for enterprise customers.',
   'closed', 3, 1, FALSE, now() - interval '30 days');

-- ============================================================
-- Posts — Bug Reports board
-- ============================================================
INSERT INTO posts (id, board_id, author_id, title, description, status, vote_count, comment_count, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000007', 'cccccccc-0000-0000-0000-000000000002', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Vote count resets on page refresh',
   'When I vote on a post and then refresh the page, the vote count goes back to the previous value.',
   'in_progress', 6, 2, now() - interval '2 days'),

  ('eeeeeeee-0000-0000-0000-000000000008', 'cccccccc-0000-0000-0000-000000000002', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Login fails with special characters in password',
   'If your password contains a backtick or backslash, login returns 500.',
   'open', 4, 1, now() - interval '1 day'),

  ('eeeeeeee-0000-0000-0000-000000000009', 'cccccccc-0000-0000-0000-000000000002', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Sidebar boards list not updating after create',
   'After creating a new board, the sidebar doesn''t show it until you do a full page refresh.',
   'done', 2, 0, now() - interval '5 days');

-- ============================================================
-- Posts — General Feedback board
-- ============================================================
INSERT INTO posts (id, board_id, author_id, title, description, status, vote_count, comment_count, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000010', 'cccccccc-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Love the clean UI!',
   'Just wanted to say the interface is really clean and easy to use. Keep it up!',
   'open', 7, 2, now() - interval '6 days'),

  ('eeeeeeee-0000-0000-0000-000000000011', 'cccccccc-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Mobile experience could be better',
   'The app works on mobile but the layout is pretty cramped. Would love a responsive redesign.',
   'planned', 9, 1, now() - interval '4 days');

-- ============================================================
-- Post tags
-- ============================================================
INSERT INTO post_tags (post_id, tag_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000001', 'dddddddd-0000-0000-0000-000000000001'), -- Dark mode -> UX
  ('eeeeeeee-0000-0000-0000-000000000002', 'dddddddd-0000-0000-0000-000000000001'), -- Keyboard shortcuts -> UX
  ('eeeeeeee-0000-0000-0000-000000000003', 'dddddddd-0000-0000-0000-000000000002'), -- API access -> Backend
  ('eeeeeeee-0000-0000-0000-000000000003', 'dddddddd-0000-0000-0000-000000000003'), -- API access -> Mobile
  ('eeeeeeee-0000-0000-0000-000000000005', 'dddddddd-0000-0000-0000-000000000002'), -- Notifications -> Backend
  ('eeeeeeee-0000-0000-0000-000000000007', 'dddddddd-0000-0000-0000-000000000004'), -- Vote reset bug -> Critical
  ('eeeeeeee-0000-0000-0000-000000000008', 'dddddddd-0000-0000-0000-000000000005'), -- Login bug -> Minor
  ('eeeeeeee-0000-0000-0000-000000000009', 'dddddddd-0000-0000-0000-000000000006'); -- Sidebar bug -> Regression

-- ============================================================
-- Votes (create matching vote rows for the counts above)
-- ============================================================
-- Dark mode (12 votes — we'll add 2 from our known users, rest are reflected in count)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000001'),
  ('eeeeeeee-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000003');
-- Keyboard shortcuts
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000002', 'aaaaaaaa-0000-0000-0000-000000000001');
-- API access (Bob voted)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000002');
-- Email notifications (admin + Carol voted)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000005', 'aaaaaaaa-0000-0000-0000-000000000001'),
  ('eeeeeeee-0000-0000-0000-000000000005', 'aaaaaaaa-0000-0000-0000-000000000003');
-- Vote count bug (all 3 voted)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000007', 'aaaaaaaa-0000-0000-0000-000000000001'),
  ('eeeeeeee-0000-0000-0000-000000000007', 'aaaaaaaa-0000-0000-0000-000000000002');
-- Love the UI (Bob + Carol voted)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000010', 'aaaaaaaa-0000-0000-0000-000000000002'),
  ('eeeeeeee-0000-0000-0000-000000000010', 'aaaaaaaa-0000-0000-0000-000000000003');
-- Mobile experience (admin voted)
INSERT INTO votes (post_id, user_id) VALUES
  ('eeeeeeee-0000-0000-0000-000000000011', 'aaaaaaaa-0000-0000-0000-000000000001');

-- ============================================================
-- Comments
-- ============================================================
-- Dark mode comments
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Yes please! I use dark mode everywhere.', FALSE, now() - interval '9 days'),
  ('eeeeeeee-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000001',
   'We''ve added this to our roadmap for next quarter. Thanks for the feedback!', TRUE, now() - interval '8 days'),
  ('eeeeeeee-0000-0000-0000-000000000001', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Can''t wait for this. Will it support system-preference auto-switching?', FALSE, now() - interval '5 days');

-- Keyboard shortcuts comment
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000002', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Great idea. We''ll look into this once the core features stabilize.', TRUE, now() - interval '6 days');

-- API access comments
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000002',
   'This would be a game changer for our team. We use Linear heavily.', FALSE, now() - interval '13 days'),
  ('eeeeeeee-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Working on this now! The initial API will cover posts and votes.', TRUE, now() - interval '10 days'),
  ('eeeeeeee-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Will there be webhooks too?', FALSE, now() - interval '8 days'),
  ('eeeeeeee-0000-0000-0000-000000000003', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Webhooks are planned for a follow-up release.', TRUE, now() - interval '7 days');

-- Email notifications comments
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000005', 'aaaaaaaa-0000-0000-0000-000000000002',
   'Would love digest emails too, not just instant notifications.', FALSE, now() - interval '2 days'),
  ('eeeeeeee-0000-0000-0000-000000000005', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Agreed, digest mode would be really nice.', FALSE, now() - interval '1 day');

-- SSO comment
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000006', 'aaaaaaaa-0000-0000-0000-000000000001',
   'We''re closing this for now as SSO is out of scope for the current roadmap. Will revisit later.', TRUE, now() - interval '25 days');

-- Vote count bug comments
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000007', 'aaaaaaaa-0000-0000-0000-000000000002',
   'I can reproduce this consistently. Using Chrome on macOS.', FALSE, now() - interval '1 day'),
  ('eeeeeeee-0000-0000-0000-000000000007', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Found the issue — we''re fixing the cache invalidation. Patch coming soon.', TRUE, now() - interval '12 hours');

-- Login bug comment
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000008', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Thanks for reporting. Can you share the exact characters that cause the issue?', TRUE, now() - interval '12 hours');

-- Love the UI comments
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000010', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Thank you! We put a lot of effort into keeping things simple.', TRUE, now() - interval '5 days'),
  ('eeeeeeee-0000-0000-0000-000000000010', 'aaaaaaaa-0000-0000-0000-000000000003',
   'Seconded! Really intuitive.', FALSE, now() - interval '4 days');

-- Mobile experience comment
INSERT INTO comments (post_id, author_id, body, is_admin_reply, created_at) VALUES
  ('eeeeeeee-0000-0000-0000-000000000011', 'aaaaaaaa-0000-0000-0000-000000000001',
   'Mobile responsive improvements are planned for the next sprint.', TRUE, now() - interval '3 days');

COMMIT;
