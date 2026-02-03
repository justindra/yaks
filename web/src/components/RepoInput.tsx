import { useState } from 'preact/hooks';
import { getRecentRepos } from '../lib/auth/config';

interface RepoInputProps {
  onSubmit: (url: string) => void;
  isLoading: boolean;
  isConnected: boolean;
}

export function RepoInput({ onSubmit, isLoading, isConnected }: RepoInputProps) {
  const [url, setUrl] = useState('');
  const [showRecent, setShowRecent] = useState(false);
  const recentRepos = getRecentRepos();

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    if (url.trim()) {
      onSubmit(url.trim());
      setShowRecent(false);
    }
  };

  const handleSelectRecent = (repoUrl: string) => {
    setUrl(repoUrl);
    setShowRecent(false);
    onSubmit(repoUrl);
  };

  return (
    <form onSubmit={handleSubmit} class="flex items-center gap-sm" style={{ position: 'relative' }}>
      <div style={{ position: 'relative', minWidth: '300px' }}>
        <input
          type="text"
          value={url}
          onInput={(e) => setUrl((e.target as HTMLInputElement).value)}
          onFocus={() => setShowRecent(true)}
          onBlur={() => setTimeout(() => setShowRecent(false), 200)}
          placeholder="owner/repo or https://github.com/..."
          disabled={isLoading}
          style={{ width: '100%' }}
        />

        {/* Recent repos dropdown */}
        {showRecent && recentRepos.length > 0 && (
          <div
            class="bg-secondary border rounded"
            style={{
              position: 'absolute',
              top: '100%',
              left: 0,
              right: 0,
              marginTop: '4px',
              zIndex: 50,
              maxHeight: '200px',
              overflowY: 'auto',
            }}
          >
            <div class="text-xs text-muted p-sm border-b">Recent repositories</div>
            {recentRepos.map((repo) => (
              <button
                key={repo}
                type="button"
                onClick={() => handleSelectRecent(repo)}
                class="w-full text-left p-sm text-sm"
                style={{
                  background: 'transparent',
                  border: 'none',
                  cursor: 'pointer',
                }}
                onMouseEnter={(e) => {
                  (e.target as HTMLElement).style.background = 'var(--color-bg-tertiary)';
                }}
                onMouseLeave={(e) => {
                  (e.target as HTMLElement).style.background = 'transparent';
                }}
              >
                {repo.replace('https://github.com/', '')}
              </button>
            ))}
          </div>
        )}
      </div>

      <button type="submit" disabled={isLoading || !url.trim()} class="primary">
        {isLoading ? (
          <>
            <span class="spinner" />
            Loading...
          </>
        ) : isConnected ? (
          'Reconnect'
        ) : (
          'Connect'
        )}
      </button>
    </form>
  );
}
