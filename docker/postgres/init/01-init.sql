CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(150) NOT NULL UNIQUE,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS orders (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  total NUMERIC(10, 2) NOT NULL,
  status VARCHAR(30) NOT NULL DEFAULT 'pending',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO users (name, email, active)
VALUES
  ('Ada Lovelace', 'ada@example.com', TRUE),
  ('Grace Hopper', 'grace@example.com', TRUE),
  ('Alan Turing', 'alan@example.com', FALSE)
ON CONFLICT (email) DO NOTHING;

INSERT INTO orders (user_id, total, status)
SELECT users.id, orders.total, orders.status
FROM (
  VALUES
    ('ada@example.com', 120.50, 'paid'),
    ('ada@example.com', 45.00, 'pending'),
    ('grace@example.com', 88.90, 'paid'),
    ('alan@example.com', 12.30, 'cancelled')
) AS orders(email, total, status)
JOIN users ON users.email = orders.email;
