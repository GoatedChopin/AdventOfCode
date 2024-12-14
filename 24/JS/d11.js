import { List, Item } from 'linked-list';

import { getInputs } from './lib/inputs.js';


const testI = '125 17'.split(' ');


const partOne = (r, steps=25) => {
    const next = (n) => {
        if (n === 0) return 1;
        else if (('' + n).length % 2 === 0) {
            const sN = '' + n;
            const left = sN.slice(0, sN.length / 2);
            const right = sN.slice(sN.length / 2, sN.length);
            return [left, right];
        }
    }

    const list = new List(...r.map(n => new Item(n)));
    for (let s = 0; s < steps; s++) {
        let it = list.head;
        for (let i = 0; i < list.size; i++) {
            const iter = next(it);
            const nxt = it.next;
            if (Array.isArray(iter)) {
                const iterItems = iter.map(n => new Item(n));
                nxt.prev = iterItems[1];
                iterItems[1].next = nxt;
                it.
            }
            it = it.next;
        }
    }
}

console.log(partOne(testI));