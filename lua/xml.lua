local _G = require("_G")

-- Local variables for script state
local table_format = {}  -- Field names to include in output

-- Function that will be called first to initialize the script environment.
-- @param format_description Table describing records format
_G.prepare = function(format_description)
    -- Define fields to include in XML output
    table_format[1] = "email"
    table_format[2] = "description"
    table_format[3] = "web_site"
    table_format[4] = "state"
    table_format[5] = "created_at"
end

-- Function that will be called before processing records to output header information.
-- @param records_count Total number of records to be processed
-- @return Header string for XML format
_G.header = function(records_count)
    -- Return XML root opening tag
    return "<records>"
end

-- Helper function to escape special XML characters
-- @param str String to escape
-- @return Escaped string
local function escape_xml(str)
    if not str then return "" end
    
    -- Replace XML special characters with their entity references
    local escaped = str:gsub("&", "&amp;")
    escaped = escaped:gsub("<", "&lt;")
    escaped = escaped:gsub(">", "&gt;")
    escaped = escaped:gsub('"', "&quot;")
    escaped = escaped:gsub("'", "&apos;")
    
    return escaped
end

-- Function to transform each email record to XML format.
-- @param record Table containing the email record data
-- @return Formatted XML string for this record
_G.next = function(record)
    -- Start record element
    local parts = {"<record>"}
    
    -- Add each field as an XML element
    for i = 1, #table_format do
        local field_name = table_format[i]
        local field_value = record[field_name] or ""
        
        -- Escape XML special characters
        field_value = escape_xml(field_value)
        
        -- Create XML element
        parts[#parts + 1] = string.format("<%s>%s</%s>", field_name, field_value, field_name)
    end
    
    -- Close record element
    parts[#parts + 1] = "</record>"
    
    -- Join all parts and return
    return table.concat(parts, "")
end

-- Function that will be called after all records are processed.
-- @return Footer string for XML format
_G.footer = function()
    -- Return XML root closing tag
    return "</records>"
end
