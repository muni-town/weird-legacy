-- Setup script for the database

-- Every page will have a url, the users github username and a simple display name
CREATE TABLE page (
    github_username TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL
);

-- Links that are created by the user will be stored in this TABLE
-- They are related to their parent page through its unique url
create TABLE links (
    title TEXT NOT NULL,
    url TEXT,
    github_username TEXT REFERENCES page(github_username)
);

-- Randomly generated sample pages
INSERT INTO page (github_username, display_name) VALUES
  ('Smokey',  'Joelle Bethea'),
  ('Noodle',  'Margarette Cress'),
  ('Loki',    'Lesia Dietz');

-- Randomly generated sample links per page
INSERT INTO links (title, url, github_username) VALUES
  ('salon',   'http://howard.com',         'Smokey'),
  ('result',  'https://www.achieved.com',  'Smokey'),
  ('owns',    'http://www.robinson.com',   'Smokey');

INSERT INTO links (title, url, github_username) VALUES  
  ('illustration',  'https://www.asus.com',   'Noodle'),
  ('boulevard',     'http://www.pierre.com',  'Noodle'),
  ('towers',        'http://www.hb.com',      'Noodle'),
  ('dome',          'https://mem.com',        'Noodle'),
  ('parts',         'https://needle.com',     'Noodle');

INSERT INTO links (title, url, github_username) VALUES 
  ('vt',        'http://athletics.inf.cu',     'Loki'),
  ('muscle',    'http://www.furnishings.com',  'Loki'),
  ('studying',  'http://www.mention.com',      'Loki'),
  ('survey',    'https://www.stationery.com',  'Loki'),
  ('overall',   'http://comparable.com',       'Loki'),
  ('cargo',     'https://colleague.com',       'Loki');
