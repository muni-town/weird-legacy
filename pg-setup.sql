-- Setup script for the database

-- Links that are created by the user will be stored in this TABLE
create TABLE links (
    url TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    github_username TEXT NOT NULL
);

-- Randomly generated sample links per page
INSERT INTO links (url, title, github_username) VALUES
  ('http://howard.com',         'salon',   'Smokey'),
  ('https://www.achieved.com',  'result',  'Smokey'),
  ('http://www.robinson.com',   'owns',    'Smokey');

INSERT INTO links (url, title, github_username) VALUES  
  ('https://www.asus.com',   'illustration',  'Noodle'),
  ('http://www.pierre.com',  'boulevard',     'Noodle'),
  ('http://www.hb.com',      'towers',        'Noodle'),
  ('https://mem.com',        'dome',          'Noodle'),
  ('https://needle.com',     'parts',         'Noodle');

INSERT INTO links (url, title, github_username) VALUES 
  ('http://athletics.inf.cu',     'vt',        'Loki'),
  ('http://www.furnishings.com',  'muscle',    'Loki'),
  ('http://www.mention.com',      'studying',  'Loki'),
  ('https://www.stationery.com',  'survey',    'Loki'),
  ('http://comparable.com',       'overall',   'Loki'),
  ('https://colleague.com',       'cargo',     'Loki');
