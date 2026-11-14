import type { MediaSubfolder } from '../../types';

export interface FolderTreeNode {
  name: string;
  relative_path: string;
  count: number;
  depth: number;
  children: FolderTreeNode[];
}

export function buildFolderTree(subfolders: MediaSubfolder[]): FolderTreeNode[] {
  interface TempNode {
    name: string;
    relative_path: string;
    count: number;
    depth: number;
    childrenMap: Map<string, TempNode>;
  }

  const root: TempNode = {
    name: '',
    relative_path: '',
    count: 0,
    depth: -1,
    childrenMap: new Map(),
  };

  for (const sub of subfolders) {
    const parts = sub.relative_path.split('/');
    let current = root;
    let accumulatedPath = '';

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      accumulatedPath = accumulatedPath ? `${accumulatedPath}/${part}` : part;

      let child = current.childrenMap.get(part);
      if (!child) {
        child = {
          name: part,
          relative_path: accumulatedPath,
          count: accumulatedPath === sub.relative_path ? sub.count : 0,
          depth: i,
          childrenMap: new Map(),
        };
        current.childrenMap.set(part, child);
      } else if (accumulatedPath === sub.relative_path) {
        child.count = sub.count;
      }
      current = child;
    }
  }

  function convert(node: TempNode): FolderTreeNode[] {
    const list: FolderTreeNode[] = [];
    for (const child of node.childrenMap.values()) {
      list.push({
        name: child.name,
        relative_path: child.relative_path,
        count: child.count,
        depth: child.depth,
        children: convert(child),
      });
    }
    return list.sort((a, b) => a.name.localeCompare(b.name));
  }

  return convert(root);
}

export function getDirectSubfoldersForLevel(
  allSubfolders: MediaSubfolder[],
  currentLevel: string | null
): MediaSubfolder[] {
  const map = new Map<string, { relative_path: string; name: string; count: number }>();
  const prefix = currentLevel ? `${currentLevel}/` : '';

  for (const sub of allSubfolders) {
    if (currentLevel === null) {
      const topSegment = sub.relative_path.split('/')[0];
      const existing = map.get(topSegment);
      if (!existing) {
        map.set(topSegment, {
          relative_path: topSegment,
          name: topSegment,
          count: sub.relative_path === topSegment ? sub.count : 0,
        });
      } else if (sub.relative_path === topSegment) {
        existing.count = sub.count;
      }
    } else if (sub.relative_path.startsWith(prefix)) {
      const remainder = sub.relative_path.slice(prefix.length);
      const directName = remainder.split('/')[0];
      const directRelPath = `${prefix}${directName}`;
      const existing = map.get(directRelPath);
      if (!existing) {
        map.set(directRelPath, {
          relative_path: directRelPath,
          name: directName,
          count: sub.relative_path === directRelPath ? sub.count : 0,
        });
      } else if (sub.relative_path === directRelPath) {
        existing.count = sub.count;
      }
    }
  }

  return Array.from(map.values()).sort((a, b) => a.name.localeCompare(b.name));
}

export function formatFolderBreadcrumbs(subfolder: string | null): Array<{ name: string; path: string }> {
  if (!subfolder) return [];
  const parts = subfolder.split('/');
  return parts.map((name, index) => ({
    name,
    path: parts.slice(0, index + 1).join('/'),
  }));
}
