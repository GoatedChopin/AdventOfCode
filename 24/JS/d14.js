import { getInputs } from './lib/inputs.js';
import { move, wrap, lookup, inBounds, multiply, emptyGrid } from './lib/matrix.js';

const i = getInputs(14).toString().split('\n').map(line => {
    const coords = [...line.matchAll(/-?\d+,-?\d+/g)].map(m => m[0].split(',').map(i => +i));
    const p = coords[0];
    const v = coords[1];
    return { p, v }
});
const testI = `p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3`.split('\n').map(line => {
    const coords = [...line.matchAll(/-?\d+,-?\d+/g)].map(m => m[0].split(',').map(i => +i));
    const p = coords[0];
    const v = coords[1];
    return { p, v }
});

const display = (robots, gridSize) => {
    const grid = emptyGrid(gridSize);
    robots.forEach(r => {
        grid[r.p[0]][r.p[1]]++;
    });
    console.log(grid.map((r, ri) => r.join('')).join('\n'));
}

const partOne = (robots, gridSize, steps=100) => {
    const grid = emptyGrid(gridSize);
    for (let s = 0; s < steps; s++) {
        robots = robots.map(r => {
            const newP = wrap(move(r.p, r.v), grid);
            return { p: newP, v: r.v };
        });
        // display(robots, gridSize);
    }
    const quadrants = new Array(4).fill(0);
    const quadrant = (coord) => {
        const x = coord[0];
        const y = coord[1];
        const midX = new Number(grid.length / 2).toPrecision(1) - 1;
        const midY = new Number(grid[0].length / 2).toPrecision(1) - 1;
        if (x == midX || y == midY) return null;
        let q = 0;
        if (x >= midX) q = 1;
        if (y >= midY) q = 2;
        if (x >= midX && y >= midY) q = 3;
        return q;
    }
    robots.forEach(r => {
        const q = quadrant(r.p);
        if (q !== null) quadrants[q]++;
    });
    return quadrants.reduce((c, i) => c*i);
}

console.log(partOne(testI, [11, 7]));
console.log(partOne(i, [101, 103]));