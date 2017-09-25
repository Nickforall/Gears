-- Your SQL goes here
CREATE TABLE projects (
    id int PRIMARY KEY,
    name varchar(255) NOT NULL,
    description varchar(255) NOT NULL,
    owner_id int NOT NULL
)
