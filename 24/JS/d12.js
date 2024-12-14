import { getInputs } from "./lib/inputs.js";
import { lookup, inBounds, move, dirs, difference } from './lib/matrix.js';

const i = getInputs(12).toString().split('\n').map(line => line.split(''));
const testI = `AAAA
BBCD
BBCC
EEEC`.split('\n').map(line => line.split(''));
const testI2 = `RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE`.split('\n').map(line => line.split(''));

const numNeighbors = (coord, visited) => {
    return dirs(false).map(dir => move(coord, dir)).filter(c => visited.has(c.join(','))).length;
}

const walk = (m, c) => {
    const result = { area: 0, perimeter: 0 };
    const val = lookup(c, m);
    const visited = new Set();
    const stack = [c];
    while (stack.length) {
        c = stack.pop();
        dirs(false).forEach(dir => {
            const newC = move(c, dir);
            if (!visited.has(newC.join(',')) && lookup(newC, m) === val) {
                const neighbors = numNeighbors(newC, visited);
                visited.add(newC.join(','));
                stack.push(newC);
                result.area++;
                result.perimeter += 4;
                result.perimeter -= neighbors;
                result.perimeter -= result.area === 1 ? 0 : neighbors;
                console.log(result);
            }
        });
    }
    if (result.area === 0) return { area: 1, perimeter: 4, visited: new Set([c.join(',')]) };
    return { ...result, visited };
}

const partOne = (m) => {
    const visited = new Set();
    let fenceCost = 0;
    for (let x=0; x < m.length; x++) {
        for (let y=0; y < m[x].length; y++) {
            const val = lookup([x, y], m);
            if (!visited.has([x, y].join(','))) {
                const res = walk(m, [x, y]);
                fenceCost += res.area * res.perimeter;
                [...res.visited].forEach(c => {
                    visited.add(c);
                })
            }
        }
    }
    return fenceCost;
}

console.log(partOne(testI));
console.log(partOne(testI2));
// console.log(partOne(i));

const numWalls = (coords) => {
    const sortedCoords = [...coords].sort((a, b) => a[1] - b[1]).sort((a, b) => a[0] - b[0]);
    const highestRow = sortedCoords.reduce((c, i) => c[0] > i[0] ? c[0] : i[0])[0];
    const highestCol = sortedCoords.reduce((c, i) => c[1] > i[1] ? c[1] : i[1])[1];
    const result = { walls: 0 }
    for (let x = 0; x <= highestRow; x++) {
        for (let y = 0; y <= highestCol; y++) {
            if (coords.has([x, y].join(','))) {
                result.walls += 3;
                break;
            }
        }
    }
}

// Helper function to calculate the cross product of two vectors (for turn direction)
function cross(o, a, b) {
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
}

// Convex Hull using Andrew's monotone chain algorithm
function convexHull(points) {
    // Sort the points lexicographically (by x, then by y)
    points.sort((a, b) => a[0] - b[0] || a[1] - b[1]);

    // Build the lower hull
    const lower = [];
    for (const point of points) {
        while (lower.length >= 2 && cross(lower[lower.length - 2], lower[lower.length - 1], point) <= 0) {
            lower.pop();
        }
        lower.push(point);
    }

    // Build the upper hull
    const upper = [];
    for (let i = points.length - 1; i >= 0; i--) {
        while (upper.length >= 2 && cross(upper[upper.length - 2], upper[upper.length - 1], points[i]) <= 0) {
            upper.pop();
        }
        upper.push(points[i]);
    }

    // Remove the last point of each half because it is repeated
    upper.pop();
    lower.pop();

    // Concatenate lower and upper hull to form the full convex hull
    return [...lower, ...upper];
}

function countWalls(coordinates) {
    // Sort the coordinates in clockwise order (assuming they define the boundary of the shape)
    // A convex hull algorithm or other perimeter sorting method should be used here.
    // For simplicity, assume the coordinates are already sorted in a clockwise manner.
    coordinates = convexHull([...coordinates]);
    let wallCount = 0;
    
    // Function to calculate direction between two points (horizontal or vertical)
    function direction(p1, p2) {
        if (p1[0] === p2[0]) {
            return p2[1] > p1[1] ? 'vertical-down' : 'vertical-up';  // vertical movement
        } else if (p1[1] === p2[1]) {
            return p2[0] > p1[0] ? 'horizontal-right' : 'horizontal-left';  // horizontal movement
        }
        return null;  // Handle case where points aren't aligned (for grid, should never happen)
    }

    // Traverse the coordinates to detect direction changes
    for (let i = 0; i < coordinates.length; i++) {
        const current = coordinates[i];
        const next = coordinates[(i + 1) % coordinates.length];  // Wrap around to the first point
        const prev = coordinates[(i - 1 + coordinates.length) % coordinates.length];  // Wrap to last point

        const prevDirection = direction(prev, current);
        const nextDirection = direction(current, next);

        // If the directions change between consecutive edges, count a new wall
        if (prevDirection !== nextDirection) {
            wallCount++;
        }
    }

    return wallCount;
}

// Example: Random boundary points forming an arbitrary shape
const shapeCoordinates = [
    [0, 0], [2, 0], [0, 3], [3, 3], [3, 1], [2, 1], [2, 2], [1, 2], [1, 0]
];

console.log(new Array(4).fill(new Array(4).fill(0)).map((r, ri) => r.map((c, ci) => shapeCoordinates.map(v => v.join(',')).includes([ri, ci].join(',')) ? 1 : 0)))
console.log(countWalls(shapeCoordinates));

const newWalk = (m, c) => {
    const result = { area: 0, perimeter: 0, walls: 0 };
    const val = lookup(c, m);
    const visited = new Set();
    const stack = [c];
    while (stack.length) {
        c = stack.pop();
        dirs(false).forEach(dir => {
            const newC = move(c, dir);
            if (!visited.has(newC.join(',')) && lookup(newC, m) === val) {
                const neighbors = numNeighbors(newC, visited);
                visited.add(newC.join(','));
                stack.push(newC);
                result.area++;
                result.perimeter += 4;
                result.perimeter -= neighbors;
                result.perimeter -= result.area === 1 ? 0 : neighbors;
                console.log(result);
            }
        });
    }
    if (result.area === 0) return { area: 1, perimeter: 4, visited: new Set([c.join(',')]) };
    const wallVisited = new Set();
    [...visited].map(c => c.split(',').map(c => +c)).sort((a, b) => a[1] - b[1]).sort((a, b) => a[0] - b[0]).forEach(c => {
        if (typeof result.lastCoord === 'undefined') result.lastCoord = c;
        if (wallVisited.has(c.join(','))) return;
        wallVisited.add(c.join(','));
        const nonNeighbors = dirs(false).map(dir => move(c, dir)).filter(newC => !visited.has(newC.join(',')));
        if (nonNeighbors.length === 0) return;
        const wallTypes = nonNeighbors.map(n => difference(c, n)).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
        if ((typeof result.wallTypes !== 'undefined' && result.wallTypes !== JSON.stringify(wallTypes))) {
            const tStrings = wallTypes.map(t => t.join(','));
            const rtStrings = JSON.parse(result.wallTypes).map(t => t.join(','));
            result.walls += rtStrings.filter(t => !tStrings.includes(t)).length;
            result.wallTypes = JSON.stringify(wallTypes);
        } else if (!dirs(false).map(d => d.join(',')).includes(difference(c, result.lastCoord).join(','))) {
            result.walls += JSON.parse(result.wallTypes || '[]').length;
            result.wallTypes = JSON.stringify(wallTypes);
        }
        result.lastCoord = c;
    });
    result.walls += JSON.parse(result.walls).length || 0;
    return { ...result, visited };
}

const partTwo = (m) => {
    const visited = new Set();
    let fenceCost = 0;
    for (let x=0; x < m.length; x++) {
        for (let y=0; y < m[x].length; y++) {
            const val = lookup([x, y], m);
            if (!visited.has([x, y].join(','))) {
                const res = newWalk(m, [x, y]);
                fenceCost += res.area * res.walls;
                [...res.visited].forEach(c => {
                    visited.add(c);
                })
            }
        }
    }
    return fenceCost;
}

console.log(partTwo(testI));
console.log(partTwo(testI2));
console.log(partTwo(m));