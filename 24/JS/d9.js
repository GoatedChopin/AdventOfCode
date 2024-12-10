import { getInputs } from './lib/inputs.js';

const i = getInputs(9);
const testI = '12345';

const partOne = (h) => {
    const nulls = [];
    const transformed = [];
    let isFile = true;
    let fileId = 0;
    h.split('').forEach(c => {
        if (isFile) {
            for (let i=0; i < +c; i++) {
                transformed.push(fileId);
            }
            isFile = !isFile;
            fileId++;
        } else {
            for (let i=0; i < +c; i++) {
                nulls.push(transformed.length);
                transformed.push(null);
            }
            isFile = !isFile;
        }
    })
    console.log(transformed);
    nulls.reverse();
    for (let i = transformed.length - 1; i > -1; i--) {
        if (transformed[i] === null) continue;
        if (!nulls.length || nulls[nulls.length - 1] >= i) continue;
        transformed[nulls.pop()] = transformed[i];
        transformed[i] = null;
    }
    return transformed.map((v, i) => v * i).reduce((c, i) => c + i);
};

console.log(partOne(testI));
console.log(partOne('2333133121414131402'));
console.log(partOne(i.toString()));

const partTwo = (h) => {
    const spaceMap = {};
    const fileMap = {};
    const transformed = [];
    let isFile = true;
    let fileId = 0;
    h.split('').forEach(c => {
        if (isFile) {
            fileMap[fileId] = [transformed.length, +c];
            for (let i=0; i < +c; i++) {
                transformed.push(fileId);
            }
            isFile = !isFile;
            fileId++;
        } else {
            spaceMap[transformed.length] = +c;
            for (let i=0; i < +c; i++) {
                // nulls.push(transformed.length);
                transformed.push(null);
            }
            isFile = !isFile;
        }
    })
    // console.log(transformed);
    fileId--;
    while (fileId > 0) {
        for (let i = +h[0]; i < transformed.length; i++) {
            if (spaceMap[i] >= fileMap[fileId][1]) {
                for (let si = 0; si < fileMap[fileId][1]; si++) {
                    transformed[i+si] = fileId;
                    transformed[fileMap[fileId][0]+si] = null;
                }
                spaceMap[i+fileMap[fileId][1]] = spaceMap[i] - fileMap[fileId][1];
                spaceMap[i] = 0;
                break;
            }
        }
        fileId--;
    }
    // console.log(transformed);
    return transformed.map((v, i) => v === null ? 0 : v * i).reduce((c, i) => c + i);
}

console.log(partTwo('2333133121414131402'));  // 2858 Test case is passing but the real input is "too high"
console.log(partTwo(i.toString()));
