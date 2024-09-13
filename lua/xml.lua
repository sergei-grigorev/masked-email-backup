local _G = require("_G")

local table_format = {}

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
_G.header = function()
	-- print all columns
	return "<records>"
end

-- Function to transform each email record to output text.
-- Input: table record
_G.next = function(record)
	-- print all columns
	local output = "<record>"
	for i = 1, #table_format do
		local field_name = table_format[i]
		local v = record[field_name] or ""
		local full = "<" .. field_name .. ">" .. v .. "</" .. field_name .. ">"

		output = output .. full
	end

	return output .. "</record>"
end

-- Function that will be called after all records.
-- No input
_G.footer = function()
	return "</records>"
end