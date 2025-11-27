-- Create ammunition_types table
CREATE TABLE IF NOT EXISTS ammunition_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    name VARCHAR(200) NOT NULL,
    caliber VARCHAR(50) NOT NULL,
    shot_size VARCHAR(50),
    is_slug BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_ammunition_types_user_id ON ammunition_types(user_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_ammunition_types_caliber ON ammunition_types(user_id, caliber) WHERE deleted_at IS NULL;
