import { useState } from 'preact/hooks';
import type { Yak, YakMap } from '../lib/yak/types';
import { canMarkDone } from '../lib/yak/types';
import { ContextEditor } from './ContextEditor';

interface YakEditorProps {
  yak: Yak;
  yakMap: YakMap;
  onUpdate: (updates: Partial<Pick<Yak, 'context'>>) => void;
  onToggleDone: (done: boolean) => void;
  onAddChild: () => void;
  onDelete: () => void;
  onClose: () => void;
}

export function YakEditor({ 
  yak, 
  yakMap, 
  onUpdate, 
  onToggleDone,
  onAddChild,
  onDelete, 
  onClose 
}: YakEditorProps) {
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const canDone = canMarkDone(yakMap, yak.id);

  // Get children
  const children = Array.from(yakMap.yaks.values()).filter(y => y.parentId === yak.id);

  return (
    <div class="p-md flex flex-col gap-md" style={{ height: '100%' }}>
      {/* Header */}
      <div class="flex items-center justify-between">
        <h2 style={{ margin: 0 }}>{yak.name}</h2>
        <button onClick={onClose} class="small" title="Close">
          ✕
        </button>
      </div>

      {/* Path */}
      <div class="text-sm text-muted font-mono">
        {yak.id}
      </div>

      {/* Status Toggle */}
      <div class="flex flex-col gap-sm">
        <div class="flex items-center gap-sm">
          <button
            onClick={() => onToggleDone(false)}
            class={`small ${!yak.done ? 'primary' : ''}`}
            style={{ flex: 1 }}
          >
            ○ To-Do
          </button>
          <button
            onClick={() => onToggleDone(true)}
            disabled={!canDone && !yak.done}
            class={`small ${yak.done ? 'primary' : ''}`}
            style={{ flex: 1 }}
            title={!canDone && !yak.done ? 'Cannot mark done: has incomplete children' : ''}
          >
            ✓ Done
          </button>
        </div>
        {!canDone && !yak.done && (
          <span class="text-xs text-warning">
            Has incomplete children - complete them first
          </span>
        )}
      </div>

      {/* Context */}
      <div class="flex-1 flex flex-col" style={{ minHeight: '150px' }}>
        <label class="text-sm text-muted mb-sm">Context / Notes</label>
        <ContextEditor
          value={yak.context ?? ''}
          onChange={(context) => onUpdate({ context: context || null })}
        />
      </div>

      {/* Children */}
      {children.length > 0 && (
        <div class="text-sm">
          <span class="text-muted">Children: </span>
          <span>{children.length}</span>
          <ul class="mt-sm" style={{ paddingLeft: '1rem', margin: 0 }}>
            {children.slice(0, 5).map(child => (
              <li key={child.id} class={child.done ? 'text-muted' : ''}>
                {child.done ? '✓ ' : '○ '}{child.name}
              </li>
            ))}
            {children.length > 5 && (
              <li class="text-muted">...and {children.length - 5} more</li>
            )}
          </ul>
        </div>
      )}

      {/* Actions */}
      <div class="border-t pt-md flex flex-col gap-sm">
        <button onClick={onAddChild} class="small w-full">
          + Add Child Yak
        </button>
        
        {showDeleteConfirm ? (
          <div class="flex flex-col gap-sm">
            <p class="text-sm text-warning">
              {children.length > 0 
                ? `Delete "${yak.name}" and ${children.length} child(ren)?`
                : `Delete "${yak.name}"?`
              }
            </p>
            <div class="flex gap-sm">
              <button onClick={onDelete} class="danger small" style={{ flex: 1 }}>
                Delete
              </button>
              <button onClick={() => setShowDeleteConfirm(false)} class="small" style={{ flex: 1 }}>
                Cancel
              </button>
            </div>
          </div>
        ) : (
          <button onClick={() => setShowDeleteConfirm(true)} class="danger small w-full">
            Delete Yak
          </button>
        )}
      </div>
    </div>
  );
}
