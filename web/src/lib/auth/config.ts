/**
 * LocalStorage keys for configuration.
 */
export const STORAGE_KEYS = {
  GITHUB_TOKEN: 'yak-map-github-token',
  GITHUB_USERNAME: 'yak-map-github-username',
  RECENT_REPOS: 'yak-map-recent-repos',
} as const;

/**
 * Gets the stored GitHub token.
 */
export function getStoredToken(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem(STORAGE_KEYS.GITHUB_TOKEN);
}

/**
 * Sets the GitHub token.
 */
export function setStoredToken(token: string | null): void {
  if (typeof window === 'undefined') return;
  if (token) {
    localStorage.setItem(STORAGE_KEYS.GITHUB_TOKEN, token);
  } else {
    localStorage.removeItem(STORAGE_KEYS.GITHUB_TOKEN);
  }
}

/**
 * Gets the stored GitHub username.
 */
export function getStoredUsername(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem(STORAGE_KEYS.GITHUB_USERNAME);
}

/**
 * Sets the GitHub username.
 */
export function setStoredUsername(username: string | null): void {
  if (typeof window === 'undefined') return;
  if (username) {
    localStorage.setItem(STORAGE_KEYS.GITHUB_USERNAME, username);
  } else {
    localStorage.removeItem(STORAGE_KEYS.GITHUB_USERNAME);
  }
}

/**
 * Gets the list of recent repositories.
 */
export function getRecentRepos(): string[] {
  if (typeof window === 'undefined') return [];
  try {
    const stored = localStorage.getItem(STORAGE_KEYS.RECENT_REPOS);
    return stored ? JSON.parse(stored) : [];
  } catch {
    return [];
  }
}

/**
 * Adds a repository to the recent repos list.
 */
export function addRecentRepo(repoUrl: string): void {
  if (typeof window === 'undefined') return;
  const repos = getRecentRepos().filter((r) => r !== repoUrl);
  repos.unshift(repoUrl);
  // Keep only the last 10 repos
  const trimmed = repos.slice(0, 10);
  localStorage.setItem(STORAGE_KEYS.RECENT_REPOS, JSON.stringify(trimmed));
}

/**
 * Clears all stored data.
 */
export function clearAllStoredData(): void {
  if (typeof window === 'undefined') return;
  Object.values(STORAGE_KEYS).forEach((key) => {
    localStorage.removeItem(key);
  });
}
