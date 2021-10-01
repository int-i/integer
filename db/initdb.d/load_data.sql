COPY members FROM '/docker-entrypoint-initdb.d/members.csv' WITH (FORMAT CSV);

