import { useState } from 'preact/hooks';

interface PATModalProps {
  isOpen: boolean;
  isLoading: boolean;
  error: string | null;
  onSave: (token: string) => Promise<boolean>;
  onClose: () => void;
}

export function PATModal({ isOpen, isLoading, error, onSave, onClose }: PATModalProps) {
  const [token, setToken] = useState('');
  const [showToken, setShowToken] = useState(false);

  if (!isOpen) return null;

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (!token.trim()) return;
    
    const success = await onSave(token.trim());
    if (success) {
      setToken('');
    }
  };

  const handleClose = () => {
    setToken('');
    onClose();
  };

  return (
    <div class="modal-overlay" onClick={handleClose}>
      <div class="modal" onClick={(e) => e.stopPropagation()}>
        <h2>GitHub Personal Access Token</h2>
        
        <p class="text-sm text-muted mb-md">
          To edit yaks, you need a GitHub Personal Access Token.
        </p>

        <div class="mb-md">
          <p class="text-sm mb-sm"><strong>Create a token with the appropriate scope:</strong></p>
          <ul class="text-sm text-muted" style={{ paddingLeft: '1.5rem', marginBottom: '0.5rem' }}>
            <li style={{ marginBottom: '0.25rem' }}>
              <a 
                href="https://github.com/settings/tokens/new?scopes=repo&description=Yaks%20Editor" 
                target="_blank" 
                rel="noopener noreferrer"
              >
                <code>repo</code> scope
              </a>
              {' '}- for private and public repositories
            </li>
            <li>
              <a 
                href="https://github.com/settings/tokens/new?scopes=public_repo&description=Yaks%20Editor" 
                target="_blank" 
                rel="noopener noreferrer"
              >
                <code>public_repo</code> scope
              </a>
              {' '}- for public repositories only
            </li>
          </ul>
        </div>

        <div class="bg-tertiary rounded p-sm mb-md text-sm">
          <strong>Note:</strong> Your token will be stored in your browser's localStorage. 
          You can clear it anytime by logging out or clearing browser data.
        </div>

        <form onSubmit={handleSubmit}>
          <div class="mb-md">
            <label class="text-sm" style={{ display: 'block', marginBottom: '0.25rem' }}>
              Personal Access Token
            </label>
            <div style={{ position: 'relative' }}>
              <input
                type={showToken ? 'text' : 'password'}
                value={token}
                onInput={(e) => setToken((e.target as HTMLInputElement).value)}
                placeholder="ghp_xxxxxxxxxxxxxxxxxxxx"
                class="w-full"
                style={{ paddingRight: '4rem' }}
                disabled={isLoading}
                autoFocus
              />
              <button
                type="button"
                onClick={() => setShowToken(!showToken)}
                class="small"
                style={{ 
                  position: 'absolute', 
                  right: '0.25rem', 
                  top: '50%', 
                  transform: 'translateY(-50%)',
                  padding: '0.25rem 0.5rem',
                  fontSize: '0.75rem'
                }}
              >
                {showToken ? 'Hide' : 'Show'}
              </button>
            </div>
          </div>

          {error && (
            <div class="text-sm text-error mb-md">
              {error}
            </div>
          )}

          <div class="flex gap-sm" style={{ justifyContent: 'flex-end' }}>
            <button type="button" onClick={handleClose} disabled={isLoading}>
              Cancel
            </button>
            <button type="submit" class="primary" disabled={isLoading || !token.trim()}>
              {isLoading ? (
                <>
                  <span class="spinner" />
                  Validating...
                </>
              ) : (
                'Save Token'
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
