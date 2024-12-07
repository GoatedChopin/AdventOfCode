

const t = `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`;

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
            return vals.map((v, i) => {
                if (p[i]) return v + p[i];
                return v;
            }).join('');
        });
        for (let i = 0; i < evals.length; i++) {
            const e = evals[i];
            console.log(e);
            if (eval(e) === key) {
                valid++;
                console.log(e, key, valid);
                break;
            }
        }
    });
    return valid;
}

console.log(partOne(testInputs))