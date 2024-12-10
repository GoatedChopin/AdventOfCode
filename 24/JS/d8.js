import { getInputs } from "./lib/inputs.js";
import { move, inBounds, lookup, difference, multiply } from './lib/matrix.js';
import { combinations } from "./lib/pattern.js";

const testM = `............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............`.split('\n').map(line => line.split(''));
const m = () => getInputs(8).toString().split('\n').map(line => line.replace('\r', '').split(''));


const partOne = (m) => {
    const display = [...m];
    const antinodes = new Set();
    const antennas = {};
    for (let x = 0; x < m.length; x++) {
        for (let y = 0; y < m[x].length; y++) {
            const val = lookup([x, y], m)
            if (val === '.') continue;
            else if (!antennas[val]) antennas[val] = [];
            antennas[val].push([x, y]);
        }
    }

    Object.keys(antennas).forEach(key => {
        const a = antennas[key];
        combinations(a, 2).forEach(c => {
            c.sort();
            const diff = difference(c[1], c[0]);
            const ant1 = move(c[1], diff);
            const ant2 = move(c[0], multiply(diff, -1));
            if (inBounds(ant1, m)) {
                antinodes.add('' + ant1);
                display[ant1[0]][ant1[1]] = '#';
            }
            if (inBounds(ant2, m)) {
                antinodes.add('' + ant2);
                display[ant2[0]][ant2[1]] = '#';
            }
        });
    });
    return antinodes;
}

console.log(partOne(testM));
console.log(partOne(m()));

const partTwo = (m) => {
    const display = [...m];
    const antinodes = new Set();
    const antennas = {};
    for (let x = 0; x < m.length; x++) {
        for (let y = 0; y < m[x].length; y++) {
            const val = lookup([x, y], m)
            if (val === '.') continue;
            else if (!antennas[val]) antennas[val] = [];
            antennas[val].push([x, y]);
        }
    }

    Object.keys(antennas).forEach(key => {
        const a = antennas[key];
        combinations(a, 2).forEach(c => {
            c.sort();
            antinodes.add('' + c[0]);
            antinodes.add('' + c[1]);
            const diff = difference(c[1], c[0]);
            while (inBounds(move(c[1], diff), m)) {
                const ant1 = move(c[1], diff)
                if (inBounds(ant1, m)) {
                    antinodes.add('' + ant1);
                    display[ant1[0]][ant1[1]] = '#';
                };
                c[1] = move(c[1], diff);
            }
            while (inBounds(move(c[0], multiply(diff, -1)), m)) {
                const ant2 = move(c[0], multiply(diff, -1));
                if (inBounds(ant2, m)) {
                    antinodes.add('' + ant2);
                    display[ant2[0]][ant2[1]] = '#';
                }
                c[0] = move(c[0], multiply(diff, -1));
            }
        });
    });
    return antinodes;
}

const testM2 = `T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........`.split('\n').map(line => line.split(''));
console.log(partTwo(testM2));
console.log(partTwo(m()));