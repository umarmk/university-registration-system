-- Create roles table
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    role_id INTEGER NOT NULL REFERENCES roles (id),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_login TIMESTAMP
    WITH
        TIME ZONE,
        created_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_tokens table for refresh tokens
CREATE TABLE user_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL,
        created_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create audit_logs table
CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id INTEGER,
    details JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMP
    WITH
        TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default roles
INSERT INTO
    roles (name, description)
VALUES
    ('admin', 'Administrator with full access'),
    ('user', 'Regular user with limited access');

-- Insert default admin user (password: admin123)
INSERT INTO
    users (
        username,
        email,
        password_hash,
        first_name,
        last_name,
        role_id
    )
VALUES
    (
        'admin',
        'admin@university.edu',
        '$2a$12$1InE4YmvrRUQpzGDKpBcbewrGCMeQ9mxGJwNmh5FsDgJzkTWEW1lW',
        'System',
        'Administrator',
        (
            SELECT
                id
            FROM
                roles
            WHERE
                name = 'admin'
        )
    );