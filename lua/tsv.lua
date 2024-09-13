local _G = require("_G")

local table_format = {};

-- Function that will be called first.
-- Input: table describing records format
_G.prepare = function(meta)
    table_format = meta
end

-- Function that will be called before [[next]].
-- If the output format need headers that function should return it.
-- Input: total records count for emails
_G.header = function(total_records_count)
    -- print all columns
    local output = ""
    for k, _ in pairs(table_format) do
        if #output > 0 then
            output = output .. '\t' .. k
        else
            output = k
        end
    end

    return output..'\n'
end

-- Function to transform each email record to output text.
-- Input: table record
_G.next = function(record)
    -- print all columns
    local output = ""
    for _, v in pairs(record) do
        if #output > 0 then
            output = output .. '\t' .. v
        else
            output = v
        end
    end

    return output..'\n'
end

-- Function that will be called after all records.
-- No input
_G.footer = function()
    return ""
end
