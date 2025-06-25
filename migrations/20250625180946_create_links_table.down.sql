-- Drop the trigger
DROP TRIGGER IF EXISTS trigger_links_updated_at ON links;

-- Drop the table
DROP TABLE links;

-- Drop the trigger function
DROP FUNCTION links_updated_at;
