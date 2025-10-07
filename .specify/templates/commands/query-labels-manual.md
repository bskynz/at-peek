# Command: Manually Query Labels (for Testing)

**Purpose:** Query ATproto labels manually using curl/HTTP

**Category:** Testing, Debugging

---

## Query Labels for a DID

```bash
# Bluesky's labeler service
LABELER_URL="https://mod.bsky.app"
DID="did:plc:example123"

curl -X GET "${LABELER_URL}/xrpc/com.atproto.label.queryLabels?uriPatterns=${DID}" \
  -H "Accept: application/json" | jq
```

---

## Query Labels for a Post (AT-URI)

```bash
LABELER_URL="https://mod.bsky.app"
AT_URI="at://did:plc:example123/app.bsky.feed.post/abc123"

# URL-encode the AT-URI
ENCODED_URI=$(echo -n "$AT_URI" | jq -sRr @uri)

curl -X GET "${LABELER_URL}/xrpc/com.atproto.label.queryLabels?uriPatterns=${ENCODED_URI}" \
  -H "Accept: application/json" | jq
```

---

## Resolve Handle to DID

```bash
HANDLE="alice.bsky.social"

# Method 1: DNS TXT lookup
dig +short TXT "_atproto.${HANDLE}" | grep "did="

# Method 2: HTTPS well-known
curl -s "https://${HANDLE}/.well-known/atproto-did"
```

---

## Example Full Workflow

```bash
#!/bin/bash
# Query labels for a Bluesky user

HANDLE="$1"
if [ -z "$HANDLE" ]; then
  echo "Usage: $0 <handle>"
  exit 1
fi

echo "🔍 Resolving handle to DID..."
DID=$(curl -s "https://${HANDLE}/.well-known/atproto-did")
echo "DID: $DID"

echo ""
echo "🏷️ Querying labels..."
curl -s "https://mod.bsky.app/xrpc/com.atproto.label.queryLabels?uriPatterns=${DID}" | jq '.labels[]' | jq -r '"  - \(.val) (from \(.src)) at \(.cts)"'

if [ $? -ne 0 ]; then
  echo "No labels found or error occurred"
fi
```

**Usage:**
```bash
chmod +x query-labels.sh
./query-labels.sh alice.bsky.social
```

---

## Expected Response Format

```json
{
  "labels": [
    {
      "val": "spam",
      "uri": "did:plc:example123",
      "src": "did:plc:labeler123",
      "cts": "2023-10-07T12:34:56Z",
      "neg": false
    }
  ],
  "cursor": null
}
```

---

## Common Label Values

- `porn` - Pornographic content
- `sexual` - Sexually suggestive content
- `nudity` - Nudity
- `graphic-media` - Graphic violence or gore
- `spam` - Spam content
- `hate` - Hateful or discriminatory content
- `!hide` - Hide from feeds (moderation action)
- `!warn` - Show warning before viewing

---

## Troubleshooting

### CORS Error
If you get CORS errors in browser, use a CORS proxy or make requests from backend:
```bash
# Use cors-anywhere (development only)
curl https://cors-anywhere.herokuapp.com/https://mod.bsky.app/xrpc/...
```

### 404 Not Found
- Check that the DID or AT-URI is valid
- Ensure the labeler service URL is correct
- Some subjects may not have any labels (empty response is normal)

### Rate Limiting
If you get 429 responses:
- Wait for the `Retry-After` header duration
- Implement exponential backoff
- Respect rate limits (typically 3000 req/5min for Bluesky)

---

## Constitution Check

- All requests transparent (visible in network tab)
- No credentials required for public labels
- HTTPS only


