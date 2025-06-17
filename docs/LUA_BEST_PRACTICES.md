# Lua Best Practices for Masked Email CLI

This document outlines best practices for writing and maintaining Lua scripts in the Masked Email CLI project.

## Script Structure

Each Lua script in this project follows a consistent structure with four main functions:

1. **prepare** - Called first to set up the script environment
2. **header** - Called before processing records to output any header information
3. **next** - Called for each record to transform it into the desired output format
4. **footer** - Called after all records are processed to output any closing information

## Best Practices

### 1. Module Structure

```lua
local _G = require("_G")

-- Local variables for script state
local table_format = {}

-- Function definitions
_G.prepare = function(format_description) 
    -- Initialize script state
end

_G.header = function(records_count)
    -- Output header information
    return "header string"
end

_G.next = function(record)
    -- Process each record
    return "formatted record string"
end

_G.footer = function()
    -- Output footer information
    return "footer string"
end
```

### 2. State Management

- Use local variables at the module level to maintain state between function calls
- Initialize all state variables in the `prepare` function
- Keep track of record counts or indices when needed for formatting (e.g., for JSON comma placement)

### 3. Error Handling

- Always check for nil values with the `or` operator: `local v = record[field_name] or ""`
- Use defensive programming to handle unexpected input formats

### 4. String Formatting

- Use Lua's string concatenation (`..`) for building output strings
- For complex outputs, build strings incrementally
- Use escape sequences (`\n`, `\t`) appropriately for the output format

### 5. Output Format Consistency

- Each script should produce valid output in its target format (JSON, XML, TSV, etc.)
- Ensure proper opening and closing of format structures (brackets, tags, etc.)
- Handle edge cases like the last record in a JSON array (no trailing comma)

### 6. Code Organization

- Group related functionality together
- Use clear, descriptive variable names
- Include comments to explain the purpose of each function and complex logic

### 7. Performance Considerations

- Minimize string concatenations in loops when possible
- Use local variables for frequently accessed values
- Avoid unnecessary computations in the `next` function, which is called for every record

## Format-Specific Best Practices

### JSON Output

- Properly escape special characters in strings
- Ensure valid JSON structure with proper nesting of braces and brackets
- Handle the last element in arrays and objects correctly (no trailing comma)

### XML Output

- Use proper XML tag structure with opening and closing tags
- Consider XML escaping for special characters if needed

### TSV/CSV Output

- Handle field delimiters consistently
- Include headers when appropriate
- Consider escaping fields that contain delimiter characters

## Testing

- Test scripts with various input data, including edge cases
- Validate output format correctness with format-specific validators
