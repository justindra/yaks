/**
 * GitHub REST API client for git operations.
 * Replaces isomorphic-git to avoid CORS proxy issues.
 */

const GITHUB_API = 'https://api.github.com';

/**
 * GitHub API error with rate limit info.
 */
export class GitHubAPIError extends Error {
  constructor(
    message: string,
    public status: number,
    public rateLimitRemaining?: number,
    public rateLimitReset?: Date
  ) {
    super(message);
    this.name = 'GitHubAPIError';
  }

  static fromResponse(response: Response, message: string): GitHubAPIError {
    const remaining = response.headers.get('x-ratelimit-remaining');
    const reset = response.headers.get('x-ratelimit-reset');

    return new GitHubAPIError(
      message,
      response.status,
      remaining ? parseInt(remaining, 10) : undefined,
      reset ? new Date(parseInt(reset, 10) * 1000) : undefined
    );
  }

  get isRateLimited(): boolean {
    return this.status === 403 && this.rateLimitRemaining === 0;
  }

  get userMessage(): string {
    if (this.isRateLimited && this.rateLimitReset) {
      const resetTime = this.rateLimitReset.toLocaleTimeString();
      return `GitHub API rate limit exceeded. Try again after ${resetTime}, or login for higher limits.`;
    }
    if (this.status === 404) {
      return 'Repository not found or refs/notes/yaks doesn\'t exist yet.';
    }
    if (this.status === 401) {
      return 'Authentication required or token expired.';
    }
    return this.message;
  }
}

/**
 * GitHub API tree entry.
 */
export interface GitHubTreeEntry {
  path: string;
  mode: string;
  type: 'blob' | 'tree';
  sha: string;
  size?: number;
}

/**
 * GitHub API commit object.
 */
export interface GitHubCommit {
  sha: string;
  tree: {
    sha: string;
  };
  parents: Array<{ sha: string }>;
  message: string;
  author: {
    name: string;
    email: string;
    date: string;
  };
}

/**
 * GitHub REST API client for a specific repository.
 */
export class GitHubClient {
  private owner: string;
  private repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  /**
   * Gets the owner of the repository.
   */
  getOwner(): string {
    return this.owner;
  }

  /**
   * Gets the repository name.
   */
  getRepo(): string {
    return this.repo;
  }

  /**
   * Makes an authenticated request to the GitHub API.
   */
  private async request<T>(
    endpoint: string,
    options: RequestInit = {},
    token?: string
  ): Promise<T> {
    const url = `${GITHUB_API}/repos/${this.owner}/${this.repo}${endpoint}`;

    const headers: Record<string, string> = {
      'Accept': 'application/vnd.github.v3+json',
      ...((options.headers as Record<string, string>) || {}),
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      let message = `GitHub API error: ${response.status}`;
      try {
        const data = await response.json();
        message = data.message || message;
      } catch {
        // Ignore JSON parse errors
      }
      throw GitHubAPIError.fromResponse(response, message);
    }

    return response.json();
  }

  /**
   * Gets a git reference (e.g., refs/notes/yaks).
   * Returns the SHA of the commit, or null if not found.
   */
  async getRef(ref: string, token?: string): Promise<string | null> {
    try {
      const data = await this.request<{ object: { sha: string } }>(
        `/git/ref/${ref}`,
        {},
        token
      );
      return data.object.sha;
    } catch (error) {
      if (error instanceof GitHubAPIError && error.status === 404) {
        return null;
      }
      throw error;
    }
  }

  /**
   * Gets a commit object.
   */
  async getCommit(sha: string, token?: string): Promise<GitHubCommit> {
    return this.request<GitHubCommit>(`/git/commits/${sha}`, {}, token);
  }

  /**
   * Gets a tree object.
   * If recursive is true, returns all entries in the tree recursively.
   */
  async getTree(
    sha: string,
    recursive = false,
    token?: string
  ): Promise<GitHubTreeEntry[]> {
    const params = recursive ? '?recursive=1' : '';
    const data = await this.request<{ tree: GitHubTreeEntry[] }>(
      `/git/trees/${sha}${params}`,
      {},
      token
    );
    return data.tree;
  }

  /**
   * Gets a blob's content (decoded from base64).
   */
  async getBlob(sha: string, token?: string): Promise<string> {
    const data = await this.request<{ content: string; encoding: string }>(
      `/git/blobs/${sha}`,
      {},
      token
    );

    if (data.encoding === 'base64') {
      return atob(data.content.replace(/\n/g, ''));
    }

    return data.content;
  }

  /**
   * Creates a new blob.
   * Returns the SHA of the created blob.
   */
  async createBlob(content: string, token: string): Promise<string> {
    const data = await this.request<{ sha: string }>(
      '/git/blobs',
      {
        method: 'POST',
        body: JSON.stringify({
          content,
          encoding: 'utf-8',
        }),
      },
      token
    );
    return data.sha;
  }

  /**
   * Creates a new tree.
   * Returns the SHA of the created tree.
   */
  async createTree(
    entries: Array<{
      path: string;
      mode: '100644' | '040000';
      type: 'blob' | 'tree';
      sha: string;
    }>,
    baseTree: string | null,
    token: string
  ): Promise<string> {
    const body: Record<string, unknown> = { tree: entries };
    if (baseTree) {
      body.base_tree = baseTree;
    }

    const data = await this.request<{ sha: string }>(
      '/git/trees',
      {
        method: 'POST',
        body: JSON.stringify(body),
      },
      token
    );
    return data.sha;
  }

  /**
   * Creates a new commit.
   * Returns the SHA of the created commit.
   */
  async createCommit(
    message: string,
    tree: string,
    parents: string[],
    author: { name: string; email: string },
    token: string
  ): Promise<string> {
    const data = await this.request<{ sha: string }>(
      '/git/commits',
      {
        method: 'POST',
        body: JSON.stringify({
          message,
          tree,
          parents,
          author: {
            name: author.name,
            email: author.email,
            date: new Date().toISOString(),
          },
        }),
      },
      token
    );
    return data.sha;
  }

  /**
   * Updates a reference to point to a new commit.
   * Creates the ref if it doesn't exist.
   */
  async updateRef(ref: string, sha: string, token: string): Promise<void> {
    // First check if the ref exists
    const existingRef = await this.getRef(ref, token);
    
    if (existingRef === null) {
      // Ref doesn't exist, create it
      await this.request(
        '/git/refs',
        {
          method: 'POST',
          body: JSON.stringify({ ref: `refs/${ref}`, sha }),
        },
        token
      );
    } else {
      // Ref exists, update it
      try {
        await this.request(
          `/git/refs/${ref}`,
          {
            method: 'PATCH',
            body: JSON.stringify({ sha, force: false }),
          },
          token
        );
      } catch (error) {
        if (error instanceof GitHubAPIError && error.status === 422) {
          // Non-fast-forward update - need to force or handle conflict
          throw new GitHubAPIError(
            'Push rejected: remote has changes. Pull first to merge.',
            422
          );
        }
        throw error;
      }
    }
  }

  /**
   * Force updates a reference (use with caution).
   */
  async forceUpdateRef(ref: string, sha: string, token: string): Promise<void> {
    try {
      await this.request(
        `/git/refs/${ref}`,
        {
          method: 'PATCH',
          body: JSON.stringify({ sha, force: true }),
        },
        token
      );
    } catch (error) {
      if (error instanceof GitHubAPIError && error.status === 404) {
        // Ref doesn't exist, create it
        await this.request(
          '/git/refs',
          {
            method: 'POST',
            body: JSON.stringify({ ref: `refs/${ref}`, sha }),
          },
          token
        );
      } else {
        throw error;
      }
    }
  }
}

/**
 * Parses a GitHub repository URL into owner and repo.
 */
export function parseGitHubUrl(url: string): { owner: string; repo: string } | null {
  // Handle various formats:
  // - https://github.com/owner/repo
  // - https://github.com/owner/repo.git
  // - github.com/owner/repo
  // - owner/repo

  let normalized = url.trim();

  // Remove .git suffix
  if (normalized.endsWith('.git')) {
    normalized = normalized.slice(0, -4);
  }

  // Handle full URLs
  const urlMatch = normalized.match(/(?:https?:\/\/)?github\.com\/([^/]+)\/([^/]+)/);
  if (urlMatch) {
    return { owner: urlMatch[1], repo: urlMatch[2] };
  }

  // Handle owner/repo format
  const shortMatch = normalized.match(/^([^/]+)\/([^/]+)$/);
  if (shortMatch) {
    return { owner: shortMatch[1], repo: shortMatch[2] };
  }

  return null;
}

/**
 * Converts a parsed GitHub repo to a full HTTPS URL.
 */
export function toGitHubUrl(owner: string, repo: string): string {
  return `https://github.com/${owner}/${repo}`;
}
