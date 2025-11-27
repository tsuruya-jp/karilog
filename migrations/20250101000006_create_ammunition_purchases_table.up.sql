-- Create ammunition_purchases table
CREATE TABLE IF NOT EXISTS ammunition_purchases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    ammunition_type_id UUID NOT NULL,
    firearm_id UUID,
    purchase_date DATE NOT NULL,
    supplier VARCHAR(200) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price INTEGER CHECK (price IS NULL OR price >= 0),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (ammunition_type_id) REFERENCES ammunition_types(id) ON DELETE RESTRICT,
    FOREIGN KEY (firearm_id) REFERENCES firearms(id) ON DELETE SET NULL
);

-- Create indexes
CREATE INDEX idx_ammunition_purchases_user_date ON ammunition_purchases(user_id, purchase_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_ammunition_purchases_ammunition_type ON ammunition_purchases(ammunition_type_id) WHERE deleted_at IS NULL;
