CREATE TABLE IF NOT EXISTS bot (
    bot_id text PRIMARY KEY,
    flows_user text NOT NULL,
    token text NOT NULL,
    workspace_id text NOT NULL,
    workspace_name text
);

CREATE TABLE IF NOT EXISTS listener (
    flow_id text NOT NULL,
    flows_user text NOT NULL,
    database text NOT NULL,
    PRIMARY KEY (flow_id, flows_user, database)
);
