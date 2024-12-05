import fs from 'fs';

export function getInputs(day=1) {
    const path = `C:\\Users\\Colby\\Programming\\AdventOfCode\\24\\JS\\inputs\\${day}.txt`;
    const file = fs.readFileSync(path);
    return file;
}