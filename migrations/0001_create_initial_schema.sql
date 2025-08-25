-- Create initial schema

-- User table
CREATE TABLE IF NOT EXISTS users
(
    id             SERIAL PRIMARY KEY,
    username       VARCHAR(100) NOT NULL UNIQUE,
    email          VARCHAR(255) NOT NULL UNIQUE,
    password_hash  VARCHAR(255) NOT NULL,
    email_verified BOOLEAN      NOT NULL DEFAULT FALSE,
    is_active      BOOLEAN      NOT NULL DEFAULT TRUE,
    last_login     TIMESTAMPTZ           DEFAULT NULL,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Category table
CREATE TABLE IF NOT EXISTS category
(
    id            SERIAL PRIMARY KEY,
    category_name VARCHAR(50)  NOT NULL UNIQUE,
    display_name  VARCHAR(100) NOT NULL,
    is_visible    BOOLEAN      NOT NULL DEFAULT TRUE,
    display_order INTEGER      NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Items table (generic entity for demonstration)
CREATE TABLE IF NOT EXISTS items
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    data        JSONB,                -- Flexible JSON field for custom data
    is_active   BOOLEAN      NOT NULL DEFAULT TRUE,
    category_id INTEGER      NOT NULL REFERENCES category (id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Create trigger function to update updated_at column
CREATE OR REPLACE FUNCTION update_modified_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to tables
CREATE TRIGGER update_users_modtime
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_category_modtime
    BEFORE UPDATE
    ON category
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_items_modtime
    BEFORE UPDATE
    ON items
    FOR EACH ROW
EXECUTE FUNCTION update_modified_column();
