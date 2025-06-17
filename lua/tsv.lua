local _G = require("_G")

-- Local variables for script state
local table_format = {}  -- Field names to include in output

-- Function that will be called first to initialize the script environment.
-- @param format_description Table describing records format
_G.prepare = function(format_description)
    -- Define fields to include in TSV output
    table_format[1] = "email"
    table_format[2] = "description"
    table_format[3] = "web_site"
    table_format[4] = "state"
    table_format[5] = "created_at"
end

-- Function that will be called before processing records to output header information.
-- @param records_count Total number of records to be processed
-- @return Header string containing tab-separated field names
_G.header = function(records_count)
    -- Create header with field names
    local parts = {}
    
    -- Add each field name to the parts table
    for i = 1, #table_format do
        parts[i] = table_format[i]
    end
    
    -- Join with tabs and add newline
    return table.concat(parts, "\t") .. "\n"
end

-- Function to transform each email record to TSV format.
-- @param record Table containing the email record data
-- @return Formatted TSV string for this record
_G.next = function(record)
    -- Create array to hold field values
    local parts = {}
    
    -- Add each field value to the parts table
    for i = 1, #table_format do
        local field_name = table_format[i]
        parts[i] = record[field_name] or ""
    end
    
    -- Join with tabs and add newline
    return table.concat(parts, "\t") .. "\n"
end

-- Function that will be called after all records are processed.
-- @return Footer string (empty for TSV format)
_G.footer = function()
    return ""
end
