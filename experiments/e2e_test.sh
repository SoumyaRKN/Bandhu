#!/usr/bin/env bash
set -e

BACKEND_URL="${BACKEND_URL:-http://127.0.0.1:3000}"

echo "Testing Bandhu backend at $BACKEND_URL"

echo "1. Health check..."
curl -s -X POST "$BACKEND_URL/health" | head -c 200
echo ""

echo "2. Chat endpoint..."
RESPONSE=$(curl -s -X POST "$BACKEND_URL/chat" \
  -H "Content-Type: application/json" \
  -d '{"prompt":"list files in current directory"}')
echo "$RESPONSE"
echo ""

echo "3. Approval endpoints..."
APPROVAL_ID="test-approval-1"

echo "Submitting approval decision (approved)..."
curl -s -X POST "$BACKEND_URL/approve" \
  -H "Content-Type: application/json" \
  -d "{\"request_id\":\"$APPROVAL_ID\",\"approved\":true}" | head -c 200
echo ""

echo "=== Tests complete ==="
