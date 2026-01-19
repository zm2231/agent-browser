#!/bin/bash
# Form filling and submission

z-agent-browser open "https://httpbin.org/forms/post" --headed
z-agent-browser snapshot -i

# Fill form fields using refs from snapshot
z-agent-browser fill @e1 "John Doe"           # Customer name
z-agent-browser fill @e2 "555-1234"           # Telephone
z-agent-browser fill @e3 "john@example.com"   # Email
z-agent-browser select @e4 "medium"           # Size dropdown
z-agent-browser check @e5                      # Checkbox

# Submit
z-agent-browser click @e10                     # Submit button
z-agent-browser wait --load networkidle
z-agent-browser snapshot -i

z-agent-browser close
