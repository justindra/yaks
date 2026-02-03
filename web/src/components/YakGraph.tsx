import { useEffect, useRef, useState, useMemo } from 'preact/hooks';
import * as d3Force from 'd3-force';
import * as d3Selection from 'd3-selection';
import * as d3Drag from 'd3-drag';
import * as d3Zoom from 'd3-zoom';
import type { YakMap } from '../lib/yak/types';
import { hasIncompleteChildren } from '../lib/yak/types';

interface YakGraphProps {
  yakMap: YakMap;
  selectedId: string | null;
  onSelect: (id: string | null) => void;
  onMove: (id: string, newParentId: string | null) => void;
  onToggleDone: (id: string) => void;
}

interface GraphNode extends d3Force.SimulationNodeDatum {
  id: string;
  name: string;
  done: boolean;
  hasIncompleteChildren: boolean;
  isRoot: boolean;
}

interface GraphLink extends d3Force.SimulationLinkDatum<GraphNode> {
  source: GraphNode | string;
  target: GraphNode | string;
}

// Colors for different states
const COLORS = {
  done: '#9ca3af',        // Gray
  active: '#3b82f6',      // Blue
  blocked: '#f59e0b',     // Amber (has incomplete children)
  selected: '#2563eb',    // Darker blue for selection ring
  link: '#475569',        // Slate for links
  text: '#f1f5f9',        // Light text
  background: '#0f172a',  // Dark background
};

/**
 * Creates a structural key from a YakMap - only changes when yaks are added/removed
 * or parent relationships change. Does NOT change for done/context updates.
 */
function getYakMapStructureKey(yakMap: YakMap): string {
  const entries = Array.from(yakMap.yaks.entries())
    .map(([id, yak]) => `${id}:${yak.parentId ?? 'root'}`)
    .sort()
    .join('|');
  return entries;
}

export function YakGraph({ yakMap, selectedId, onSelect, onMove, onToggleDone }: YakGraphProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<d3Selection.Selection<SVGGElement, unknown, null, undefined> | null>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const simulationRef = useRef<d3Force.Simulation<GraphNode, GraphLink> | null>(null);
  const dragTargetRef = useRef<string | null>(null);
  const nodesRef = useRef<GraphNode[]>([]);
  
  // Use refs for callbacks to avoid rebuilding graph when they change
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

  // Compute structural key - only changes when structure changes
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

  // Build and render the graph - only when structure changes
  useEffect(() => {
    if (!svgRef.current || yakMap.yaks.size === 0) return;

    const svg = d3Selection.select(svgRef.current);
    const { width, height } = dimensions;

    // Clear previous content
    svg.selectAll('*').remove();

    // Build nodes and links from yakMap
    const nodes: GraphNode[] = [];
    const links: GraphLink[] = [];
    const nodeMap = new Map<string, GraphNode>();

    for (const [id, yak] of yakMap.yaks) {
      const node: GraphNode = {
        id,
        name: yak.name,
        done: yak.done,
        hasIncompleteChildren: hasIncompleteChildren(yakMap, id),
        isRoot: yak.parentId === null,
      };
      nodes.push(node);
      nodeMap.set(id, node);
    }

    for (const yak of yakMap.yaks.values()) {
      if (yak.parentId && nodeMap.has(yak.parentId)) {
        links.push({
          source: yak.parentId,
          target: yak.id,
        });
      }
    }

    // Store nodes for updates
    nodesRef.current = nodes;

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

    // Create arrow marker for links
    svg.append('defs').append('marker')
      .attr('id', 'arrowhead')
      .attr('viewBox', '-0 -5 10 10')
      .attr('refX', 20)
      .attr('refY', 0)
      .attr('orient', 'auto')
      .attr('markerWidth', 6)
      .attr('markerHeight', 6)
      .append('path')
      .attr('d', 'M 0,-5 L 10,0 L 0,5')
      .attr('fill', COLORS.link);

    // Create links
    const link = container.append('g')
      .attr('class', 'links')
      .selectAll('line')
      .data(links)
      .enter()
      .append('line')
      .attr('stroke', COLORS.link)
      .attr('stroke-width', 1.5)
      .attr('marker-end', 'url(#arrowhead)');

    // Create node groups
    const node = container.append('g')
      .attr('class', 'nodes')
      .selectAll('g')
      .data(nodes)
      .enter()
      .append('g')
      .attr('class', 'node')
      .attr('cursor', 'pointer');

    // Add drop target circle (larger, invisible)
    node.append('circle')
      .attr('r', 30)
      .attr('fill', 'transparent')
      .attr('class', 'drop-target');

    // Add main circle
    node.append('circle')
      .attr('r', 16)
      .attr('class', 'main-circle')
      .attr('fill', (d: GraphNode) => getNodeColor(d))
      .attr('stroke', (d: GraphNode) => d.id === selectedIdRef.current ? COLORS.selected : 'transparent')
      .attr('stroke-width', 3);

    // Add checkbox indicator for done state
    node.append('text')
      .attr('class', 'done-indicator')
      .attr('text-anchor', 'middle')
      .attr('dominant-baseline', 'central')
      .attr('fill', COLORS.text)
      .attr('font-size', '12px')
      .attr('pointer-events', 'none')
      .text((d: GraphNode) => d.done ? '✓' : '');

    // Add label
    node.append('text')
      .attr('x', 0)
      .attr('y', 28)
      .attr('text-anchor', 'middle')
      .attr('fill', COLORS.text)
      .attr('font-size', '11px')
      .attr('pointer-events', 'none')
      .text((d: GraphNode) => d.name.length > 15 ? d.name.substring(0, 12) + '...' : d.name);

    // Click handler - use ref to get current callback
    node.on('click', (event: MouseEvent, d: GraphNode) => {
      event.stopPropagation();
      onSelectRef.current(d.id === selectedIdRef.current ? null : d.id);
    });

    // Double-click handler for toggling done - use ref
    node.on('dblclick', (event: MouseEvent, d: GraphNode) => {
      event.stopPropagation();
      onToggleDoneRef.current(d.id);
    });

    // Drag behavior for reparenting
    type DragEvent = d3Drag.D3DragEvent<SVGGElement, GraphNode, GraphNode>;
    const drag = d3Drag.drag<SVGGElement, GraphNode>()
      .on('start', function(event: DragEvent, d: GraphNode) {
        if (!event.active) simulationRef.current?.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
        dragTargetRef.current = null;
      })
      .on('drag', function(event: DragEvent, d: GraphNode) {
        d.fx = event.x;
        d.fy = event.y;

        // Find potential drop target
        let closestNodeId: string | null = null;
        let closestDist = 50; // Minimum distance for drop

        nodes.forEach(n => {
          if (n.id === d.id) return;
          // Don't allow dropping onto descendants
          if (isDescendant(yakMap, n.id, d.id)) return;

          const dx = (n.x ?? 0) - event.x;
          const dy = (n.y ?? 0) - event.y;
          const dist = Math.sqrt(dx * dx + dy * dy);

          if (dist < closestDist) {
            closestDist = dist;
            closestNodeId = n.id;
          }
        });

        // Highlight drop target
        dragTargetRef.current = closestNodeId;
        node.select('.drop-target')
          .attr('stroke', (n: unknown) => (n as GraphNode).id === dragTargetRef.current ? COLORS.selected : 'transparent')
          .attr('stroke-width', 2)
          .attr('stroke-dasharray', '4,2');
      })
      .on('end', function(event: DragEvent, d: GraphNode) {
        if (!event.active) simulationRef.current?.alphaTarget(0);
        d.fx = null;
        d.fy = null;

        // Handle drop - use ref
        if (dragTargetRef.current && dragTargetRef.current !== d.id) {
          onMoveRef.current(d.id, dragTargetRef.current);
        }

        // Clear highlight
        dragTargetRef.current = null;
        node.select('.drop-target')
          .attr('stroke', 'transparent');
      });

    node.call(drag as any);

    // Click on background to deselect - use ref
    svg.on('click', () => onSelectRef.current(null));

    // Set up force simulation
    const simulation = d3Force.forceSimulation<GraphNode>(nodes)
      .force('link', d3Force.forceLink<GraphNode, GraphLink>(links)
        .id(d => d.id)
        .distance(100)
        .strength(0.5))
      .force('charge', d3Force.forceManyBody().strength(-300))
      .force('center', d3Force.forceCenter(width / 2, height / 2))
      .force('collision', d3Force.forceCollide().radius(40))
      .force('x', d3Force.forceX(width / 2).strength(0.05))
      .force('y', d3Force.forceY(height / 2).strength(0.05));

    simulationRef.current = simulation;

    // Update positions on tick
    simulation.on('tick', () => {
      link
        .attr('x1', (d: GraphLink) => ((d.source as GraphNode).x ?? 0))
        .attr('y1', (d: GraphLink) => ((d.source as GraphNode).y ?? 0))
        .attr('x2', (d: GraphLink) => ((d.target as GraphNode).x ?? 0))
        .attr('y2', (d: GraphLink) => ((d.target as GraphNode).y ?? 0));

      node.attr('transform', (d: GraphNode) => `translate(${d.x ?? 0},${d.y ?? 0})`);
    });

    // Cleanup
    return () => {
      simulation.stop();
    };
  }, [structureKey, dimensions]); // Only rebuild on structure change or resize

  // Update selection highlight without rebuilding the graph
  useEffect(() => {
    if (!svgRef.current) return;
    const svg = d3Selection.select(svgRef.current);

    svg.selectAll('.node .main-circle')
      .attr('stroke', (d: any) => d.id === selectedId ? COLORS.selected : 'transparent');
  }, [selectedId]);

  // Update node visual properties (done state, colors) without rebuilding
  useEffect(() => {
    if (!svgRef.current || !containerRef.current) return;
    
    const container = containerRef.current;
    
    // Update each node's visual state based on current yakMap data
    container.selectAll('.node').each(function(d: any) {
      const yak = yakMap.yaks.get(d.id);
      if (!yak) return;
      
      const nodeSelection = d3Selection.select(this);
      const isDone = yak.done;
      const hasIncomplete = hasIncompleteChildren(yakMap, d.id);
      
      // Update node data
      d.done = isDone;
      d.hasIncompleteChildren = hasIncomplete;
      
      // Update circle color
      nodeSelection.select('.main-circle')
        .attr('fill', getNodeColor(d));
      
      // Update done indicator
      nodeSelection.select('.done-indicator')
        .text(isDone ? '✓' : '');
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
 * Gets the fill color for a node based on its state.
 */
function getNodeColor(node: GraphNode): string {
  if (node.done) {
    return COLORS.done;
  }
  if (node.hasIncompleteChildren) {
    return COLORS.blocked;
  }
  return COLORS.active;
}

/**
 * Checks if targetId is a descendant of sourceId.
 */
function isDescendant(yakMap: YakMap, targetId: string, sourceId: string): boolean {
  let current = yakMap.yaks.get(targetId);
  while (current) {
    if (current.parentId === sourceId) return true;
    current = current.parentId ? yakMap.yaks.get(current.parentId) : undefined;
  }
  return false;
}
