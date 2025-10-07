#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
#
# check-labels.sh - Check content moderation labels for ATproto posts and users
#
# Usage:
#   ./check-labels.sh <post-url|at-uri|did>
#
# Examples:
#   ./check-labels.sh "https://bsky.app/profile/alice.bsky.social/post/abc123"
#   ./check-labels.sh "at://did:plc:xyz/app.bsky.feed.post/abc123"
#   ./check-labels.sh "did:plc:xyz"

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check dependencies
check_dependencies() {
    local missing=()
    
    if ! command -v curl &> /dev/null; then
        missing+=("curl")
    fi
    
    if ! command -v jq &> /dev/null; then
        missing+=("jq")
    fi
    
    if [ ${#missing[@]} -ne 0 ]; then
        echo -e "${RED}Error: Missing required dependencies: ${missing[*]}${NC}" >&2
        echo "Install with: brew install ${missing[*]} (macOS) or apt-get install ${missing[*]} (Linux)" >&2
        exit 1
    fi
}

# Resolve handle to DID
resolve_handle() {
    local handle="$1"
    local did
    
    echo -e "${BLUE}🔍 Resolving handle to DID...${NC}" >&2
    
    # Try Bluesky resolver
    local response
    response=$(curl -s "https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle=${handle}")
    did=$(echo "$response" | jq -r '.did // empty')
    
    if [ -z "$did" ]; then
        echo -e "${RED}Error: Failed to resolve handle: $handle${NC}" >&2
        exit 1
    fi
    
    echo "$did"
}

# Parse Bluesky URL to AT-URI
parse_bsky_url() {
    local url="$1"
    
    # Extract handle and rkey from URL
    # Format: https://bsky.app/profile/HANDLE/post/RKEY
    local handle rkey
    handle=$(echo "$url" | sed -n 's|.*/profile/\([^/]*\)/post.*|\1|p')
    rkey=$(echo "$url" | sed -n 's|.*/post/\([^/?]*\).*|\1|p')
    
    if [ -z "$handle" ] || [ -z "$rkey" ]; then
        echo -e "${RED}Error: Invalid Bluesky URL format${NC}" >&2
        exit 1
    fi
    
    local did
    did=$(resolve_handle "$handle")
    
    echo "at://${did}/app.bsky.feed.post/${rkey}"
}

# Parse API URL to extract repo and rkey
parse_api_url() {
    local url="$1"
    
    # Format: https://bsky.social/xrpc/com.atproto.repo.getRecord?repo=DID&collection=...&rkey=RKEY
    local repo rkey collection
    repo=$(echo "$url" | sed -n 's/.*repo=\([^&]*\).*/\1/p')
    rkey=$(echo "$url" | sed -n 's/.*rkey=\([^&]*\).*/\1/p')
    collection=$(echo "$url" | sed -n 's/.*collection=\([^&]*\).*/\1/p')
    
    if [ -z "$repo" ] || [ -z "$rkey" ]; then
        echo -e "${RED}Error: Invalid API URL format${NC}" >&2
        exit 1
    fi
    
    # Default to post collection if not specified
    if [ -z "$collection" ]; then
        collection="app.bsky.feed.post"
    fi
    
    echo "at://${repo}/${collection}/${rkey}"
}

# Normalize input to AT-URI or DID
normalize_input() {
    local input="$1"
    
    # Already an AT-URI
    if [[ $input =~ ^at:// ]]; then
        echo "$input"
    # DID (user-level labels)
    elif [[ $input =~ ^did: ]]; then
        echo "$input"
    # Bluesky URL
    elif [[ $input =~ ^https://bsky\.app/profile/ ]]; then
        parse_bsky_url "$input"
    # API URL
    elif [[ $input =~ ^https://.*xrpc/com\.atproto\.repo\.getRecord ]]; then
        parse_api_url "$input"
    # Handle (convert to DID)
    elif [[ $input =~ \. ]]; then
        resolve_handle "$input"
    else
        echo -e "${RED}Error: Unrecognized input format${NC}" >&2
        echo "Expected: AT-URI, DID, Bluesky URL, or handle" >&2
        exit 1
    fi
}

# Get authentication token if credentials provided
get_auth_token() {
    local handle="${BLUESKY_HANDLE:-}"
    local password="${BLUESKY_APP_PASSWORD:-}"
    
    # Prompt if not set
    if [ -n "$handle" ] && [ -z "$password" ]; then
        echo -e "${YELLOW}Enter app password for $handle:${NC}" >&2
        read -rs password
        echo >&2
    fi
    
    if [ -z "$handle" ] || [ -z "$password" ]; then
        echo ""
        return
    fi
    
    echo -e "${BLUE}🔐 Authenticating...${NC}" >&2
    
    local response
    response=$(curl -s -X POST "https://bsky.social/xrpc/com.atproto.server.createSession" \
        -H "Content-Type: application/json" \
        -d "{\"identifier\":\"$handle\",\"password\":\"$password\"}")
    
    local access_token
    access_token=$(echo "$response" | jq -r '.accessJwt // empty')
    
    if [ -z "$access_token" ]; then
        echo -e "${YELLOW}Warning: Authentication failed, proceeding without auth${NC}" >&2
        echo ""
    else
        echo "$access_token"
    fi
}

# Query labels from labeler service
query_labels() {
    local subject="$1"
    local auth_token="${2:-}"
    local labeler_url="https://mod.bsky.app"
    
    echo -e "${BLUE}🏷️  Querying labels from $labeler_url...${NC}" >&2
    
    local encoded_subject
    encoded_subject=$(echo -n "$subject" | jq -sRr @uri)
    
    local auth_header=""
    if [ -n "$auth_token" ]; then
        auth_header="Authorization: Bearer $auth_token"
    fi
    
    local response
    if [ -n "$auth_header" ]; then
        response=$(curl -s -H "$auth_header" \
            "${labeler_url}/xrpc/com.atproto.label.queryLabels?uriPatterns=${encoded_subject}")
    else
        response=$(curl -s \
            "${labeler_url}/xrpc/com.atproto.label.queryLabels?uriPatterns=${encoded_subject}")
    fi
    
    echo "$response"
}

# Display labels with formatting
display_labels() {
    local response="$1"
    local subject="$2"
    
    # Check for errors
    local error
    error=$(echo "$response" | jq -r '.error // empty')
    if [ -n "$error" ]; then
        echo -e "${RED}❌ Error: $error${NC}"
        local message
        message=$(echo "$response" | jq -r '.message // empty')
        if [ -n "$message" ]; then
            echo -e "${RED}   $message${NC}"
        fi
        return 1
    fi
    
    # Extract labels
    local label_count
    label_count=$(echo "$response" | jq '.labels | length')
    
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}📋 Label Report${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Subject:${NC} $subject"
    echo -e "${BLUE}Labels found:${NC} $label_count"
    echo ""
    
    if [ "$label_count" -eq 0 ]; then
        echo -e "${GREEN}✅ No moderation labels applied${NC}"
        echo ""
        return 0
    fi
    
    # Group labels by category
    local adult_labels violence_labels spam_labels hate_labels moderation_labels other_labels
    
    adult_labels=$(echo "$response" | jq -r '.labels[] | select(.val | test("^(porn|sexual|nudity)$")) | .val' | sort -u)
    violence_labels=$(echo "$response" | jq -r '.labels[] | select(.val | test("^(graphic-media|gore)$")) | .val' | sort -u)
    spam_labels=$(echo "$response" | jq -r '.labels[] | select(.val == "spam") | .val' | sort -u)
    hate_labels=$(echo "$response" | jq -r '.labels[] | select(.val == "hate") | .val' | sort -u)
    moderation_labels=$(echo "$response" | jq -r '.labels[] | select(.val | startswith("!")) | .val' | sort -u)
    other_labels=$(echo "$response" | jq -r '.labels[] | select(.val | test("^(porn|sexual|nudity|graphic-media|gore|spam|hate|!)") | not) | .val' | sort -u)
    
    # Display by category
    if [ -n "$adult_labels" ]; then
        echo -e "${RED}🔞 Adult Content${NC}"
        echo "$adult_labels" | while read -r label; do
            display_label_details "$response" "$label"
        done
        echo ""
    fi
    
    if [ -n "$violence_labels" ]; then
        echo -e "${YELLOW}⚠️  Violence & Gore${NC}"
        echo "$violence_labels" | while read -r label; do
            display_label_details "$response" "$label"
        done
        echo ""
    fi
    
    if [ -n "$spam_labels" ]; then
        echo -e "${YELLOW}🚫 Spam${NC}"
        display_label_details "$response" "spam"
        echo ""
    fi
    
    if [ -n "$hate_labels" ]; then
        echo -e "${RED}🛑 Hate & Harassment${NC}"
        display_label_details "$response" "hate"
        echo ""
    fi
    
    if [ -n "$moderation_labels" ]; then
        echo -e "${MAGENTA}👁️  Moderation Actions${NC}"
        echo "$moderation_labels" | while read -r label; do
            display_label_details "$response" "$label"
        done
        echo ""
    fi
    
    if [ -n "$other_labels" ]; then
        echo -e "${CYAN}🏷️  Other Labels${NC}"
        echo "$other_labels" | while read -r label; do
            display_label_details "$response" "$label"
        done
        echo ""
    fi
}

# Display details for a specific label
display_label_details() {
    local response="$1"
    local label_val="$2"
    
    echo "$response" | jq -r --arg val "$label_val" \
        '.labels[] | select(.val == $val) | 
        "  • \(.val)\n    Source: \(.src)\n    Created: \(.cts)\(.exp // "" | if . != "" then "\n    Expires: \(.)" else "" end)"'
}

# Main function
main() {
    if [ $# -eq 0 ]; then
        echo "Usage: $0 <post-url|at-uri|did|handle>" >&2
        echo "" >&2
        echo "Examples:" >&2
        echo "  $0 'https://bsky.app/profile/alice.bsky.social/post/abc123'" >&2
        echo "  $0 'at://did:plc:xyz/app.bsky.feed.post/abc123'" >&2
        echo "  $0 'did:plc:xyz'" >&2
        echo "  $0 'alice.bsky.social'" >&2
        exit 1
    fi
    
    check_dependencies
    
    local input="$1"
    local subject
    subject=$(normalize_input "$input")
    
    local auth_token
    auth_token=$(get_auth_token)
    
    local response
    response=$(query_labels "$subject" "$auth_token")
    
    display_labels "$response" "$subject"
}

main "$@"
