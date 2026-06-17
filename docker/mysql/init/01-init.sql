CREATE TABLE IF NOT EXISTS users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(150) NOT NULL UNIQUE,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS orders (
  id INT AUTO_INCREMENT PRIMARY KEY,
  user_id INT NOT NULL,
  total DECIMAL(10, 2) NOT NULL,
  status VARCHAR(30) NOT NULL DEFAULT 'pending',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_orders_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT IGNORE INTO users (name, email, active)
VALUES
  ('Ada Lovelace', 'ada@example.com', TRUE),
  ('Grace Hopper', 'grace@example.com', TRUE),
  ('Alan Turing', 'alan@example.com', FALSE);

INSERT INTO orders (user_id, total, status)
SELECT users.id, seeded_orders.total, seeded_orders.status
FROM (
  SELECT 'ada@example.com' AS email, 120.50 AS total, 'paid' AS status
  UNION ALL SELECT 'ada@example.com', 45.00, 'pending'
  UNION ALL SELECT 'grace@example.com', 88.90, 'paid'
  UNION ALL SELECT 'alan@example.com', 12.30, 'cancelled'
) AS seeded_orders
JOIN users ON users.email = seeded_orders.email;
