-- Add migration script here

CREATE TABLE IF NOT EXISTS "civ" (
    "id" VARCHAR PRIMARY KEY,
    -- The ref is a sha256 hash of the id followed by the first name then last name.
    -- as the middle name can be optional, it is not included here.
    -- For the devs among us: Similarly to Git, the first few characters may be used 
    -- as an ID to reference a civ. Why? Because we use snowflakes which are a bit too long
    -- to riddle off fast and what not. However, unlike Git, it's the first 4 characters.
    -- This is valid in ANY form that requires a user id. You can use this or the full ID.
    "ref" VARCHAR NOT NULL,
    -- The unique state id. Internally its called this but on the frontend (any part that the 
    -- user interacts with) it will be shown with whatever the server owner/manager has set it to.
    "unique_state_id" VARCHAR NOT NULL,
    "first" VARCHAR NOT NULL,
    "middle" VARCHAR,
    "last" VARCHAR NOT NULL,
    "date_of_birth" DATE NOT NULL,
    "race" VARCHAR NOT NULL,
    -- THIS IS EXPERIMENTAL FROM THE BEGINNING LOL.
    -- This is pretty simple. It's an list of teams,orgs,etc. that the user has access to.
    -- the format of each entry is `<key>:<id>`. 
    -- where:
    -- key = a string that represents what the whitelist is for
    -- id = the id of said whitelisted item. if there is no id for the item then it is left as just the key
    -- Example: ["pd","org:123456789"]
    -- the example user is able to access the PD and anything pertaining to the organization with the id 123456789
    "whitelists" VARCHAR[],
    "ip_that_created" INET NOT NULL,
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);