import { getInputs } from "./lib/inputs.js";
import { lookup, inBounds, move, dirs } from './lib/matrix.js';

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
console.log(partOne(i));
