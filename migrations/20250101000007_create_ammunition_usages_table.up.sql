-- Create ammunition_usages table
CREATE TABLE IF NOT EXISTS ammunition_usages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    ammunition_type_id UUID NOT NULL,
    firearm_id UUID,
    hunting_record_id UUID,
    usage_date DATE NOT NULL,
    location VARCHAR(200),
    quantity_used INTEGER NOT NULL CHECK (quantity_used > 0),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (ammunition_type_id) REFERENCES ammunition_types(id) ON DELETE RESTRICT,
    FOREIGN KEY (firearm_id) REFERENCES firearms(id) ON DELETE SET NULL
);

-- Create indexes
CREATE INDEX idx_ammunition_usages_user_date ON ammunition_usages(user_id, usage_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_ammunition_usages_ammunition_type ON ammunition_usages(ammunition_type_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_ammunition_usages_hunting_record ON ammunition_usages(hunting_record_id) WHERE hunting_record_id IS NOT NULL AND deleted_at IS NULL;
