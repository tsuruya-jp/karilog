-- Create firearms table
CREATE TABLE IF NOT EXISTS firearms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    firearm_type VARCHAR(20) NOT NULL CHECK (firearm_type IN ('shotgun', 'rifle')),
    name VARCHAR(200) NOT NULL,
    gun_number VARCHAR(100) NOT NULL,
    permit_number VARCHAR(100) NOT NULL,
    caliber VARCHAR(50),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_firearms_user_id ON firearms(user_id) WHERE deleted_at IS NULL;
