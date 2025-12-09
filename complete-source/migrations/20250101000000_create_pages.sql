CREATE TABLE IF NOT EXISTS pages (
    slug TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO pages (slug, title, content) VALUES 
('about', 'About', '<p>This is the about page. Content goes here.</p>'),
('contact', 'Contact', '<p>Get in touch with me.</p>'),
('footer', 'Footer', '<p>&copy; 2024 Artist Portfolio</p>')
ON CONFLICT (slug) DO NOTHING;
