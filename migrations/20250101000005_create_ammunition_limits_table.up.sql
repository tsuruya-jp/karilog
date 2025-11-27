-- Create ammunition_limits table
CREATE TABLE IF NOT EXISTS ammunition_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    caliber VARCHAR(50) NOT NULL,
    max_quantity INTEGER NOT NULL CHECK (max_quantity > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT unique_user_caliber UNIQUE (user_id, caliber)
);

-- Create indexes
CREATE INDEX idx_ammunition_limits_user_caliber ON ammunition_limits(user_id, caliber);
