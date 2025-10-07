#!/usr/bin/env python3
"""
check-labels.py - Check labels on Bluesky posts

Usage: python check-labels.py <post-uri-or-url>
"""

import sys
import json
import requests
import re
import os
from urllib.parse import urlparse, parse_qs
from getpass import getpass


def parse_input(input_str):
    """Parse various input formats to AT-URI"""
    
    # Already an AT-URI
    if input_str.startswith("at://"):
        return input_str
    
    # API URL format
    if "xrpc/com.atproto.repo.getRecord" in input_str:
        parsed = urlparse(input_str)
        params = parse_qs(parsed.query)
        
        repo = params.get('repo', [None])[0]
        collection = params.get('collection', [None])[0]
        rkey = params.get('rkey', [None])[0]
        
        if repo and collection and rkey:
            return f"at://{repo}/{collection}/{rkey}"
        raise ValueError("Could not parse API URL")
    
    # Bluesky web URL format
    if "bsky.app" in input_str:
        match = re.search(r'profile/([^/]+)/post/([^/?\s]+)', input_str)
        if match:
            handle, rkey = match.groups()
            
            # Resolve handle to DID
            print(f"🔍 Resolving handle {handle}...")
            resp = requests.get(
                f"https://bsky.social/xrpc/com.atproto.identity.resolveHandle",
                params={"handle": handle}
            )
            resp.raise_for_status()
            did = resp.json().get('did')
            
            if not did:
                raise ValueError("Could not resolve handle to DID")
            
            return f"at://{did}/app.bsky.feed.post/{rkey}"
        
        raise ValueError("Could not parse Bluesky URL")
    
    raise ValueError("Unrecognized input format")


def create_session(handle, password):
    """Create authenticated session"""
    resp = requests.post(
        "https://bsky.social/xrpc/com.atproto.server.createSession",
        json={
            "identifier": handle,
            "password": password
        }
    )
    resp.raise_for_status()
    return resp.json()['accessJwt']


def get_post(at_uri, access_token=None):
    """Fetch post with labels"""
    headers = {}
    if access_token:
        headers['Authorization'] = f'Bearer {access_token}'
    
    resp = requests.get(
        "https://bsky.social/xrpc/app.bsky.feed.getPosts",
        params={"uris": at_uri},
        headers=headers
    )
    resp.raise_for_status()
    return resp.json()


def main():
    if len(sys.argv) < 2:
        print("Usage: python check-labels.py <post-uri-or-url>")
        print()
        print("Examples:")
        print("  python check-labels.py at://did:plc:abc123/app.bsky.feed.post/3lylub2qvq22i")
        print("  python check-labels.py https://bsky.app/profile/handle.bsky.social/post/3lylub2qvq22i")
        print()
        print("Environment variables (optional):")
        print("  BLUESKY_HANDLE - Your Bluesky handle")
        print("  BLUESKY_APP_PASSWORD - Your app password")
        sys.exit(1)
    
    input_str = sys.argv[1]
    
    try:
        at_uri = parse_input(input_str)
        print(f"📍 Checking labels for: {at_uri}")
        print()
        
        # Try unauthenticated first
        print("🔓 Attempting unauthenticated request...")
        access_token = None
        
        try:
            data = get_post(at_uri)
        except requests.HTTPError as e:
            if e.response.status_code == 401:
                print("🔐 Authentication required. Logging in...")
                
                # Get credentials
                handle = os.environ.get('BLUESKY_HANDLE')
                if not handle:
                    handle = input("Bluesky handle: ")
                
                password = os.environ.get('BLUESKY_APP_PASSWORD')
                if not password:
                    password = getpass("App password: ")
                
                # Create session
                access_token = create_session(handle, password)
                print("✅ Authentication successful!")
                print()
                
                # Retry with auth
                data = get_post(at_uri, access_token)
            else:
                raise
        
        # Check if post exists
        posts = data.get('posts', [])
        if not posts:
            print("❌ Post not found")
            sys.exit(1)
        
        post = posts[0]
        labels = post.get('labels', [])
        
        print("✅ Post found")
        print()
        
        # Display post info
        author = post['author']['handle']
        text = post['record'].get('text', '')[:100]
        print(f"👤 Author: {author}")
        print(f"📝 Text: {text}...")
        print()
        
        # Display labels
        if labels:
            print(f"⚠️  Found {len(labels)} label(s):")
            print()
            for label in labels:
                val = label.get('val', 'unknown')
                src = label.get('src', 'unknown')
                print(f"  • {val} (from: {src})")
        else:
            print("✅ No labels found")
        
        print()
        print("📊 Full label data:")
        print(json.dumps(labels, indent=2))
        
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()

