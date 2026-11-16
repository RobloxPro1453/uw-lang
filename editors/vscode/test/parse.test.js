'use strict';

// T-101: `node editors/vscode/test/parse.test.js`. Çerçeve yok (npm paketi gerekmez): kontroller
// burada toplanır, biri bile düşerse çıkış kodu 1 olur.

const assert = require('assert');
const path = require('path');

const { parseCheckOutput } = require(path.join(__dirname, '..', 'extension.js'));

const checks = [];
function check(name, fn) {
  try {
    fn();
    checks.push({ name, ok: true });
  } catch (error) {
    checks.push({ name, ok: false, error });
  }
}

// T-100 sözleşmesi: 1 tabanlı satır/sütun.
const LINE =
  '{"file":"/mods/kalkan/lords.uw","line":3,"column":30,"end_line":3,"end_column":49,' +
  '"severity":"error","code":"E0001","message":"bilinmeyen ad: $lord",' +
  '"help":"`$lord` tanımlı değil; bir lord adı yazın."}';

check('1 tabanlı aralık 0 tabanlı aralığa çevrilir', () => {
  const diagnostics = parseCheckOutput(LINE + '\n');
  assert.strictEqual(diagnostics.length, 1);
  const d = diagnostics[0];
  assert.strictEqual(d.file, '/mods/kalkan/lords.uw');
  // 3. satır 30. sütun → 2. satır 29. karakter; bitiş 49 → 48.
  assert.deepStrictEqual(d.range, { start: { line: 2, character: 29 }, end: { line: 2, character: 48 } });
  assert.strictEqual(d.severity, 'error');
  assert.strictEqual(d.code, 'E0001');
  assert.strictEqual(d.message, 'bilinmeyen ad: $lord\n`$lord` tanımlı değil; bir lord adı yazın.');
});

check('aralık hâlâ 1 tabanlı olsaydı düşerdi', () => {
  const d = parseCheckOutput(LINE)[0];
  assert.notDeepStrictEqual(d.range.start, { line: 3, character: 30 });
  assert.notDeepStrictEqual(d.range.end, { line: 3, character: 49 });
});

check('özet satırı tanı üretmez', () => {
  const text = LINE + '\n{"summary":{"errors":1,"warnings":0}}\n';
  assert.strictEqual(parseCheckOutput(text).length, 1);
  assert.deepStrictEqual(parseCheckOutput('{"summary":{"errors":0,"warnings":2}}'), []);
});

check('boş çıktı ve JSON olmayan satırlar yoksayılır', () => {
  assert.deepStrictEqual(parseCheckOutput(''), []);
  assert.deepStrictEqual(parseCheckOutput('uw 0.1.0\nkullanım: uw check <klasör>\n'), []);
});

check('birden çok tanı sırayı ve dosyaları korur', () => {
  const second =
    '{"file":"/mods/kalkan/troops.uw","line":10,"column":1,"end_line":11,"end_column":5,' +
    '"severity":"warning","code":"W0002","message":"kullanılmayan mod"}';
  const diagnostics = parseCheckOutput(LINE + '\r\n' + second + '\r\n');
  assert.strictEqual(diagnostics.length, 2);
  assert.strictEqual(diagnostics[1].file, '/mods/kalkan/troops.uw');
  assert.deepStrictEqual(diagnostics[1].range, { start: { line: 9, character: 0 }, end: { line: 10, character: 4 } });
  assert.strictEqual(diagnostics[1].severity, 'warning');
  assert.strictEqual(diagnostics[1].message, 'kullanılmayan mod'); // help yoksa eklenmez
});

check('bitiş alanları yoksa aralık başlangıçta biter', () => {
  const line = '{"file":"/m/a.uw","line":1,"column":1,"severity":"error","code":"E0001","message":"x"}';
  const d = parseCheckOutput(line)[0];
  assert.deepStrictEqual(d.range, { start: { line: 0, character: 0 }, end: { line: 0, character: 0 } });
  assert.strictEqual(d.message, 'x');
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
