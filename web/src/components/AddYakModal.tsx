import { useState } from 'preact/hooks';
import type { YakMap } from '../lib/yak/types';
import { validateYakName } from '../lib/yak/validation';

interface AddYakModalProps {
  yakMap: YakMap;
  selectedParentId: string | null;
  onAdd: (name: string, parentId?: string) => void;
  onClose: () => void;
}

export function AddYakModal({ yakMap, selectedParentId, onAdd, onClose }: AddYakModalProps) {
  const [name, setName] = useState('');
  const [parentId, setParentId] = useState<string>(selectedParentId ?? '');
  const [error, setError] = useState<string | null>(null);

  // Get all yaks for parent dropdown
  const allYaks = Array.from(yakMap.yaks.values()).sort((a, b) => a.id.localeCompare(b.id));

  const handleSubmit = (e: Event) => {
    e.preventDefault();

    // Validate name
    const validationError = validateYakName(name);
    if (validationError) {
      setError(validationError);
      return;
    }

    // Check if ID already exists
    const fullId = parentId ? `${parentId}/${name}` : name;
    if (yakMap.yaks.has(fullId)) {
      setError(`A yak with name "${fullId}" already exists`);
      return;
    }

    onAdd(name, parentId || undefined);
  };

  return (
    <div class="modal-overlay" onClick={onClose}>
      <div class="modal-content" onClick={(e) => e.stopPropagation()}>
        <h2 style={{ marginTop: 0 }}>Add New Yak</h2>

        <form onSubmit={handleSubmit}>
          {/* Name input */}
          <div class="mb-md">
            <label class="text-sm text-muted mb-sm" style={{ display: 'block' }}>
              Name
            </label>
            <input
              type="text"
              value={name}
              onInput={(e) => {
                setName((e.target as HTMLInputElement).value);
                setError(null);
              }}
              placeholder="my-task"
              autoFocus
            />
            <p class="text-xs text-muted mt-sm">
              Use letters, numbers, hyphens, underscores. Use / for hierarchy.
            </p>
          </div>

          {/* Parent dropdown */}
          <div class="mb-md">
            <label class="text-sm text-muted mb-sm" style={{ display: 'block' }}>
              Parent (optional)
            </label>
            <select
              value={parentId}
              onChange={(e) => setParentId((e.target as HTMLSelectElement).value)}
              style={{
                width: '100%',
                padding: 'var(--spacing-sm) var(--spacing-md)',
                backgroundColor: 'var(--color-bg-secondary)',
                border: '1px solid var(--color-border)',
                borderRadius: 'var(--radius-md)',
                color: 'var(--color-text)',
              }}
            >
              <option value="">(Root level)</option>
              {allYaks.map((yak) => (
                <option key={yak.id} value={yak.id}>
                  {yak.id}
                </option>
              ))}
            </select>
          </div>

          {/* Preview */}
          {name && (
            <div class="mb-md p-sm bg-tertiary rounded">
              <span class="text-sm text-muted">Will create: </span>
              <span class="font-mono text-sm">
                {parentId ? `${parentId}/${name}` : name}
              </span>
            </div>
          )}

          {/* Error */}
          {error && (
            <div class="mb-md text-error text-sm">
              {error}
            </div>
          )}

          {/* Buttons */}
          <div class="flex justify-end gap-sm">
            <button type="button" onClick={onClose}>
              Cancel
            </button>
            <button type="submit" class="primary" disabled={!name.trim()}>
              Add Yak
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
