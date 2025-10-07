# Deploying to Cloudflare Pages

This guide explains how to deploy the `at-peek` WASM app to Cloudflare Pages.

**Production URL**: https://peek.bsky.nz  
**Cloudflare URL**: https://at-peek.pages.dev

## ✅ Compatibility

This app is **fully compatible** with Cloudflare Pages because it's a client-side WASM application with no server-side requirements.

## 🚀 Automatic Deployment (Recommended)

The repository is configured with GitHub Actions to automatically deploy on every push to `main`.

### What's Automated

✅ **Rust compilation** with release optimizations  
✅ **WASM optimization** with wasm-opt (-Oz flag)  
✅ **Debug stripping** for smaller bundles  
✅ **Cloudflare Pages deployment** to production  
✅ **Deployment summary** with bundle sizes and URLs  

### Quick Setup

1. **Add GitHub Secrets** (Settings → Secrets and variables → Actions):
   - `CLOUDFLARE_API_TOKEN` - [Get from Cloudflare Dashboard](https://dash.cloudflare.com/profile/api-tokens)
   - `CLOUDFLARE_ACCOUNT_ID` - [Find in Cloudflare Dashboard](https://dash.cloudflare.com)

2. **Push to main branch**:
   ```bash
   git push origin main
   ```

3. **Watch deployment** in GitHub Actions tab

4. **Configure custom domain** (first deployment only - see Custom Domain Setup below)

### Deployment Workflow

On every push to `main`:

1. **Test** - Runs formatting, clippy, and tests
2. **Build** - Compiles WASM in release mode
3. **Optimize** - Applies wasm-opt for 20-30% size reduction
4. **Deploy** - Uploads to Cloudflare Pages
5. **Summary** - Shows bundle sizes and deployment URLs

**Expected build time**: 3-5 minutes

**Deployment URLs**:
- Production: https://peek.bsky.nz (after custom domain setup)
- Cloudflare: https://at-peek.pages.dev
- Preview: Each deployment gets a unique URL

For detailed setup instructions, see [`.github/DEPLOYMENT_SETUP.md`](.github/DEPLOYMENT_SETUP.md)

---

## 📦 Manual Deployment

## Prerequisites

1. A [Cloudflare account](https://dash.cloudflare.com/sign-up)
2. Rust toolchain with `wasm32-unknown-unknown` target
3. Trunk build tool: `cargo install trunk --locked`

## Build for Production

### Quick Build

```bash
cd crates/at-peek-web
trunk build --release
```

### Optimized Build (Recommended)

For maximum optimization, use `wasm-opt` after building:

```bash
cd crates/at-peek-web

# Build with trunk
trunk build --release

# Install wasm-opt (if not already installed)
npm install -g wasm-opt

# Optimize the WASM file
cd dist
WASM_FILE=$(ls *.wasm)
echo "Before optimization: $(ls -lh $WASM_FILE)"

wasm-opt -Oz --strip-debug --strip-producers -o "${WASM_FILE}.opt" "${WASM_FILE}"
mv "${WASM_FILE}.opt" "${WASM_FILE}"

echo "After optimization: $(ls -lh $WASM_FILE)"
```

**Optimization flags explained:**
- `-Oz`: Aggressive size optimization (can reduce bundle by 10-30%)
- `--strip-debug`: Remove debug information
- `--strip-producers`: Remove build tool metadata

This creates optimized output in `crates/at-peek-web/dist/`:
- `index.html` - Entry point
- `*.js` - JavaScript bindings
- `*.wasm` - WebAssembly binary (optimized with wasm-opt)

## Deployment Options

### Option 1: Direct Upload (Wrangler CLI)

1. Install Wrangler:
   ```bash
   npm install -g wrangler
   ```

2. Login to Cloudflare:
   ```bash
   wrangler login
   ```

3. Deploy:
   ```bash
   cd crates/at-peek-web
   wrangler pages deploy dist --project-name=at-peek
   ```

   This deploys to: `https://at-peek.pages.dev`

4. (Optional) Add custom domain:
   ```bash
   wrangler pages domain add at-peek your-domain.com
   ```

### Option 2: GitHub Integration (Recommended)

1. Push your code to GitHub
2. Go to [Cloudflare Dashboard](https://dash.cloudflare.com) → Workers & Pages
3. Click "Create application" → "Pages" → "Connect to Git"
4. Select your repository
5. Configure build settings:
   - **Build command**: `cd crates/at-peek-web && trunk build --release`
   - **Build output directory**: `crates/at-peek-web/dist`
   - **Root directory**: `/` (or leave empty)
   - **Environment variables**: (none required)

6. Click "Save and Deploy"

### Option 3: Drag & Drop

1. Build the app locally: `cd crates/at-peek-web && trunk build --release`
2. Go to Cloudflare Dashboard → Workers & Pages
3. Click "Create application" → "Pages" → "Upload assets"
4. Drag and drop the `crates/at-peek-web/dist` folder

## Configuration

### Custom Domain Setup (`peek.bsky.nz`)

Cloudflare automatically provides: `https://at-peek.pages.dev`  
Custom domain configured: `https://peek.bsky.nz`

#### Step-by-Step Setup

**1. Deploy Your Site First**

Make sure your site is deployed to Cloudflare Pages (either via GitHub Actions or manual deployment).

**2. Add Custom Domain via Cloudflare Dashboard**

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Navigate to **Workers & Pages** from the left sidebar
3. Click on your **at-peek** project
4. Click the **Custom domains** tab at the top
5. Click **Set up a custom domain** button
6. Enter: `peek.bsky.nz`
7. Click **Continue**

**3. DNS Configuration**

Cloudflare will show you what DNS records need to be added.

**Option A: If `bsky.nz` is already on Cloudflare (Recommended):**

✅ Cloudflare will automatically add the required DNS records for you!

Just click **Activate domain** and you're done. DNS propagates in seconds to minutes.

**Option B: If `bsky.nz` uses external DNS:**

You'll need to manually add a CNAME record at your DNS provider:

```
Type:     CNAME
Name:     peek
Target:   at-peek.pages.dev
TTL:      Auto (or 3600)
Proxy:    No (if using Cloudflare DNS, you can enable proxy after)
```

**4. Verify Setup**

After DNS propagates (usually 1-5 minutes for Cloudflare DNS, up to 24 hours for external):

```bash
# Check DNS resolution
dig peek.bsky.nz

# Or
nslookup peek.bsky.nz
```

You should see it pointing to Cloudflare's infrastructure.

**5. Enable HTTPS**

Cloudflare Pages automatically provisions SSL certificates for custom domains. This takes a few minutes after DNS is configured.

Visit `https://peek.bsky.nz` - you should see your site with a valid SSL certificate!

#### Managing Custom Domains

**View Current Domains:**
```bash
wrangler pages project list
```

**Remove Custom Domain:**

Via Dashboard:
1. Go to Workers & Pages → at-peek → Custom domains
2. Click the three dots next to `peek.bsky.nz`
3. Click **Remove domain**

#### Troubleshooting

**"Domain is already associated with another project"**
- The domain might be configured on another Cloudflare Pages project
- Remove it from the other project first, or use a different subdomain

**"DNS record not found"**
- Wait a few minutes for DNS propagation
- Check your DNS provider has the correct CNAME record
- Verify there are no conflicting A/AAAA records for the same subdomain

**"SSL certificate provisioning failed"**
- Wait up to 15 minutes - certificate provisioning can take time
- Ensure DNS is correctly configured and propagated
- Try removing and re-adding the custom domain

**Site loads but shows SSL error**
- Clear your browser cache
- Wait for certificate provisioning to complete
- Check in Cloudflare Dashboard → Workers & Pages → at-peek → Custom domains
  - Should show "Active" with a green checkmark

**DNS propagation taking too long**
- If using Cloudflare DNS: propagation is usually instant
- If using external DNS: can take up to 24 hours
- Check propagation status: https://dnschecker.org/#CNAME/peek.bsky.nz

### Environment Variables

This app doesn't require any environment variables or secrets since:
- All API calls are made from the browser
- Authentication uses user-provided credentials
- No backend/server-side processing

## CORS & Security

✅ **No CORS issues**: The app makes API calls directly from the browser to:
- `mod.bsky.app` (Bluesky moderation service)
- `bsky.social` (ATProto PDS)
- `plc.directory` (DID resolution)

All these services support CORS for browser requests.

## Bundle Size

Current WASM bundle size is approximately **6.3MB** (debug) or **~2-3MB** (release).

To check your release bundle size:
```bash
cd crates/at-peek-web/dist
ls -lh *.wasm
```

Cloudflare Pages limits:
- ✅ Max file size: 25 MB per file
- ✅ Max deployment size: 25,000 files

You're well within these limits.

## Performance Optimizations

The GitHub Actions workflow automatically applies these optimizations. For manual builds:

1. **WASM optimization with wasm-opt** (Already applied in CI/CD):
   ```bash
   npm install -g wasm-opt
   cd crates/at-peek-web/dist
   
   # Find the WASM file
   WASM_FILE=$(ls *.wasm)
   
   # Optimize aggressively for size
   wasm-opt -Oz --strip-debug --strip-producers -o "${WASM_FILE}.opt" "${WASM_FILE}"
   mv "${WASM_FILE}.opt" "${WASM_FILE}"
   ```
   
   **Expected results:**
   - Before optimization: ~700-800 KB
   - After optimization: ~500-600 KB (20-30% reduction)

2. **Enable Cloudflare caching**:
   - WASM files are automatically cached by Cloudflare CDN
   - Configure cache headers in `_headers` file (see below)

3. **Create a `_headers` file** in `dist/`:
   ```
   /*.wasm
     Cache-Control: public, max-age=31536000, immutable
   
   /*.js
     Cache-Control: public, max-age=31536000, immutable
   
   /index.html
     Cache-Control: public, max-age=0, must-revalidate
   ```

## CI/CD Integration

The GitHub workflow already builds the WASM app. To add automatic deployment:

1. Get a Cloudflare API token
2. Add these secrets to your GitHub repo:
   - `CLOUDFLARE_API_TOKEN`
   - `CLOUDFLARE_ACCOUNT_ID`

3. Add this to `.github/workflows/ci.yml`:
   ```yaml
   deploy:
     name: Deploy to Cloudflare Pages
     runs-on: ubuntu-latest
     needs: build-wasm
     if: github.ref == 'refs/heads/main'
     steps:
       - uses: actions/checkout@v4
       
       - name: Install Rust
         uses: dtolnay/rust-toolchain@stable
         with:
           targets: wasm32-unknown-unknown
       
       - name: Install trunk
         run: cargo install trunk --locked
       
       - name: Build
         run: |
           cd crates/at-peek-web
           trunk build --release
       
       - name: Deploy to Cloudflare Pages
         uses: cloudflare/pages-action@v1
         with:
           apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
           accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
           projectName: at-peek
           directory: crates/at-peek-web/dist
           gitHubToken: ${{ secrets.GITHUB_TOKEN }}
   ```

## Troubleshooting

### WASM fails to load

- Check browser console for errors
- Ensure WASM file is served with correct MIME type (`application/wasm`)
- Cloudflare Pages handles this automatically

### Large bundle size

- Verify you're building with `--release` flag
- Check dependencies - consider feature flags to reduce size
- Use `wasm-opt` for additional optimization

### 404 errors

- Cloudflare Pages serves `index.html` for all routes by default
- This works perfectly for Leptos client-side routing

## Resources

- [Cloudflare Pages Docs](https://developers.cloudflare.com/pages/)
- [Trunk Build Tool](https://trunkrs.dev/)
- [Leptos Framework](https://leptos.dev/)

