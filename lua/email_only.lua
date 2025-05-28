local _G = require("_G")

-- Function that will be called first.
-- Input: table describing records format
_G.prepare = function(_) end

-- Function that will be called before [[next]].
-- If the output format need headers that function should return it.
-- Input: total records count for emails
_G.header = function()
	return ""
end

-- Function to transform each email record to output text.
-- Input: table record
_G.next = function(record)
	if record.state == "enabled" then
		return record.email .. "\n"
	else
		return ""
	end
end

-- Function that will be called after all records.
-- No input
_G.footer = function()
	return ""
end
