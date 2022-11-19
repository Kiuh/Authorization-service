INSERT INTO 
users(login, password, email)
VALUES
('TestUser', 'Password', 'mertwole@gmail.com'), 
('Admin', 'PasswordAdmin', 'admin@gmail.com');

INSERT INTO 
generations(name, owner_id, description, map_prefab, life_type_prefab, feed_type_prefab, setup_type_prefab, map_json, life_type_json, feed_type_json, setup_type_json, tick_period) 
VALUES 
('testGen', 1, 'First ever generation', 'MP', 'LT', 'FT', 'ST', '{}', '{}', '{}', '{}', 0.1),
('testGen1', 1, 'Descr', 'MP', 'LT', 'FT', 'ST', '{}', '{}', '{}', '{}', 0.1),
('testGen2', 1, 'f', 'MP', 'LT', 'FT', 'ST', '{}', '{}', '{}', '{}', 0.1);
