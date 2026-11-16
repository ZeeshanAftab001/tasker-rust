-- migrate:up
INSERT INTO Todo(title,content,status)
VALUES
('Reading','Read War and Peace at Night',false),
('Exersise','Do exersise at Night',false),
('Programming','Word on Tasker using rust',true);
-- migrate:down

