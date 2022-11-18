DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL,

    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,

    UNIQUE (login),
    UNIQUE (email)
);

DROP TABLE IF EXISTS generations;
CREATE TABLE generations(
    id SERIAL,

    name VARCHAR NOT NULL,
    owner_id INTEGER NOT NULL,
    description VARCHAR NOT NULL DEFAULT '',

    map_prefab VARCHAR NOT NULL,
    life_type_prefab VARCHAR NOT NULL,
    feed_type_prefab VARCHAR NOT NULL,
    setup_type_prefab VARCHAR NOT NULL,

    map_json VARCHAR NOT NULL,
    life_type_json VARCHAR NOT NULL,
    feed_type_json VARCHAR NOT NULL,
    setup_type_json VARCHAR NOT NULL,

    tick_period DECIMAL NOT NULL,

    last_send_num BIGINT NOT NULL DEFAULT 0,
    last_cell_num BIGINT NOT NULL DEFAULT 0,

    UNIQUE(name, owner_id)
);

DROP TABLE IF EXISTS cells;
CREATE TABLE cells(
    id SERIAL,
    generation_id INTEGER NOT NULL,
    local_id INTEGER NOT NULL,

    parent_id INTEGER NOT NULL
);

DROP TABLE IF EXISTS modules;
CREATE TABLE modules(
    id SERIAL,
    cell_id INTEGER NOT NULL,

    name VARCHAR NOT NULL,
    value DECIMAL,

    UNIQUE(cell_id, name)
);

DROP TABLE IF EXISTS intellect;
CREATE TABLE intellect(
    id SERIAL,
    cell_id INTEGER NOT NULL,

    in_neuron_count INTEGER NOT NULL,
    out_neuron_count INTEGER NOT NULL
);

DROP TABLE IF EXISTS neurons;
CREATE TABLE neurons (
    id SERIAL,
    intellect_id INTEGER NOT NULL,

    bias DECIMAL NOT NULL DEFAULT 0
);

DROP TABLE IF EXISTS gens;
CREATE TABLE gens(
    id SERIAL,
    intellect_id INTEGER NOT NULL,

    from_id INTEGER NOT NULL,
    to_id INTEGER NOT NULL,
    weight DECIMAL NOT NULL DEFAULT 0
);

CREATE TYPE diff_type AS ENUM ('create_cell', 'change_module_value', 'remove_cell');

DROP TABLE IF EXISTS diffs;
CREATE TABLE diffs(
    id SERIAL,
    cell_id INTEGER NOT NULL, -- global cell id
    tick_id INTEGER NOT NULL,

    type diff_type NOT NULL DEFAULT 'change_module_value',
    
    changed_module VARCHAR,
    new_value DECIMAL
);