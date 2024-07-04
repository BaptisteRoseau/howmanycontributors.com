CREATE DATABASE hmc;
\c hmc;

-- Created and Updated helpers
-- Requires the table to have an `updated_at` field
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    NEW.valid_until = now() + make_interval(weeks => 1);
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Use as follows:
-- CREATE OR REPLACE TRIGGER update_<YOUR_TABLE>_updated_at
--     BEFORE UPDATE ON <YOUR_TABLE>
--     FOR EACH ROW EXECUTE FUNCTION update_modified_column();
-- CREATE INDEX index_<YOUR_TABLE>__created_at ON <YOUR_TABLE>(created_at);

-- -----------------------------------------------------------------------------
-- REPOSITORIES
-- -----------------------------------------------------------------------------

-- GitHub username limit is 39 characters
-- GitHub repo limit is 100 characters
-- We use 150 to get a bit of margin

CREATE TABLE repositories (
    path                VARCHAR(150) UNIQUE NOT NULL,
    contributors        INTEGER,
    dependencies        VARCHAR(150) ARRAY,
    created_at  TIMESTAMP WITH TIME ZONE    DEFAULT now(),
    updated_at  TIMESTAMP WITH TIME ZONE    DEFAULT now(),
    valid_until TIMESTAMP WITH TIME ZONE    DEFAULT now() + make_interval(weeks => 1),
    PRIMARY KEY(path)
);
CREATE INDEX index_repositories__path ON repositories(path);
CREATE INDEX index_repositories__created_at ON repositories(created_at);
CREATE INDEX index_repositories__updated_at ON repositories(updated_at);
CREATE OR REPLACE TRIGGER update_repositories__updated_at
    BEFORE UPDATE ON repositories
    FOR EACH ROW EXECUTE FUNCTION update_modified_column();
