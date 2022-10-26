INSERT INTO 
users(login, password, email)
VALUES
('TestUser', 'Password', 'TestUser@gmail.com'), 
('Admin', 'PasswordAdmin', 'admin@gmail.com');

INSERT INTO 
generations(name, owner_id, description, map_id, life_type, feed_type, setup_type, tick_period, setup_json) 
VALUES 
('testGen', 1, 'First ever generation', 'Cube', 'life', 'feed', 'setup', 0.1, '{}'),
('testGen1', 1, 's', 'Cube', 'life', 'feed', 'setup2', 0.2, '{}'),
('testGen1', 2, '', 'Circle', 'life1', 'feed', 'setup', 0.5, '{}');

INSERT INTO maps(name) VALUES ('Map0'), ('Map1');
INSERT INTO life_types(name) VALUES ('LT0'), ('LT1');
INSERT INTO feed_types(name) VALUES ('FT0'), ('FT1');
INSERT INTO tick_periods(period) VALUES (0.1), (0.5);
INSERT INTO setup_types(name, json) VALUES ('ST0', '{"fuck" : true}'), ('ST1', '{}');