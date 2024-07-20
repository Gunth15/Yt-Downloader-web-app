DROP TABLE IF EXISTS videos;

CREATE TABLE videos (
  title  VARCHAR(100),
  url VARCHAR(60),
  video_id VARCHAR(11) PRIMARY KEY,
  thumbnail_url VARCHAR(250),
  query_time TIMESTAMP,
  user_id INT,
  size INT
);

GRANT ALL PRIVILEGES on table videos to yt;
GRANT ALL PRIVILEGES on all sequences in schema public to yt;
