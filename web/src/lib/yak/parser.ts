import type { YakMap } from './types';
import { createEmptyYakMap, createYak, getLeafName, getParentPath } from './types';
import type { GitHubClient, GitHubTreeEntry } from '../git/client';

/**
 * Parses a GitHub tree (from recursive API call) into a YakMap.
 * 
 * The tree entries have paths like:
 * - "yak-name/.yak" (marker file, always present)
 * - "yak-name/done" (marks yak as done)
 * - "yak-name/context.md" (yak context)
 * - "parent/child/done" (nested yak)
 */
export async function parseGitHubTree(
  client: GitHubClient,
  treeEntries: GitHubTreeEntry[],
  token?: string
): Promise<YakMap> {
  const yakMap = createEmptyYakMap();
  
  // Track which paths are yaks (directories) and their metadata
  const yakPaths = new Set<string>();
  const donePaths = new Set<string>();
  const contextBlobs = new Map<string, string>(); // yakPath -> blob SHA
  
  // First pass: identify yak directories and their metadata
  for (const entry of treeEntries) {
    const parts = entry.path.split('/');
    
    if (entry.type === 'blob') {
      const fileName = parts[parts.length - 1];
      const yakPath = parts.slice(0, -1).join('/');
      
      if (fileName === '.yak' && yakPath) {
        // .yak marker file - this directory is a yak
        yakPaths.add(yakPath);
      } else if (fileName === 'done' && yakPath) {
        donePaths.add(yakPath);
        yakPaths.add(yakPath);
      } else if (fileName === 'context.md' && yakPath) {
        contextBlobs.set(yakPath, entry.sha);
        yakPaths.add(yakPath);
      }
    } else if (entry.type === 'tree') {
      // A directory is a yak (for backward compatibility with trees without .yak markers)
      yakPaths.add(entry.path);
    }
  }
  
  // Fetch context content for yaks that have it
  const contextContents = new Map<string, string>();
  const contextPromises = Array.from(contextBlobs.entries()).map(
    async ([yakPath, sha]) => {
      try {
        const content = await client.getBlob(sha, token);
        contextContents.set(yakPath, content.trim());
      } catch {
        // Ignore errors fetching context
      }
    }
  );
  await Promise.all(contextPromises);
  
  // Second pass: create yaks
  for (const yakPath of yakPaths) {
    const name = getLeafName(yakPath);
    const parentId = getParentPath(yakPath);
    const done = donePaths.has(yakPath);
    const context = contextContents.get(yakPath) || null;
    
    const yak = createYak(yakPath, name, parentId, done, context);
    yakMap.yaks.set(yakPath, yak);
  }
  
  // Build roots list
  yakMap.roots = Array.from(yakMap.yaks.values())
    .filter(yak => yak.parentId === null)
    .map(yak => yak.id)
    .sort();
  
  return yakMap;
}

/**
 * Parses a flat list of file paths into a YakMap.
 * Useful for testing or alternative data sources.
 * 
 * Format:
 * - "yak-name/.yak" - marker file (always present)
 * - "yak-name/done" - marks yak as done
 * - "yak-name/context.md" with content - sets context
 * - "parent/child/..." - creates hierarchy
 */
export function parseFromPaths(
  paths: { path: string; content?: string }[]
): YakMap {
  const yakMap = createEmptyYakMap();
  const yakPaths = new Set<string>();
  const doneSet = new Set<string>();
  const contextMap = new Map<string, string>();
  
  // First pass: collect all yak paths and their metadata
  for (const { path, content } of paths) {
    const parts = path.split('/');
    
    // Check if this is a metadata file
    const lastPart = parts[parts.length - 1];
    if (lastPart === '.yak') {
      // .yak marker file - this directory is a yak
      const yakPath = parts.slice(0, -1).join('/');
      if (yakPath) {
        yakPaths.add(yakPath);
      }
    } else if (lastPart === 'done') {
      // Mark parent path as done
      const yakPath = parts.slice(0, -1).join('/');
      if (yakPath) {
        doneSet.add(yakPath);
        yakPaths.add(yakPath);
      }
    } else if (lastPart === 'context.md') {
      // Store context for parent path
      const yakPath = parts.slice(0, -1).join('/');
      if (yakPath && content) {
        contextMap.set(yakPath, content.trim());
        yakPaths.add(yakPath);
      }
    } else {
      // This is a directory path - add all ancestor paths as yaks
      let currentPath = '';
      for (const part of parts) {
        currentPath = currentPath ? `${currentPath}/${part}` : part;
        yakPaths.add(currentPath);
      }
    }
  }
  
  // Second pass: create yaks
  for (const yakPath of yakPaths) {
    const name = getLeafName(yakPath);
    const parentId = getParentPath(yakPath);
    const done = doneSet.has(yakPath);
    const context = contextMap.get(yakPath) ?? null;
    
    const yak = createYak(yakPath, name, parentId, done, context);
    yakMap.yaks.set(yakPath, yak);
  }
  
  // Build roots list
  yakMap.roots = Array.from(yakMap.yaks.values())
    .filter(yak => yak.parentId === null)
    .map(yak => yak.id)
    .sort();
  
  return yakMap;
}
