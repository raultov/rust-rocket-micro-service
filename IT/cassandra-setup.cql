CREATE KEYSPACE vehicles WITH REPLICATION = {
    'class' : 'SimpleStrategy',
    'replication_factor' : '1'
};

CREATE TABLE vehicles.vehicle (
    user_id uuid,
    vehicle_id uuid,
    vehicle_type text,
    name text,
    created_at timestamp,
    retired_at timestamp,
    brand text,
    model text,
    distance int,
    owner_since date,
    manufacturing_date date,
    picture text,
    PRIMARY KEY ((user_id), vehicle_id)
);

INSERT INTO vehicles.vehicle (user_id, vehicle_id, vehicle_type, name, created_at, retired_at, brand, model, distance,
    owner_since, manufacturing_date, picture)
    VALUES(d13fe953-297a-4781-807a-f9becc1b71f6, 60e18f00-34b8-4a52-916c-adbb0204618e, 'bike', 'test vehicle 2',
        '2019-10-02T00:00:00.111Z', null, 'Time', 'rtm', 8766, '2014-09-02',
        '2014-09-02', '/images/bike/time/rtm/time.jpg');