var fs = require('fs');

const file = fs.readFileSync('./src/twain.h').toString();
const lines = file.split('\n');
const constLines = lines.slice(622, 2082);

const out = [];

let map = {};
let raw = {};
for (const line of constLines) {
    if (line.startsWith('#define ')) {
        let [_, key, value] = line.split(' ').filter(it => it).map(it => it.replace('\r', ''));
        let [k, ...variant] = key.split('_');
        raw[key] = value;
        variant = variant.join('_');
        let items = map[k] = map[k] || {};
        if (!isNaN(variant[0])) {
            variant = k + '_' + variant;
        }
        if (value.indexOf('_') !== -1) {
            value = raw[value] || value;// value.replace('_', '::') + ' as isize'
        }

        if (value.startsWith('0x') && value.endsWith('L')) {
            value = value.slice(0, value.length - 1);
        }

        if (value.startsWith('0X')) {
            value = value.replace('0X', '0x');
        }

        if (value.endsWith('????')) {
            variant = '// ' + variant;
        }

        items[value] = items[value] || [];
        items[value].push(variant);

    } else {
        out.push(line);
    }
}

out.push('*/');
out.unshift('/*');

Object.keys(map).map(kind => {
    let items = map[kind];
    out.unshift('#[derive(Debug)]', '#[repr(C)]', `pub enum ${kind} {`, ...Object.keys(items).map(key => {
        return `    ${items[key].join('_OR_')} = ${key},`
    }), '}');
});


fs.writeFileSync('./src/data.rs', out.join('\n'));

