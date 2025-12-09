-- Create default admin user
-- Username: admin
-- Password: admin123 (IMPORTANT: Change this after first login!)
INSERT INTO users (username, password_hash) 
VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$eeJjrWObHdxiWoanvq+MGA$lJqpAXsubB1USc/Ds6HaYWbTq4znJ9YK6wmcnxHEr0Q')
ON CONFLICT (username) DO NOTHING;
