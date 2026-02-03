// Re-export all types from lib modules for convenience
export type {
  Yak,
  YakMap,
  Author,
  SyncResult,
} from '../lib/yak/types';

export {
  createEmptyYakMap,
  createYak,
  getChildren,
  getDescendants,
  getParentPath,
  getLeafName,
  hasIncompleteChildren,
  canMarkDone,
  cloneYakMap,
} from '../lib/yak/types';

export {
  validateYakName,
  validateMarkDone,
  validateDelete,
  validateMove,
  validateAdd,
} from '../lib/yak/validation';

export { parseGitHubTree, parseFromPaths } from '../lib/yak/parser';

export { serializeYakMap, createTreeFromYakMap, toFilePaths } from '../lib/yak/serializer';

export type { GitHubTreeEntry, GitHubCommit } from '../lib/git/client';
export { GitHubClient, GitHubAPIError, parseGitHubUrl, toGitHubUrl } from '../lib/git/client';
