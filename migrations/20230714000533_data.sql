-- Add migration script here
-- Add migration script here

INSERT INTO categories (id,name) VALUES (1,'Category A');
INSERT INTO categories (id,name) VALUES (2,'Category B');
INSERT INTO categories (id,name) VALUES (3,'Category C');

INSERT INTO posts (id, title, description) VALUES (1, 'Post 1', 'Description of post 1');
INSERT INTO posts (id, title, description) VALUES (2, 'Post 2', 'Description of post 2');
INSERT INTO posts (id, title, description) VALUES (3, 'Post 3', 'Description of post 3');
INSERT INTO posts (id, title, description) VALUES (4,  'Post 4','Description of post 4');
INSERT INTO posts (id, title, description) VALUES (5, 'Post 5','Description of post 5');


insert into categories_posts values(1,1);
insert into categories_posts values(2,1);
insert into categories_posts values(3,1);
insert into categories_posts values(4,1);