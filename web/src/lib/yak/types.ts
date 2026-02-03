/**
 * Represents a single yak (task) in the yak map.
 * Matches the domain model from the Rust CLI.
 */
export interface Yak {
  /** Full path identifier, e.g., "parent/child/grandchild" */
  id: string;
  
  /** The leaf name only, e.g., "grandchild" */
  name: string;
  
  /** Parent yak ID, or null if root */
  parentId: string | null;
  
  /** Whether this yak is marked as done */
  done: boolean;
  
  /** Optional context/notes stored in context.md */
  context: string | null;
}

/**
 * The complete yak map containing all yaks and their relationships.
 */
export interface YakMap {
  /** Map of yak ID to Yak object */
  yaks: Map<string, Yak>;
  
  /** IDs of root-level yaks (no parent) */
  roots: string[];
}

/**
 * Git tree entry for parsing/serializing yak directories.
 */
export interface TreeEntry {
  /** Entry name (file or directory name) */
  name: string;
  
  /** Git object type: 'tree' for directory, 'blob' for file */
  type: 'tree' | 'blob';
  
  /** Git object ID (SHA) */
  oid: string;
  
  /** File mode (40000 for tree, 100644 for blob) */
  mode: string;
}

/**
 * Author information for git commits.
 */
export interface Author {
  name: string;
  email: string;
}

/**
 * Result of a sync operation.
 */
export interface SyncResult {
  /** Whether there were conflicts that were auto-resolved */
  hadConflicts: boolean;
  
  /** Number of local changes that were pushed */
  localChanges: number;
  
  /** Number of remote changes that were pulled */
  remoteChanges: number;
  
  /** The new commit SHA if a commit was created */
  commitSha?: string;
}

/**
 * Creates an empty YakMap.
 */
export function createEmptyYakMap(): YakMap {
  return {
    yaks: new Map(),
    roots: [],
  };
}

/**
 * Creates a new Yak with the given properties.
 */
export function createYak(
  id: string,
  name: string,
  parentId: string | null = null,
  done: boolean = false,
  context: string | null = null
): Yak {
  return { id, name, parentId, done, context };
}

/**
 * Gets all children of a yak.
 */
export function getChildren(yakMap: YakMap, parentId: string): Yak[] {
  return Array.from(yakMap.yaks.values()).filter(yak => yak.parentId === parentId);
}

/**
 * Gets all descendants of a yak (children, grandchildren, etc.).
 */
export function getDescendants(yakMap: YakMap, parentId: string): Yak[] {
  const descendants: Yak[] = [];
  const children = getChildren(yakMap, parentId);
  
  for (const child of children) {
    descendants.push(child);
    descendants.push(...getDescendants(yakMap, child.id));
  }
  
  return descendants;
}

/**
 * Gets the parent path from a yak ID.
 * e.g., "a/b/c" -> "a/b"
 */
export function getParentPath(id: string): string | null {
  const lastSlash = id.lastIndexOf('/');
  return lastSlash === -1 ? null : id.substring(0, lastSlash);
}

/**
 * Gets the leaf name from a yak ID.
 * e.g., "a/b/c" -> "c"
 */
export function getLeafName(id: string): string {
  const lastSlash = id.lastIndexOf('/');
  return lastSlash === -1 ? id : id.substring(lastSlash + 1);
}

/**
 * Checks if a yak has any incomplete children.
 */
export function hasIncompleteChildren(yakMap: YakMap, yakId: string): boolean {
  const children = getChildren(yakMap, yakId);
  return children.some(child => !child.done);
}

/**
 * Checks if a yak can be marked as done.
 * A yak cannot be done if it has incomplete children.
 */
export function canMarkDone(yakMap: YakMap, yakId: string): boolean {
  return !hasIncompleteChildren(yakMap, yakId);
}

/**
 * Deep clones a YakMap.
 */
export function cloneYakMap(yakMap: YakMap): YakMap {
  const clonedYaks = new Map<string, Yak>();
  for (const [id, yak] of yakMap.yaks) {
    clonedYaks.set(id, { ...yak });
  }
  return {
    yaks: clonedYaks,
    roots: [...yakMap.roots],
  };
}
