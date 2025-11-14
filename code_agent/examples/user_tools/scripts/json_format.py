#!/usr/bin/env python3
"""
Example user tool: json_format
Formats and validates JSON data
"""
import sys
import json

def main():
    # Read params from stdin
    params = json.loads(sys.stdin.read())

    json_string = params.get('json_string')
    indent = params.get('indent', 2)

    try:
        # Parse and format JSON
        data = json.loads(json_string)
        formatted = json.dumps(data, indent=indent, sort_keys=True)

        # Return success result
        result = {
            "success": True,
            "output": formatted
        }
    except json.JSONDecodeError as e:
        # Return error result
        result = {
            "success": False,
            "output": "",
            "error": f"Invalid JSON: {str(e)}"
        }

    # Output result as JSON
    print(json.dumps(result))

if __name__ == '__main__':
    main()
