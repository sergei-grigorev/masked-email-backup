local _G = require("_G")

local table_format = {}
local total_records_count = 0
local idx = 0

-- Function that will be called first.
-- Input: table describing records format
_G.prepare = function(_)
	table_format[1] = "email"
	table_format[2] = "description"
	table_format[3] = "web_site"
	table_format[4] = "state"
	table_format[5] = "created_at"
end

-- Function that will be called before [[next]].
-- If the output format need headers that function should return it.
-- Input: total records count for emails
_G.header = function(records_count)
	-- print all columns
	total_records_count = records_count
	return '{ "records": ['
end

-- Function to transform each email record to output text.
-- Input: table record
_G.next = function(record)
	idx = idx + 1
	-- print all columns
	local output = "{"
	for i = 1, #table_format do
		local field_name = table_format[i]
		local v = record[field_name] or ""
		local full = '"' .. field_name .. '": "' .. v .. '"'

		if i < #table_format then
			output = output .. full .. ","
		else
			output = output .. full
		end
	end

	if idx < total_records_count then
		return output .. "},"
	else
		return output .. "}"
	end
end

-- Function that will be called after all records.
-- No input
_G.footer = function()
	return "]}"
end
