DROP TABLE IF EXISTS videos;

CREATE TABLE videos (
  title  VARCHAR(100) NOT NULL,
  url VARCHAR(60) NOT NULL,
  video_id VARCHAR(11) PRIMARY KEY,
  thumbnail_url VARCHAR(250) NOT NULL,
  query_time TIMESTAMP,
  user_id INT NOT NULL,
  size BIGINT NOT NULL 
);

GRANT ALL PRIVILEGES on table videos to yt;
GRANT ALL PRIVILEGES on all sequences in schema public to yt;
