import { getInputs } from "./lib/inputs.js";

const t = `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`;

const i = getInputs(7).toString().split('\n').map(l => {
    const line = l.split(': ');
    const vals = line[1].split(' ').map(i => +i);
    const key = +line[0];
    return { key, vals }
});
const testInputs = t.split('\n').map(l => {
    const line = l.split(': ');
    const vals = line[1].split(' ').map(i => +i);
    const key = +line[0];
    return { key, vals }
});

console.log(testInputs);

const getPermutations = (arr, length) => {
    // Base case: If length is 1, return the elements as individual arrays
    if (length === 1) {
        return arr.map(item => [item]);
    }

    // Recursive case: Get permutations of smaller length and extend with the current array elements
    let result = [];
    const smallerPerms = getPermutations(arr, length - 1);

    for (let i = 0; i < arr.length; i++) {
        for (let perm of smallerPerms) {
            result.push([arr[i], ...perm]);
        }
    }

    return result;
}

// Example usage:
//   const inputArray = ['+', '*'];
//   const permLength = 3;
//   const permutations = getPermutations(inputArray, permLength);

//   console.log(permutations);


const partOne = (a) => {
    let valid = 0
    a.forEach(args => {
        const { key, vals } = args;
        const perms = getPermutations(['+', '*'], vals.length - 1);
        perms.sort((a, b) => +a.map(op => op === '+' ? '1' : '2').join('') - +b.map(op => op === '+' ? '1' : '2').join(''))
        const evals = perms.map(p => {
            let parenth = false;
            let pi = 0
            return vals.reduce((c, i) => {
                i = i + '';
                if (parenth) {
                    pi++;
                    return '(' + c + p[pi-1] + i + ')'
                }
                else {
                    parenth = true;
                    pi++;
                    return '(' + c + p[pi-1] + i + ')';
                }
            });
        });
        for (let i = 0; i < evals.length; i++) {
            const e = evals[i];
            // console.log(e);
            if (eval(e) === key) {
                valid += key;
                // console.log(e, key, valid);
                break;
            }
        }
    });
    return valid;
}

console.log(partOne(testInputs))
// console.log(partOne(i))

const opMap = {
    '+': (a, b) => a + b,
    '*': (a, b) => a * b,
    '||': (a, b) => +`${a}${b}`,
}

const partTwo = (a) => {
    let valid = 0
    a.forEach(args => {
        const { key, vals } = args;
        const perms = getPermutations(['+', '*', '||'], vals.length - 1);
        perms.sort((a, b) => +a.map(op => op === '+' ? '1' : '2').join('') - +b.map(op => op === '+' ? '1' : '2').join(''))
        const evals = perms.map(p => {
            let pi = 0;
            return vals.reduce((c, i) => {
                if (c > key || c == null) return null;
                if (i === 0) return 0;
                pi++;
                return opMap[p[pi-1]](c, i);
            });
        }).filter(v => v !== null);
        for (let i = 0; i < evals.length; i++) {
            if (evals[i] === key) {
                valid += key;
                break;
            }
        }
    });
    return valid;
}

console.log(partTwo(testInputs));
console.log(partTwo(i));