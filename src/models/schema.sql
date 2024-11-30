USE sopranodb;

-- Table holds all .wav files

CREATE TABLE IF NOT EXISTS songs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Deletes any duplicates from the table

DELETE FROM songs 
     WHERE id NOT IN (
     SELECT * FROM (
     SELECT MIN(id) 
     FROM songs
     GROUP BY name, file_path
     ) AS alias
);

