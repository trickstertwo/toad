#!/bin/bash
# Test script to verify graceful shutdown

echo "Testing Toad TUI graceful shutdown..."
echo "This will run the TUI for 2 seconds, then send Ctrl+C"
echo "Watch for any terminal corruption or errors"
echo ""
echo "Press Enter to start..."
read

# Run toad in the background
timeout 2s cargo run --release &
PID=$!

# Wait for it to start
sleep 0.5

# Send Ctrl+C
kill -INT $PID 2>/dev/null

# Wait for it to finish
wait $PID 2>/dev/null

# Check terminal state
echo ""
echo "Testing terminal state after shutdown:"
echo "Can you see this message clearly? (terminal should be restored)"
echo "Try typing: "
read -p "Type 'ok' if terminal works: " response

if [ "$response" = "ok" ]; then
    echo "✓ Graceful shutdown test PASSED"
    exit 0
else
    echo "✗ Terminal may be corrupted"
    exit 1
fi
