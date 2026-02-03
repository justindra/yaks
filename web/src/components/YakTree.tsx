import { useEffect, useRef, useState, useMemo } from 'preact/hooks';
import * as d3Hierarchy from 'd3-hierarchy';
import * as d3Selection from 'd3-selection';
import * as d3Drag from 'd3-drag';
import * as d3Zoom from 'd3-zoom';
import * as d3Shape from 'd3-shape';
import type { YakMap } from '../lib/yak/types';
import { hasIncompleteChildren } from '../lib/yak/types';

interface YakTreeProps {
  yakMap: YakMap;
  selectedId: string | null;
  onSelect: (id: string | null) => void;
  onMove: (id: string, newParentId: string | null) => void;
  onToggleDone: (id: string) => void;
}

interface TreeNodeData {
  id: string;
  name: string;
  done: boolean;
  hasIncompleteChildren: boolean;
  isVirtualRoot?: boolean;
  children?: TreeNodeData[];
}

type HierarchyNode = d3Hierarchy.HierarchyPointNode<TreeNodeData>;

// Node dimensions
const NODE_WIDTH = 100;
const NODE_HEIGHT = 32;
const NODE_MARGIN_X = 20;
const NODE_MARGIN_Y = 50;

// Colors for different states
const COLORS = {
  done: { bg: '#374151', border: '#6b7280', text: '#9ca3af' },
  active: { bg: '#1e3a5f', border: '#3b82f6', text: '#f1f5f9' },
  blocked: { bg: '#451a03', border: '#f59e0b', text: '#fef3c7' },
  selected: '#2563eb',
  link: '#475569',
  background: '#0f172a',
};

/**
 * Builds a hierarchical tree structure from a YakMap.
 * Creates a virtual root if there are multiple root yaks.
 */
function buildHierarchy(yakMap: YakMap): TreeNodeData {
  const nodeMap = new Map<string, TreeNodeData>();

  // Create nodes for all yaks
  for (const [id, yak] of yakMap.yaks) {
    nodeMap.set(id, {
      id,
      name: yak.name,
      done: yak.done,
      hasIncompleteChildren: hasIncompleteChildren(yakMap, id),
      children: [],
    });
  }

  // Build parent-child relationships
  for (const yak of yakMap.yaks.values()) {
    if (yak.parentId) {
      const parent = nodeMap.get(yak.parentId);
      const child = nodeMap.get(yak.id);
      if (parent && child) {
        parent.children!.push(child);
      }
    }
  }

  // Sort children alphabetically
  for (const node of nodeMap.values()) {
    if (node.children && node.children.length > 0) {
      node.children.sort((a, b) => a.name.localeCompare(b.name));
    }
  }

  // Get root nodes
  const roots: TreeNodeData[] = [];
  for (const yak of yakMap.yaks.values()) {
    if (yak.parentId === null) {
      const node = nodeMap.get(yak.id);
      if (node) roots.push(node);
    }
  }

  // Sort roots alphabetically
  roots.sort((a, b) => a.name.localeCompare(b.name));

  // If single root, return it; otherwise create virtual root
  if (roots.length === 1) {
    return roots[0];
  }

  return {
    id: '__virtual_root__',
    name: '',
    done: false,
    hasIncompleteChildren: false,
    isVirtualRoot: true,
    children: roots,
  };
}

/**
 * Creates a structural key from a YakMap for dependency tracking.
 */
function getYakMapStructureKey(yakMap: YakMap): string {
  const entries = Array.from(yakMap.yaks.entries())
    .map(([id, yak]) => `${id}:${yak.parentId ?? 'root'}`)
    .sort()
    .join('|');
  return entries;
}

export function YakTree({ yakMap, selectedId, onSelect, onMove, onToggleDone }: YakTreeProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<d3Selection.Selection<SVGGElement, unknown, null, undefined> | null>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const dragTargetRef = useRef<string | null>(null);

  // Use refs for callbacks to avoid rebuilding tree when they change
  const onSelectRef = useRef(onSelect);
  const onMoveRef = useRef(onMove);
  const onToggleDoneRef = useRef(onToggleDone);
  const selectedIdRef = useRef(selectedId);

  // Keep refs up to date
  useEffect(() => {
    onSelectRef.current = onSelect;
    onMoveRef.current = onMove;
    onToggleDoneRef.current = onToggleDone;
    selectedIdRef.current = selectedId;
  });

  // Compute structural key
  const structureKey = useMemo(() => getYakMapStructureKey(yakMap), [yakMap]);

  // Update dimensions on resize
  useEffect(() => {
    const updateDimensions = () => {
      if (svgRef.current?.parentElement) {
        const { width, height } = svgRef.current.parentElement.getBoundingClientRect();
        setDimensions({ width, height });
      }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  // Build and render the tree
  useEffect(() => {
    if (!svgRef.current || yakMap.yaks.size === 0) return;

    const svg = d3Selection.select(svgRef.current);
    const { width, height } = dimensions;

    // Clear previous content
    svg.selectAll('*').remove();

    // Build hierarchy
    const rootData = buildHierarchy(yakMap);
    const hierarchy = d3Hierarchy.hierarchy(rootData);

    // Calculate tree layout
    const treeLayout = d3Hierarchy.tree<TreeNodeData>()
      .nodeSize([NODE_WIDTH + NODE_MARGIN_X, NODE_HEIGHT + NODE_MARGIN_Y]);

    const treeData = treeLayout(hierarchy);

    // Get all nodes and links
    const allNodes = treeData.descendants();
    const allLinks = treeData.links();

    // Filter out virtual root from rendering
    const nodes = allNodes.filter(d => !d.data.isVirtualRoot);
    const links = allLinks.filter(d => !d.source.data.isVirtualRoot);

    // Calculate bounds
    let minX = Infinity, maxX = -Infinity;
    let minY = Infinity, maxY = -Infinity;
    nodes.forEach(d => {
      minX = Math.min(minX, d.x);
      maxX = Math.max(maxX, d.x);
      minY = Math.min(minY, d.y);
      maxY = Math.max(maxY, d.y);
    });

    const treeWidth = maxX - minX + NODE_WIDTH + 40;
    const treeHeight = maxY - minY + NODE_HEIGHT + 40;

    // Create container group for zoom/pan
    const container = svg.append('g');
    containerRef.current = container;

    // Set up zoom behavior
    const zoom = d3Zoom.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        container.attr('transform', event.transform);
      });

    svg.call(zoom);

    // Calculate initial transform to center the tree
    const scale = Math.min(
      (width - 40) / treeWidth,
      (height - 40) / treeHeight,
      1
    );
    const centerX = width / 2 - (minX + maxX) / 2 * scale;
    const centerY = 40;

    // Apply initial transform
    svg.call(
      zoom.transform,
      d3Zoom.zoomIdentity.translate(centerX, centerY).scale(scale)
    );

    // Create links (curved paths)
    const linkGenerator = d3Shape.linkVertical<d3Hierarchy.HierarchyPointLink<TreeNodeData>, d3Hierarchy.HierarchyPointNode<TreeNodeData>>()
      .x(d => d.x)
      .y(d => d.y);

    container.append('g')
      .attr('class', 'links')
      .selectAll('path')
      .data(links)
      .enter()
      .append('path')
      .attr('d', d => linkGenerator(d as any))
      .attr('fill', 'none')
      .attr('stroke', COLORS.link)
      .attr('stroke-width', 1.5);

    // Create node groups
    const node = container.append('g')
      .attr('class', 'nodes')
      .selectAll('g')
      .data(nodes)
      .enter()
      .append('g')
      .attr('class', 'node')
      .attr('transform', d => `translate(${d.x},${d.y})`)
      .attr('cursor', 'pointer');

    // Add drop target rect (larger, invisible)
    node.append('rect')
      .attr('class', 'drop-target')
      .attr('x', -NODE_WIDTH / 2 - 8)
      .attr('y', -NODE_HEIGHT / 2 - 8)
      .attr('width', NODE_WIDTH + 16)
      .attr('height', NODE_HEIGHT + 16)
      .attr('rx', 8)
      .attr('fill', 'transparent');

    // Add main rectangle
    node.append('rect')
      .attr('class', 'main-rect')
      .attr('x', -NODE_WIDTH / 2)
      .attr('y', -NODE_HEIGHT / 2)
      .attr('width', NODE_WIDTH)
      .attr('height', NODE_HEIGHT)
      .attr('rx', 6)
      .attr('fill', d => getNodeColors(d.data).bg)
      .attr('stroke', d => d.data.id === selectedIdRef.current ? COLORS.selected : getNodeColors(d.data).border)
      .attr('stroke-width', d => d.data.id === selectedIdRef.current ? 3 : 1.5);

    // Add name text
    node.append('text')
      .attr('class', 'name-text')
      .attr('text-anchor', 'middle')
      .attr('dominant-baseline', 'central')
      .attr('fill', d => getNodeColors(d.data).text)
      .attr('font-size', '11px')
      .attr('pointer-events', 'none')
      .text(d => {
        const name = d.data.name;
        return name.length > 12 ? name.substring(0, 10) + '...' : name;
      });

    // Add checkmark for done items
    node.filter(d => d.data.done)
      .append('text')
      .attr('x', NODE_WIDTH / 2 - 12)
      .attr('y', 0)
      .attr('text-anchor', 'middle')
      .attr('dominant-baseline', 'central')
      .attr('fill', d => getNodeColors(d.data).text)
      .attr('font-size', '10px')
      .attr('pointer-events', 'none')
      .text('✓');

    // Click handler
    node.on('click', (event: MouseEvent, d: HierarchyNode) => {
      event.stopPropagation();
      onSelectRef.current(d.data.id === selectedIdRef.current ? null : d.data.id);
    });

    // Double-click handler for toggling done
    node.on('dblclick', (event: MouseEvent, d: HierarchyNode) => {
      event.stopPropagation();
      onToggleDoneRef.current(d.data.id);
    });

    // Drag behavior for reparenting
    type DragEvent = d3Drag.D3DragEvent<SVGGElement, HierarchyNode, HierarchyNode>;
    
    const drag = d3Drag.drag<SVGGElement, HierarchyNode>()
      .on('start', function(_event: DragEvent) {
        d3Selection.select(this).raise();
        dragTargetRef.current = null;
      })
      .on('drag', function(event: DragEvent, d: HierarchyNode) {
        // Move the node visually
        d3Selection.select(this)
          .attr('transform', `translate(${event.x},${event.y})`);

        // Find potential drop target
        let closestNodeId: string | null = null;
        let closestDist = 60;

        nodes.forEach(n => {
          if (n.data.id === d.data.id) return;
          // Don't allow dropping onto descendants
          if (isDescendant(n, d)) return;

          const dx = n.x - event.x;
          const dy = n.y - event.y;
          const dist = Math.sqrt(dx * dx + dy * dy);

          if (dist < closestDist) {
            closestDist = dist;
            closestNodeId = n.data.id;
          }
        });

        // Highlight drop target
        dragTargetRef.current = closestNodeId;
        node.select('.drop-target')
          .attr('stroke', (n: any) => n.data.id === dragTargetRef.current ? COLORS.selected : 'transparent')
          .attr('stroke-width', 2)
          .attr('stroke-dasharray', '4,2');
      })
      .on('end', function(_event: DragEvent, d: HierarchyNode) {
        // Handle drop
        if (dragTargetRef.current && dragTargetRef.current !== d.data.id) {
          onMoveRef.current(d.data.id, dragTargetRef.current);
        } else {
          // Reset position if no valid drop
          d3Selection.select(this)
            .attr('transform', `translate(${d.x},${d.y})`);
        }

        // Clear highlight
        dragTargetRef.current = null;
        node.select('.drop-target')
          .attr('stroke', 'transparent');
      });

    node.call(drag as any);

    // Click on background to deselect
    svg.on('click', () => onSelectRef.current(null));

    // Cleanup
    return () => {};
  }, [structureKey, dimensions]);

  // Update selection highlight without rebuilding
  useEffect(() => {
    if (!svgRef.current) return;
    const svg = d3Selection.select(svgRef.current);

    svg.selectAll('.node .main-rect')
      .attr('stroke', (d: any) => d.data.id === selectedId ? COLORS.selected : getNodeColors(d.data).border)
      .attr('stroke-width', (d: any) => d.data.id === selectedId ? 3 : 1.5);
  }, [selectedId]);

  // Update node visual properties without rebuilding
  useEffect(() => {
    if (!svgRef.current || !containerRef.current) return;

    const container = containerRef.current;

    container.selectAll('.node').each(function(d: any) {
      const yak = yakMap.yaks.get(d.data.id);
      if (!yak) return;

      const nodeSelection = d3Selection.select(this);
      const colors = getNodeColors({
        ...d.data,
        done: yak.done,
        hasIncompleteChildren: hasIncompleteChildren(yakMap, d.data.id),
      });

      // Update data
      d.data.done = yak.done;
      d.data.hasIncompleteChildren = hasIncompleteChildren(yakMap, d.data.id);

      // Update rect colors
      nodeSelection.select('.main-rect')
        .attr('fill', colors.bg)
        .attr('stroke', d.data.id === selectedIdRef.current ? COLORS.selected : colors.border);

      // Update text color
      nodeSelection.select('.name-text')
        .attr('fill', colors.text);

      // Update or add/remove checkmark
      const existingCheck = nodeSelection.select('text:last-of-type');
      if (yak.done && existingCheck.text() !== '✓') {
        nodeSelection.append('text')
          .attr('x', NODE_WIDTH / 2 - 12)
          .attr('y', 0)
          .attr('text-anchor', 'middle')
          .attr('dominant-baseline', 'central')
          .attr('fill', colors.text)
          .attr('font-size', '10px')
          .attr('pointer-events', 'none')
          .text('✓');
      } else if (!yak.done && existingCheck.text() === '✓') {
        existingCheck.remove();
      }
    });
  }, [yakMap]);

  // Empty state
  if (yakMap.yaks.size === 0) {
    return (
      <div class="flex items-center justify-center h-full text-muted">
        <div class="text-center">
          <p>No yaks in this repository yet.</p>
          <p class="text-sm mt-sm">Click "Add Yak" to create your first yak.</p>
        </div>
      </div>
    );
  }

  return (
    <svg
      ref={svgRef}
      width={dimensions.width}
      height={dimensions.height}
      style={{ display: 'block', background: COLORS.background }}
    />
  );
}

/**
 * Gets the colors for a node based on its state.
 */
function getNodeColors(node: TreeNodeData): { bg: string; border: string; text: string } {
  if (node.done) {
    return COLORS.done;
  }
  if (node.hasIncompleteChildren) {
    return COLORS.blocked;
  }
  return COLORS.active;
}

/**
 * Checks if potentialDescendant is a descendant of potentialAncestor.
 */
function isDescendant(potentialDescendant: HierarchyNode, potentialAncestor: HierarchyNode): boolean {
  let current: HierarchyNode | null = potentialDescendant;
  while (current) {
    if (current.data.id === potentialAncestor.data.id) return true;
    current = current.parent;
  }
  return false;
}
