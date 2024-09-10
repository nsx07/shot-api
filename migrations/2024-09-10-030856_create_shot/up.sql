CREATE TABLE shot (
    id INT AUTO_INCREMENT PRIMARY KEY,
    lat VARCHAR(255) NOT NULL,
    lon VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    image64 TEXT NOT NULL,
    location VARCHAR(255) NOT NULL,
    hashtags VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Seed data
INSERT INTO shot (lat, lon, name, image64, location, hashtags, description) VALUES
('40.7128', '-74.0060', 'New York Skyline', 'base64_encoded_image_1', 'New York City, USA', '#nyc #skyline', 'A breathtaking view of the New York City skyline at sunset.'),
('48.8566', '2.3522', 'Eiffel Tower', 'base64_encoded_image_2', 'Paris, France', '#paris #eiffeltower', 'The iconic Eiffel Tower lit up at night.'),
('35.6762', '139.6503', 'Tokyo Street', 'base64_encoded_image_3', 'Tokyo, Japan', '#tokyo #street', 'A bustling street in Tokyo with neon signs and crowds.'),
('-33.8688', '151.2093', 'Sydney Opera House', 'base64_encoded_image_4', 'Sydney, Australia', '#sydney #operahouse', 'The famous Sydney Opera House with its distinctive sail-like design.'),
('51.5074', '-0.1278', 'London Bridge', 'base64_encoded_image_5', 'London, UK', '#london #bridge', 'A view of the iconic London Bridge spanning the River Thames.'),
('37.7749', '-122.4194', 'Golden Gate Bridge', 'base64_encoded_image_6', 'San Francisco, USA', '#sanfrancisco #goldengate', 'The majestic Golden Gate Bridge shrouded in fog.'),
('41.9028', '12.4964', 'Colosseum', 'base64_encoded_image_7', 'Rome, Italy', '#rome #colosseum', 'The ancient Colosseum illuminated at night.'),
('-22.9068', '-43.1729', 'Christ the Redeemer', 'base64_encoded_image_8', 'Rio de Janeiro, Brazil', '#rio #christtheredeemer', 'The statue of Christ the Redeemer overlooking Rio de Janeiro.'),
('1.3521', '103.8198', 'Marina Bay Sands', 'base64_encoded_image_9', 'Singapore', '#singapore #marinabay', 'The futuristic Marina Bay Sands hotel and its infinity pool.'),
('25.1972', '55.2744', 'Burj Khalifa', 'base64_encoded_image_10', 'Dubai, UAE', '#dubai #burjkhalifa', 'The world''s tallest building, Burj Khalifa, piercing the sky.');
