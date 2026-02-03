interface AuthStatusProps {
  isAuthenticated: boolean;
  username: string | null;
  onLogout: () => void;
}

export function AuthStatus({
  isAuthenticated,
  username,
  onLogout,
}: AuthStatusProps) {
  if (!isAuthenticated) {
    return null;
  }

  return (
    <div class="flex items-center gap-sm">
      <span class="text-sm text-muted">
        Logged in as <strong>{username}</strong>
      </span>
      <button onClick={onLogout} class="small">Logout</button>
    </div>
  );
}
