import { getInputs } from "./lib/inputs.js";
import { move, lookup, inBounds, dirs } from './lib/matrix.js';


const i = getInputs(10).toString().split('\n').map(line => line.replace('\r', '').split('').map(c => +c));
const testI = `0123
1234
8765
9876`.split('\n').map(line => line.replace('\r', '').split('').map(c => +c));
const testI2 = `89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732`.split('\n').map(line => line.replace('\r', '').split('').map(c => +c))

const partOne = (m) => {
    const zeros = [];
    let goodTrails = 0;

    for (let x = 0; x < m.length; x++) {
        for (let y = 0; y < m[x].length; y++) {
            if (lookup([x, y], m) === 0) zeros.push([x, y]);
        }
    }

    let coord, nextCoord, trailHead;
    const nineSet = new Set();
    const stack = [...zeros.map(coord => [...coord, 0])];
    while (stack.length) {
        coord = stack.pop();
        if (coord[2] === 0) trailHead = coord;
        console.log(m.map((x, xi) => {
            return m[xi].map((y, yi) => {
                if ([xi, yi].join('') == coord.slice(0, 2).join('')) return '#';
                else return lookup([xi, yi], m);
            }).join('')
        }).join('\n') + '\n');
        if (coord[2] === 9) {
            goodTrails++;
            nineSet.add([...trailHead.slice(0, 2), ...coord.slice(0, 2)].join(','))
            continue;
        }
        dirs(false).forEach(dir => {
            nextCoord = move(coord.slice(0, 2), dir)
            if (lookup(nextCoord, m) === coord[2] + 1) {
                stack.push([...nextCoord, coord[2] + 1]);
            }
        });
    }
    return nineSet.size;
}

console.log(partOne(testI));
console.log(partOne(testI2));
console.log(partOne(i));