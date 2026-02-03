# Yak Map Viewer

A browser-only SPA for viewing and editing yak maps from GitHub repositories.

## Features

- **View**: Fetches `refs/notes/yaks` from any GitHub repo and renders as interactive D3.js force-directed graph
- **Edit**: Full CRUD operations - add, edit, remove yaks, commit changes, push back to origin
- **Auth**: GitHub Device Flow for private repos (no backend needed)
- **Merge**: Handles conflicts with 3-way merge (last-write-wins at yak level)

## Setup

### 1. Create GitHub OAuth App

1. Go to https://github.com/settings/developers
2. Click "New OAuth App"
3. Fill in:
   - Application name: `Yak Map Viewer`
   - Homepage URL: `https://your-username.github.io/yaks/` (or your deployment URL)
   - Authorization callback URL: `https://github.com/login/device` (not used but required)
4. Click "Register application"
5. **Enable Device Flow**: In the app settings, check "Enable Device Flow"
6. Copy your Client ID

### 2. Configure the App

Either:

- Replace the placeholder in `src/lib/auth/config.ts`:
  ```typescript
  export const DEFAULT_GITHUB_CLIENT_ID = "your-client-id-here";
  ```
- Or set it in browser localStorage:
  ```javascript
  localStorage.setItem("yak-map-github-client-id", "your-client-id-here");
  ```

## Development

```bash
# Install dependencies
bun install

# Start dev server
bun run dev

# Build for production
bun run build

# Preview production build
bun run preview

# Lint
bun run lint
```

## Usage

1. Enter a GitHub repository URL (e.g., `owner/repo` or `https://github.com/owner/repo`)
2. Click "Connect" to load the yak map
3. For private repos or to push changes, click "Login with GitHub" and complete the Device Flow

### Graph Interactions

- **Click** a node to select it (opens editor sidebar)
- **Double-click** a node to toggle its done status
- **Drag** a node onto another to reparent it
- **Drag** nodes around to reposition (visual only)

### Keyboard Shortcuts

- `Escape` - Deselect current yak

## Architecture

```
web/
├── src/
│   ├── components/        # Preact UI components
│   ├── hooks/            # State management hooks
│   └── lib/
│       ├── auth/         # GitHub OAuth Device Flow
│       ├── git/          # isomorphic-git operations
│       └── yak/          # Yak data model & validation
```

## Deployment

The app is configured to deploy to GitHub Pages at `/yaks/`. A GitHub Actions workflow (`.github/workflows/deploy-web.yml`) handles automatic deployment on push to main.

To deploy manually:

```bash
bun run build
# Upload dist/ to your hosting
```

## Tech Stack

- **Preact** + **TypeScript** - UI framework
- **Vite** - Build tool
- **isomorphic-git** - Git operations in browser
- **LightningFS** - IndexedDB-backed filesystem
- **D3.js** - Force-directed graph visualization
- **@preact/signals** - State management
