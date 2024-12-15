import { Heap } from '@datastructures-js/heap';

import { move, difference, vectorDistance, divisible, divide, determinant, multiply } from './lib/matrix.js';
import { getInputs } from './lib/inputs.js';

const i = getInputs(13).toString().replaceAll('\r', '').split('\n\n').map(g => {
    const lines = g.split('\n');
    const aNums = [...lines[0].matchAll(/\d+/g)];
    const bNums = [...lines[1].matchAll(/\d+/g)];
    const gNums = [...lines[2].matchAll(/\d+/g)]
    const a = { delta: aNums.map(i => +i[0]), cost: 3 };
    const b = { delta: bNums.map(i => +i[0]), cost: 1 };
    return { bigGoal: gNums.map(i => 10000000000000 + (+i[0])), goal: gNums.map(i => +i[0]), a, b }
});
// const why = getInputs(13).toString();
const testI = `Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279`.split('\n\n').map(g => {
    const lines = g.split('\n');
    const aNums = [...lines[0].matchAll(/\d+/g)];
    const bNums = [...lines[1].matchAll(/\d+/g)];
    const gNums = [...lines[2].matchAll(/\d+/g)]
    const a = { delta: aNums.map(i => +i[0]), cost: 3 };
    const b = { delta: bNums.map(i => +i[0]), cost: 1 };
    return { goal: gNums.map(i => +i[0]), a, b }
});

const arrayEquals = (a1, a2) => {
    if (a1.length !== a2.length) return false;
    return !a1.some((v, i) => v !== a2[i]);
}

const lowestCost = (config) => {
    console.log('Searching for solution to', JSON.stringify(config));
    const start = { cost: 0, coord: [0, 0] };
    const heap = new Heap((a, b) => a.cost - b.cost || vectorDistance(config.goal, b.coord) - vectorDistance(config.goal, a.coord));
    const visited = new Set();
    heap.push(start);
    while (heap.size()) {
        const current = heap.pop();
        // console.log(JSON.stringify(current));
        if (arrayEquals(config.goal, current.coord)) return current.cost;
        else if (current.coord[0] > config.goal[0]) continue;
        else if (current.coord[1] > config.goal[1]) continue;
        [config.a, config.b].forEach(o => {
            const nextCoord = move(current.coord, o.delta)
            if (!visited.has(nextCoord.join(','))) {
                heap.push({ cost: current.cost + o.cost, coord: nextCoord });
                visited.add(nextCoord.join(','));
            }
        })
    }
    return 0;
}

const partOne = (configs) => {
    const result = { cost: 0 };
    configs.forEach(c => {
        result.cost += lowestCost(c);
    })
    return result.cost;
}

// console.log(partOne(testI));
// console.log(partOne(i));

const inferredLowestCost = (config) => {
    console.log('Searching for solution to', JSON.stringify(config));
    const result = {}
    const start = { cost: 0, coord: [0, 0] };
    const heap = new Heap((a, b) => a.cost - b.cost || vectorDistance(config.goal, b.coord) - vectorDistance(config.goal, a.coord));
    const visited = new Set();
    heap.push(start);
    while (heap.size()) {
        const current = heap.pop();
        // console.log(JSON.stringify(current));
        if (arrayEquals(config.goal, current.coord)) break;
        else if (current.coord[0] > config.goal[0]) continue;
        else if (current.coord[1] > config.goal[1]) continue;
        [config.a, config.b].forEach(o => {
            const nextCoord = move(current.coord, o.delta)
            if (!visited.has(nextCoord.join(','))) {
                heap.push({ cost: current.cost + o.cost, coord: nextCoord });
                visited.add(nextCoord.join(','));
                const realDistance = difference(config.bigGoal, current.coord);
                if (divisible(realDistance, o.delta)) {
                    const realCost = (divide(realDistance, o.delta) * o.cost) + current.cost;
                    if (typeof result.bigCost === 'undefined' || realCost < result.bigCost) result.bigCost = realCost;
                }
            }
        })
    }
    return result.bigCost || 0;
}

const calculatedCost = (config) => {
    const solutions = [];
    if (divisible(config.bigGoal, config.a.delta)) {
        solutions.push(divide(config.bigGoal, config.a.delta) * config.a.cost);
    }
    if (divisible(config.bigGoal, config.b.delta)) {
        solutions.push(divide(config.bigGoal, config.b.delta) * config.b.cost);
    }
    solutions.sort((a, b) => a - b);

    const d = determinant(config.a.delta, config.b.delta);
    if (d === 0) {
        return solutions.length > 0 ? solutions[0] : 0;
    }

    const n1Top = ((config.bigGoal[0] * config.b.delta[1]) - (config.bigGoal[1] *  config.a.delta[1]));
    const n2Top = ((config.bigGoal[1] * config.a.delta[0]) - (config.bigGoal[0] *  config.b.delta[0]));

    if (n1Top % d !== 0 || n2Top % d !== 0) {
        console.log('Cannot reach big goal with floating point solution', config.bigGoal, n1Top % d, n2Top % d);
        return solutions[0] || 0;
    }
    const n1 = n1Top / d;
    const n2 = n2Top / d;

    if (n1 < 0 || n2 < 0) {
        console.log('Cannot un-push the buttons')
    } else {
        console.log('Solution found!', n1, 'of', config.a.delta, 'at', config.a.cost, 'and', n2, 'of', config.b.delta, 'at', config.b.cost, '| sanity', move(multiply(config.a.delta, n1), multiply(config.b.delta, n2)), config.bigGoal);
        solutions.push(((n1 * config.a.cost) + (n2 * config.b.cost)));
    }
    solutions.sort((a, b) => a - b);
    return solutions[0];
}

export const partTwo = (configs) => {
    const result = { cost: 0 };
    configs.forEach(c => {
        result.cost += calculatedCost(c);
    })
    return result.cost;
}

console.log(partTwo(i));