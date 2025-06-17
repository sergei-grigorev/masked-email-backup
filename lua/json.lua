local _G = require("_G")

-- Local variables for script state
local table_format = {}  -- Field names to include in output
local total_records_count = 0  -- Total number of records to process
local current_record_index = 0  -- Current record being processed

-- Function that will be called first to initialize the script environment.
-- @param format_description Table describing records format
_G.prepare = function(format_description)
    -- Define fields to include in JSON output
    table_format[1] = "email"
    table_format[2] = "description"
    table_format[3] = "web_site"
    table_format[4] = "state"
    table_format[5] = "created_at"
    
    -- Reset record counter
    current_record_index = 0
end

-- Function that will be called before processing records to output header information.
-- @param records_count Total number of records to be processed
-- @return Header string for JSON format
_G.header = function(records_count)
    -- Store total records for comma handling
    total_records_count = records_count
    return '{ "records": ['
end

-- Function to transform each email record to JSON format.
-- @param record Table containing the email record data
-- @return Formatted JSON string for this record
_G.next = function(record)
    -- Increment record counter
    current_record_index = current_record_index + 1
    
    -- Build JSON object for this record
    local output = "{"
    
    -- Add each field to the JSON object
    for i = 1, #table_format do
        local field_name = table_format[i]
        local field_value = record[field_name] or ""
        
        -- Format as "field": "value"
        local field_json = string.format('"%s": "%s"', field_name, field_value)
        
        -- Add comma between fields if not the last field
        if i < #table_format then
            output = output .. field_json .. ","
        else
            output = output .. field_json
        end
    end
    
    -- Close the object and add comma if not the last record
    if current_record_index < total_records_count then
        return output .. "},"
    else
        return output .. "}"
    end
end

-- Function that will be called after all records are processed.
-- @return Footer string for JSON format
_G.footer = function()
    return "]}"
end
