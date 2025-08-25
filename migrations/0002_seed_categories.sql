-- Seed basic categories

INSERT INTO category (category_name, display_name, display_order) VALUES
('general', 'General', 0),
('projects', 'Projects', 1),
('resources', 'Resources', 2),
('examples', 'Examples', 3)
ON CONFLICT (category_name) DO NOTHING;
