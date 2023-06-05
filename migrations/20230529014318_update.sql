-- Add migration script here
UPDATE posts SET name = 'Category C' WHERE name IS NULL;
