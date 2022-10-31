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

INSERT INTO maps(name) VALUES ('StandartMap');
INSERT INTO life_types(name) VALUES ('RepeatSetupLifeType');
INSERT INTO feed_types(name) VALUES ('StandartFeeding');
INSERT INTO tick_periods(period) VALUES (0.1), (0.5);
INSERT INTO setup_types(name, json) VALUES ('Random_Generation', '{"start_cells_count" : 0, "description" : "Random generation description"}');