#!/bin/bash
# Multiple isolated browser sessions (for parallel workers)

# Session 1: User A
z-agent-browser --session userA open "https://example.com"
z-agent-browser --session userA snapshot -i

# Session 2: User B (completely isolated)
z-agent-browser --session userB open "https://example.org"
z-agent-browser --session userB snapshot -i

# Each session has its own cookies, storage, history
echo "=== Session A URL ==="
z-agent-browser --session userA get url

echo "=== Session B URL ==="
z-agent-browser --session userB get url

# Clean up
z-agent-browser --session userA close
z-agent-browser --session userB close
