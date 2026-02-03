import type { YakMap } from './types';
import type { GitHubClient } from '../git/client';

/**
 * Entry for creating a tree via GitHub API.
 */
export interface TreeCreateEntry {
  path: string;
  mode: '100644' | '040000';
  type: 'blob' | 'tree';
  sha: string;
}

/**
 * Serializes a YakMap to GitHub tree entries.
 * Creates blobs for each yak: .yak marker, optional done file, optional context.md.
 */
export async function serializeYakMap(
  client: GitHubClient,
  yakMap: YakMap,
  token: string
): Promise<TreeCreateEntry[]> {
  const entries: TreeCreateEntry[] = [];
  
  // Create a single empty blob SHA to reuse for marker files
  // (GitHub deduplicates blobs by content, so this is efficient)
  const emptyBlobSha = await client.createBlob('', token);
  
  // Create blobs and entries for each yak
  for (const yak of yakMap.yaks.values()) {
    // Always add .yak marker file to ensure the directory exists
    entries.push({
      path: `${yak.id}/.yak`,
      mode: '100644',
      type: 'blob',
      sha: emptyBlobSha,
    });
    
    // Add done marker if done
    if (yak.done) {
      entries.push({
        path: `${yak.id}/done`,
        mode: '100644',
        type: 'blob',
        sha: emptyBlobSha,
      });
    }
    
    // Add context if present
    if (yak.context) {
      const contextSha = await client.createBlob(yak.context, token);
      entries.push({
        path: `${yak.id}/context.md`,
        mode: '100644',
        type: 'blob',
        sha: contextSha,
      });
    }
  }
  
  return entries;
}

/**
 * Creates a full tree from a YakMap using GitHub API.
 * Returns the SHA of the created tree.
 */
export async function createTreeFromYakMap(
  client: GitHubClient,
  yakMap: YakMap,
  token: string
): Promise<string> {
  const entries = await serializeYakMap(client, yakMap, token);
  
  // Create tree without a base (new tree from scratch)
  return client.createTree(entries, null, token);
}

/**
 * Converts a YakMap to a flat list of file paths.
 * Useful for debugging or alternative storage.
 */
export function toFilePaths(yakMap: YakMap): { path: string; content?: string }[] {
  const paths: { path: string; content?: string }[] = [];
  
  for (const yak of yakMap.yaks.values()) {
    // Always add .yak marker
    paths.push({ path: `${yak.id}/.yak` });
    
    // Add done marker if done
    if (yak.done) {
      paths.push({ path: `${yak.id}/done` });
    }
    
    // Add context if present
    if (yak.context) {
      paths.push({ path: `${yak.id}/context.md`, content: yak.context });
    }
  }
  
  // Sort paths for consistent output
  paths.sort((a, b) => a.path.localeCompare(b.path));
  
  return paths;
}
