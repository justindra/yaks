import type { YakMap } from './types';
import { hasIncompleteChildren, getChildren } from './types';

/**
 * Characters that are forbidden in yak names.
 * These are filesystem-unsafe characters.
 */
const FORBIDDEN_CHARS = /[\\:*?|<>"]/;

/**
 * Reserved names that cannot be used for yaks.
 */
const RESERVED_NAMES = ['', '.', '..', 'done', 'context.md'];

/**
 * Validates a yak name.
 * Returns null if valid, or an error message if invalid.
 */
export function validateYakName(name: string): string | null {
  if (!name || name.trim() === '') {
    return 'Yak name cannot be empty';
  }

  const trimmed = name.trim();

  // Check for forbidden characters
  if (FORBIDDEN_CHARS.test(trimmed)) {
    return 'Yak name contains forbidden characters: \\ : * ? | < > "';
  }

  // Check for reserved names
  const parts = trimmed.split('/');
  for (const part of parts) {
    if (RESERVED_NAMES.includes(part)) {
      return `"${part}" is a reserved name and cannot be used`;
    }
    
    // Check each part for forbidden chars
    if (FORBIDDEN_CHARS.test(part)) {
      return 'Yak name contains forbidden characters: \\ : * ? | < > "';
    }

    // Check for empty parts (double slashes)
    if (part === '') {
      return 'Yak name cannot contain empty path segments (double slashes)';
    }

    // Check for leading/trailing whitespace in parts
    if (part !== part.trim()) {
      return 'Yak name parts cannot have leading or trailing whitespace';
    }
  }

  // Check for leading/trailing slashes
  if (trimmed.startsWith('/') || trimmed.endsWith('/')) {
    return 'Yak name cannot start or end with a slash';
  }

  return null;
}

/**
 * Validates that a yak can be marked as done.
 * Returns null if valid, or an error message if invalid.
 */
export function validateMarkDone(
  yakMap: YakMap,
  yakId: string,
  recursive: boolean = false
): string | null {
  const yak = yakMap.yaks.get(yakId);
  if (!yak) {
    return `Yak "${yakId}" not found`;
  }

  if (yak.done) {
    return null; // Already done, nothing to validate
  }

  if (!recursive && hasIncompleteChildren(yakMap, yakId)) {
    return `Cannot mark "${yakId}" as done because it has incomplete children. Use recursive mode to mark all children as done.`;
  }

  return null;
}

/**
 * Validates that a yak can be deleted.
 * Returns null if valid, or an error message if invalid.
 */
export function validateDelete(yakMap: YakMap, yakId: string): string | null {
  const yak = yakMap.yaks.get(yakId);
  if (!yak) {
    return `Yak "${yakId}" not found`;
  }

  // Check if has children
  const children = getChildren(yakMap, yakId);
  if (children.length > 0) {
    return `Cannot delete "${yakId}" because it has ${children.length} child(ren). Delete children first or use recursive delete.`;
  }

  return null;
}

/**
 * Validates that a yak can be moved to a new parent.
 * Returns null if valid, or an error message if invalid.
 */
export function validateMove(
  yakMap: YakMap,
  yakId: string,
  newParentId: string | null
): string | null {
  const yak = yakMap.yaks.get(yakId);
  if (!yak) {
    return `Yak "${yakId}" not found`;
  }

  // If moving to root, that's always valid
  if (newParentId === null) {
    return null;
  }

  // Check that new parent exists
  const newParent = yakMap.yaks.get(newParentId);
  if (!newParent) {
    return `Parent yak "${newParentId}" not found`;
  }

  // Check that we're not moving to self
  if (yakId === newParentId) {
    return 'Cannot move a yak to be its own child';
  }

  // Check that we're not moving to a descendant (would create cycle)
  let current: string | null = newParentId;
  while (current !== null) {
    if (current === yakId) {
      return 'Cannot move a yak to be a descendant of itself (would create cycle)';
    }
    const currentYak = yakMap.yaks.get(current);
    current = currentYak?.parentId ?? null;
  }

  // Check that the new ID doesn't already exist
  const newId = newParentId + '/' + yak.name;
  if (yakMap.yaks.has(newId) && newId !== yakId) {
    return `A yak with name "${yak.name}" already exists under "${newParentId}"`;
  }

  return null;
}

/**
 * Validates that a new yak can be added.
 * Returns null if valid, or an error message if invalid.
 */
export function validateAdd(
  yakMap: YakMap,
  name: string,
  parentId: string | null
): string | null {
  // Validate the name itself
  const nameError = validateYakName(name);
  if (nameError) {
    return nameError;
  }

  // If parent specified, check it exists
  if (parentId !== null && !yakMap.yaks.has(parentId)) {
    return `Parent yak "${parentId}" not found`;
  }

  // Check that the full ID doesn't already exist
  const fullId = parentId ? `${parentId}/${name}` : name;
  if (yakMap.yaks.has(fullId)) {
    return `A yak with name "${fullId}" already exists`;
  }

  return null;
}
