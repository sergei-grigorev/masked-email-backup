local _G = require("_G")

-- Local variables for script state (none needed for this simple script)

-- Function that will be called first to initialize the script environment.
-- @param format_description Table describing records format
_G.prepare = function(format_description) 
    -- No initialization needed for this simple output format
end

-- Function that will be called before processing records to output header information.
-- @param records_count Total number of records to be processed
-- @return Header string (empty for this format)
_G.header = function(records_count)
    return ""
end

-- Function to transform each email record to output text.
-- Only outputs enabled email addresses, one per line.
-- @param record Table containing the email record data
-- @return Formatted email string with newline or empty string
_G.next = function(record)
    -- Only output enabled email addresses
    if record.state == "enabled" then
        return record.email .. "\n"
    else
        return ""
    end
end

-- Function that will be called after all records are processed.
-- @return Footer string (empty for this format)
_G.footer = function()
    return ""
end
