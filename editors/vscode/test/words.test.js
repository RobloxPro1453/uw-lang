'use strict';

// T-104: `node editors/vscode/test/words.test.js`. Çerçeve yok (npm paketi gerekmez): kontroller
// burada toplanır, biri bile düşerse çıkış kodu 1 olur.

const assert = require('assert');
const path = require('path');

const { parseWords } = require(path.join(__dirname, '..', 'extension.js'));

const checks = [];
function check(name, fn) {
  try {
    fn();
    checks.push({ name, ok: true });
  } catch (error) {
    checks.push({ name, ok: false, error });
  }
}

const LINE = '{"label":"praven","kind":"name","detail":"Praven (party)"}';

check('bir JSON satırı bir kelime olur', () => {
  assert.deepStrictEqual(parseWords(LINE + '\n'), [
    { label: 'praven', kind: 'name', detail: 'Praven (party)' },
  ]);
});

check('dört tür de korunur', () => {
  const text =
    LINE +
    '\n{"label":"say","kind":"word","detail":"kendi `fn` kelimeniz"}' +
    '\n{"label":"player.gold","kind":"field","detail":"nesne alanı"}' +
    '\n{"label":"override","kind":"keyword","detail":"var olan bir kaydı değiştirir"}';
  const words = parseWords(text);
  assert.deepStrictEqual(
    words.map((word) => word.kind),
    ['name', 'word', 'field', 'keyword'],
  );
  assert.strictEqual(words[1].label, 'say');
  assert.strictEqual(words[3].label, 'override');
});

check('boş ve JSON olmayan satırlar yoksayılır', () => {
  assert.deepStrictEqual(parseWords(''), []);
  assert.deepStrictEqual(parseWords('uw 0.1.0\nkullanım: uw words\n\n'), []);
});

check('label ya da kind yoksa satır yoksayılır', () => {
  assert.deepStrictEqual(parseWords('{"label":"x"}\n{"kind":"word"}\n{"label":3,"kind":"word"}\n'), []);
});

check('detail yoksa boş olur', () => {
  const words = parseWords('{"label":"say","kind":"word"}');
  assert.strictEqual(words.length, 1);
  assert.strictEqual(words[0].detail, '');
});

check('eksik detail sütunu tamamlamayı bozmaz', () => {
  const words = parseWords(LINE + '\r\n{"label":"say","kind":"word"}\r\n');
  assert.deepStrictEqual(
    words.map((word) => word.label),
    ['praven', 'say'],
  );
});

let failed = 0;
for (const result of checks) {
  if (result.ok) {
    console.log('ok   ' + result.name);
  } else {
    failed += 1;
    console.error('FAIL ' + result.name + ': ' + (result.error && result.error.message));
  }
}
console.log(`${checks.length - failed}/${checks.length} kontrol geçti`);
process.exit(failed === 0 ? 0 : 1);
