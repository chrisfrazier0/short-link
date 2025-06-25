-- Create Links Table
CREATE TABLE links (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  code CHAR(3) NOT NULL,
  url TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Function to handle updated_at
CREATE FUNCTION links_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to call function
CREATE TRIGGER trigger_links_update_at
BEFORE UPDATE ON links
FOR EACH ROW EXECUTE FUNCTION links_updated_at();
