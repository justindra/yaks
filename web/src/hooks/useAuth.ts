import { signal, computed } from '@preact/signals';
import { useCallback } from 'preact/hooks';
import {
  getStoredToken,
  setStoredToken,
  getStoredUsername,
  setStoredUsername,
} from '../lib/auth/config';

// Global auth state (shared across all components)
const token = signal<string | null>(getStoredToken());
const username = signal<string | null>(getStoredUsername());
const isLoading = signal(false);
const error = signal<string | null>(null);

// Modal state
const showPATModal = signal(false);

// Callback to run after successful authentication
let pendingAction: (() => void) | null = null;

// Computed states
const isAuthenticated = computed(() => token.value !== null);

/**
 * Validates that a token is valid by calling the GitHub API.
 * Returns the username if valid, null if invalid.
 */
export async function validateToken(tokenToValidate: string): Promise<string | null> {
  try {
    const response = await fetch('https://api.github.com/user', {
      headers: {
        'Authorization': `Bearer ${tokenToValidate}`,
        'Accept': 'application/vnd.github.v3+json',
      },
    });
    if (response.ok) {
      const data = await response.json();
      return data.login;
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Hook for authentication state and actions.
 */
export function useAuth() {
  const openPATModal = useCallback(() => {
    error.value = null;
    showPATModal.value = true;
  }, []);

  const closePATModal = useCallback(() => {
    showPATModal.value = false;
    error.value = null;
    pendingAction = null;
  }, []);

  const saveToken = useCallback(async (newToken: string): Promise<boolean> => {
    isLoading.value = true;
    error.value = null;

    try {
      const validatedUsername = await validateToken(newToken);
      
      if (validatedUsername) {
        token.value = newToken;
        username.value = validatedUsername;
        setStoredToken(newToken);
        setStoredUsername(validatedUsername);
        showPATModal.value = false;
        isLoading.value = false;
        
        // Run pending action if any
        if (pendingAction) {
          const action = pendingAction;
          pendingAction = null;
          action();
        }
        
        return true;
      } else {
        error.value = 'Invalid token. Please check that your token is correct and has not expired.';
        isLoading.value = false;
        return false;
      }
    } catch {
      error.value = 'Failed to validate token. Please try again.';
      isLoading.value = false;
      return false;
    }
  }, []);

  const logout = useCallback(() => {
    token.value = null;
    username.value = null;
    setStoredToken(null);
    setStoredUsername(null);
    error.value = null;
  }, []);

  /**
   * Requires authentication before performing an action.
   * If already authenticated, runs the action immediately.
   * If not authenticated, opens the PAT modal and runs the action after successful auth.
   */
  const requireAuth = useCallback((action: () => void) => {
    if (token.value) {
      // Already authenticated, run action immediately
      action();
    } else {
      // Not authenticated, store action and open modal
      pendingAction = action;
      error.value = null;
      showPATModal.value = true;
    }
  }, []);

  return {
    // State
    token,
    username,
    isLoading,
    error,
    isAuthenticated,
    showPATModal,
    
    // Actions
    openPATModal,
    closePATModal,
    saveToken,
    logout,
    requireAuth,
  };
}
