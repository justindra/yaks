import type { GitHubClient } from './client';
import type { YakMap, Author } from '../yak/types';
import { parseGitHubTree } from '../yak/parser';
import { createTreeFromYakMap } from '../yak/serializer';

/** The git ref where yaks are stored */
const YAKS_REF = 'notes/yaks';

/**
 * Fetches and parses the yak map from refs/notes/yaks.
 * Returns null if the ref doesn't exist yet.
 */
export async function fetchYakMap(
  client: GitHubClient,
  token?: string
): Promise<{ yakMap: YakMap | null; commitOid: string | null }> {
  // Get the ref
  const commitOid = await client.getRef(YAKS_REF, token);

  if (!commitOid) {
    return { yakMap: null, commitOid: null };
  }

  // Get the commit to find the tree
  const commit = await client.getCommit(commitOid, token);
  const treeSha = commit.tree.sha;

  // Get the tree recursively
  const treeEntries = await client.getTree(treeSha, true, token);

  // Parse into YakMap
  const yakMap = await parseGitHubTree(client, treeEntries, token);

  return { yakMap, commitOid };
}

/**
 * Commits and pushes a yak map to refs/notes/yaks.
 */
export async function pushYakMap(
  client: GitHubClient,
  yakMap: YakMap,
  message: string,
  author: Author,
  token: string,
  parentOid?: string
): Promise<string> {
  // Create tree from yak map
  const treeSha = await createTreeFromYakMap(client, yakMap, token);

  // Create commit
  const parents = parentOid ? [parentOid] : [];
  const commitSha = await client.createCommit(message, treeSha, parents, author, token);

  // Update the ref
  await client.updateRef(YAKS_REF, commitSha, token);

  return commitSha;
}

/**
 * Generates a commit message for yak map changes.
 */
export function generateCommitMessage(
  added: string[],
  removed: string[],
  modified: string[]
): string {
  const parts: string[] = [];

  if (added.length > 0) {
    parts.push(`add ${added.length} yak(s)`);
  }
  if (removed.length > 0) {
    parts.push(`remove ${removed.length} yak(s)`);
  }
  if (modified.length > 0) {
    parts.push(`update ${modified.length} yak(s)`);
  }

  if (parts.length === 0) {
    return 'Update yak map';
  }

  // Capitalize first letter
  const message = parts.join(', ');
  return message.charAt(0).toUpperCase() + message.slice(1);
}

/**
 * Compares two yak maps and returns the changes.
 */
export function diffYakMaps(
  oldMap: YakMap | null,
  newMap: YakMap
): { added: string[]; removed: string[]; modified: string[] } {
  const added: string[] = [];
  const removed: string[] = [];
  const modified: string[] = [];

  const oldIds = oldMap ? new Set(oldMap.yaks.keys()) : new Set<string>();
  const newIds = new Set(newMap.yaks.keys());

  // Find added and modified
  for (const id of newIds) {
    if (!oldIds.has(id)) {
      added.push(id);
    } else {
      const oldYak = oldMap!.yaks.get(id)!;
      const newYak = newMap.yaks.get(id)!;

      // Check if modified
      if (
        oldYak.done !== newYak.done ||
        oldYak.context !== newYak.context ||
        oldYak.parentId !== newYak.parentId
      ) {
        modified.push(id);
      }
    }
  }

  // Find removed
  for (const id of oldIds) {
    if (!newIds.has(id)) {
      removed.push(id);
    }
  }

  return { added, removed, modified };
}
