CREATE TABLE issues (
    id int PRIMARY KEY,
    user_id int NOT NULL,
    project_id int NOT NULL,
    posted_at int NOT NULL,
    content TEXT,
    is_resolved TINYINT(1) NOT NULL,
    assignee int NOT NULL
)
