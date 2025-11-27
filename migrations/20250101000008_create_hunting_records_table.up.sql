-- Create hunting_records table
CREATE TABLE hunting_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    hunting_date DATE NOT NULL,
    location VARCHAR(200),
    is_planned BOOLEAN NOT NULL DEFAULT true,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);

-- Create indexes
CREATE INDEX idx_hunting_records_user_date ON hunting_records(user_id, hunting_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_hunting_records_user_planned ON hunting_records(user_id, is_planned) WHERE deleted_at IS NULL;
