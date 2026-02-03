import type { YakMap, Yak, SyncResult, Author } from '../yak/types';
import { createEmptyYakMap } from '../yak/types';
import type { GitHubClient } from './client';
import { fetchYakMap, pushYakMap, diffYakMaps, generateCommitMessage } from './refs';

/**
 * Merges two yak maps using a 3-way merge with last-write-wins strategy.
 * 
 * Strategy:
 * - If a yak exists only in local: keep it (was added locally)
 * - If a yak exists only in remote: add it (was added remotely, unless deleted locally)
 * - If a yak exists in both: keep local version (last-write-wins)
 * - If a yak was deleted locally: don't include it (local delete wins)
 */
export function mergeYakMaps(
  base: YakMap | null,
  local: YakMap,
  remote: YakMap
): { merged: YakMap; hadConflicts: boolean } {
  const merged = createEmptyYakMap();
  let hadConflicts = false;

  const baseIds = base ? new Set(base.yaks.keys()) : new Set<string>();
  const localIds = new Set(local.yaks.keys());
  const remoteIds = new Set(remote.yaks.keys());

  // Get all unique IDs
  const allIds = new Set([...localIds, ...remoteIds]);

  for (const id of allIds) {
    const inBase = baseIds.has(id);
    const inLocal = localIds.has(id);
    const inRemote = remoteIds.has(id);

    if (inLocal && !inRemote) {
      // Only in local
      if (inBase) {
        // Was in base but removed from remote - local delete vs remote delete
        // Keep local (it was added back or never deleted locally)
        merged.yaks.set(id, cloneYak(local.yaks.get(id)!));
      } else {
        // New in local, add it
        merged.yaks.set(id, cloneYak(local.yaks.get(id)!));
      }
    } else if (!inLocal && inRemote) {
      // Only in remote
      if (inBase) {
        // Was in base, deleted locally - local delete wins
        // Don't add to merged
        hadConflicts = true;
      } else {
        // New in remote, add it
        merged.yaks.set(id, cloneYak(remote.yaks.get(id)!));
      }
    } else if (inLocal && inRemote) {
      // In both - check if there's a conflict
      const localYak = local.yaks.get(id)!;
      const remoteYak = remote.yaks.get(id)!;
      const baseYak = inBase ? base!.yaks.get(id) : null;

      if (yaksEqual(localYak, remoteYak)) {
        // No conflict, same content
        merged.yaks.set(id, cloneYak(localYak));
      } else if (baseYak && yaksEqual(localYak, baseYak)) {
        // Local unchanged, take remote
        merged.yaks.set(id, cloneYak(remoteYak));
      } else if (baseYak && yaksEqual(remoteYak, baseYak)) {
        // Remote unchanged, take local
        merged.yaks.set(id, cloneYak(localYak));
      } else {
        // Both changed - last-write-wins: keep local
        merged.yaks.set(id, cloneYak(localYak));
        hadConflicts = true;
      }
    }
  }

  // Rebuild roots
  merged.roots = Array.from(merged.yaks.values())
    .filter(yak => yak.parentId === null)
    .map(yak => yak.id)
    .sort();

  return { merged, hadConflicts };
}

/**
 * Deep clones a yak.
 */
function cloneYak(yak: Yak): Yak {
  return { ...yak };
}

/**
 * Checks if two yaks are equal (same content).
 */
function yaksEqual(a: Yak, b: Yak): boolean {
  return (
    a.id === b.id &&
    a.name === b.name &&
    a.parentId === b.parentId &&
    a.done === b.done &&
    a.context === b.context
  );
}

/**
 * Performs a full sync cycle: fetch, merge, commit, push.
 * 
 * This handles the case where remote has changed since we last fetched:
 * 1. Fetch latest from remote
 * 2. Merge with local changes
 * 3. Commit the merged result
 * 4. Push to remote
 */
export async function syncYakMap(
  client: GitHubClient,
  localYakMap: YakMap,
  baseYakMap: YakMap | null,
  baseCommitOid: string | null,
  author: Author,
  token: string
): Promise<SyncResult> {
  // Fetch latest from remote
  const { yakMap: remoteYakMap, commitOid: remoteCommitOid } = await fetchYakMap(client, token);

  // If remote is empty, just push our local
  if (!remoteYakMap) {
    const diff = diffYakMaps(null, localYakMap);
    const message = generateCommitMessage(diff.added, diff.removed, diff.modified);
    const commitSha = await pushYakMap(client, localYakMap, message, author, token);

    return {
      hadConflicts: false,
      localChanges: diff.added.length + diff.modified.length,
      remoteChanges: 0,
      commitSha,
    };
  }

  // If remote hasn't changed since base, just push local
  if (remoteCommitOid === baseCommitOid) {
    const diff = diffYakMaps(baseYakMap, localYakMap);
    const message = generateCommitMessage(diff.added, diff.removed, diff.modified);
    const commitSha = await pushYakMap(client, localYakMap, message, author, token, remoteCommitOid ?? undefined);

    return {
      hadConflicts: false,
      localChanges: diff.added.length + diff.modified.length + diff.removed.length,
      remoteChanges: 0,
      commitSha,
    };
  }

  // Remote has changed, need to merge
  const { merged, hadConflicts } = mergeYakMaps(baseYakMap, localYakMap, remoteYakMap);

  // Calculate changes
  const localDiff = diffYakMaps(baseYakMap, localYakMap);
  const remoteDiff = diffYakMaps(baseYakMap, remoteYakMap);

  // Generate commit message
  const message = hadConflicts
    ? 'Merge yak maps (conflicts auto-resolved with last-write-wins)'
    : 'Merge yak maps';

  // Push merged result
  const commitSha = await pushYakMap(client, merged, message, author, token, remoteCommitOid ?? undefined);

  return {
    hadConflicts,
    localChanges: localDiff.added.length + localDiff.modified.length + localDiff.removed.length,
    remoteChanges: remoteDiff.added.length + remoteDiff.modified.length + remoteDiff.removed.length,
    commitSha,
  };
}

/**
 * Pulls latest from remote and returns the merged result.
 * Does not push - useful for just refreshing local state.
 */
export async function pullYakMap(
  client: GitHubClient,
  localYakMap: YakMap | null,
  baseYakMap: YakMap | null,
  token?: string
): Promise<{ yakMap: YakMap | null; commitOid: string | null; hadConflicts: boolean }> {
  const { yakMap: remoteYakMap, commitOid } = await fetchYakMap(client, token);

  if (!remoteYakMap) {
    // Remote is empty
    return { yakMap: localYakMap, commitOid, hadConflicts: false };
  }

  if (!localYakMap) {
    // Local is empty, take remote
    return { yakMap: remoteYakMap, commitOid, hadConflicts: false };
  }

  // Merge
  const { merged, hadConflicts } = mergeYakMaps(baseYakMap, localYakMap, remoteYakMap);

  return { yakMap: merged, commitOid, hadConflicts };
}
