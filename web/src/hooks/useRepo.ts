import { signal, computed } from '@preact/signals';
import { GitHubClient, parseGitHubUrl, toGitHubUrl } from '../lib/git/client';
import { addRecentRepo } from '../lib/auth/config';

// Global repo state
const client = signal<GitHubClient | null>(null);
const repoUrl = signal<string | null>(null);
const repoOwner = signal<string | null>(null);
const repoName = signal<string | null>(null);
const isLoading = signal(false);
const error = signal<string | null>(null);

// Computed states
const isConnected = computed(() => client.value !== null);
const displayName = computed(() => {
  if (repoOwner.value && repoName.value) {
    return `${repoOwner.value}/${repoName.value}`;
  }
  return null;
});

/**
 * Hook for repository connection state and actions.
 */
export function useRepo() {
  /**
   * Connects to a GitHub repository.
   * Returns the client if successful, null otherwise.
   */
  const connect = async (url: string, _token?: string): Promise<GitHubClient | null> => {
    // Parse the URL
    const parsed = parseGitHubUrl(url);
    if (!parsed) {
      error.value = 'Invalid GitHub repository URL. Use format: owner/repo or https://github.com/owner/repo';
      return null;
    }

    isLoading.value = true;
    error.value = null;

    try {
      // Create the GitHub client
      const newClient = new GitHubClient(parsed.owner, parsed.repo);

      // Update state
      client.value = newClient;
      repoUrl.value = toGitHubUrl(parsed.owner, parsed.repo);
      repoOwner.value = parsed.owner;
      repoName.value = parsed.repo;

      // Add to recent repos
      addRecentRepo(toGitHubUrl(parsed.owner, parsed.repo));
      
      return newClient;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to connect to repository';
      client.value = null;
      repoUrl.value = null;
      repoOwner.value = null;
      repoName.value = null;
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Disconnects from the current repository.
   */
  const disconnect = () => {
    client.value = null;
    repoUrl.value = null;
    repoOwner.value = null;
    repoName.value = null;
    error.value = null;
  };

  return {
    // State
    client,
    repoUrl,
    repoOwner,
    repoName,
    isLoading,
    error,
    isConnected,
    displayName,
    
    // Actions
    connect,
    disconnect,
  };
}
