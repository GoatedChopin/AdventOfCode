import fs from 'fs';
import path from 'path';

export function getInputs(day=1) {
    // const path = `C:\\Users\\Colby\\Programming\\AdventOfCode\\24\\JS\\inputs\\${day}.txt`;
    // const filePath = `/home/colby/code/AdventOfCode/24/JS/inputs/${day}.txt`
    const filePath = path.resolve('inputs', `${day}.txt`);
    const file = fs.readFileSync(filePath);
    return file;
}