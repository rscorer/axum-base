-- Seed sample items for demonstration

INSERT INTO items (title, description, data, category_id) VALUES
(
    'Welcome to Axum Base',
    'This is a sample item to demonstrate the generic item system.',
    '{"type": "welcome", "priority": "high"}',
    (SELECT id FROM category WHERE category_name = 'general')
),
(
    'Example Project',
    'A sample project item showing how to store structured data.',
    '{"status": "active", "tags": ["rust", "web", "example"], "created_by": "system"}',
    (SELECT id FROM category WHERE category_name = 'projects')
),
(
    'API Documentation',
    'Link to the API documentation and examples.',
    '{"url": "https://docs.rs/axum/", "external": true}',
    (SELECT id FROM category WHERE category_name = 'resources')
),
(
    'Database Query Example',
    'Shows how to use the flexible JSONB data field.',
    '{"query_type": "select", "table": "items", "complexity": "medium"}',
    (SELECT id FROM category WHERE category_name = 'examples')
);
