'use strict';

// T-101: `.uw` dosyaları açılınca ve kaydedilince `uw check --json <klasör>` çalışır, çıktısındaki her
// hata kırmızı dalgalı çizgi ve Problems panelinde görünür. Düz CommonJS'tir: npm paketi gerekmez ve
// `vscode` yalnızca `activate` içinde çağrılır, böylece bu dosya `node` ile de yüklenebilir
// (editors/vscode/test/parse.test.js).

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

const SOURCE = 'uw';
const DEFAULT_COMMAND = 'uw';

/// `uw check --json` çıktısı: satır başına bir JSON nesnesi, son satır `{"summary":{...}}`.
/// Nesneleri VS Code'un beklediği biçime çevirir: satır/sütun 0 tabanlı, `help` mesaja eklenir.
function parseCheckOutput(text) {
  const diagnostics = [];
  for (const raw of String(text == null ? '' : text).split(/\r?\n/)) {
    const line = raw.trim();
    if (line === '') continue;
    let item;
    try {
      item = JSON.parse(line);
    } catch (e) {
      continue; // tanı değil (kullanım metni, yarım satır): yoksay
    }
    if (typeof item !== 'object' || item === null) continue;
    // `{"summary":{...}}` satırında `file` yoktur; böyle satırlar tanı üretmez.
    if (typeof item.file !== 'string' || typeof item.line !== 'number') continue;
    const startLine = Math.max(item.line - 1, 0);
    const startCharacter = Math.max((typeof item.column === 'number' ? item.column : 1) - 1, 0);
    let endLine = typeof item.end_line === 'number' ? Math.max(item.end_line - 1, 0) : startLine;
    let endCharacter = typeof item.end_column === 'number' ? Math.max(item.end_column - 1, 0) : startCharacter;
    if (endLine < startLine || (endLine === startLine && endCharacter < startCharacter)) {
      endLine = startLine;
      endCharacter = startCharacter;
    }
    const message = typeof item.message === 'string' ? item.message : '';
    diagnostics.push({
      file: item.file,
      range: {
        start: { line: startLine, character: startCharacter },
        end: { line: endLine, character: endCharacter },
      },
      severity: typeof item.severity === 'string' ? item.severity : 'error',
      code: item.code == null ? undefined : String(item.code),
      message: typeof item.help === 'string' && item.help !== '' ? message + '\n' + item.help : message,
    });
  }
  return diagnostics;
}

/// Dosyanın en yakın `uw.toml` içeren üst klasörü; yoksa dosyanın kendi klasörü.
function projectFolder(file) {
  const own = path.dirname(file);
  let dir = own;
  for (;;) {
    if (fs.existsSync(path.join(dir, 'uw.toml'))) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) return own;
    dir = parent;
  }
}

/// `uw check --json <folder>`: çözülünce stdout (çıkış kodu 1 olsa bile hatalar oradadır).
function runCheck(command, folder) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, ['check', '--json', folder], { windowsHide: true });
    let stdout = '';
    child.stdout.setEncoding('utf8');
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', () => {});
    child.on('error', reject);
    child.on('close', () => resolve(stdout));
  });
}

function activate(context) {
  const vscode = require('vscode');
  const collection = vscode.languages.createDiagnosticCollection('uw');
  context.subscriptions.push(collection);

  const severities = {
    error: vscode.DiagnosticSeverity.Error,
    warning: vscode.DiagnosticSeverity.Warning,
    info: vscode.DiagnosticSeverity.Information,
    hint: vscode.DiagnosticSeverity.Hint,
  };
  let missingToolWarned = false;

  const check = async (document) => {
    if (!document || document.languageId !== 'uw' || document.uri.scheme !== 'file') return;
    const configured = vscode.workspace.getConfiguration('uw').get('path', DEFAULT_COMMAND);
    const command = configured || DEFAULT_COMMAND;
    let stdout;
    try {
      stdout = await runCheck(command, projectFolder(document.uri.fsPath));
    } catch (error) {
      if (error && error.code === 'ENOENT') {
        if (!missingToolWarned) {
          missingToolWarned = true;
          vscode.window.showWarningMessage(
            `uw bulunamadı ("${command}"). uw'yu PATH'e ekleyin veya "uw.path" ayarını doldurun.`,
          );
        }
      } else {
        vscode.window.showWarningMessage(`uw check çalıştırılamadı: ${(error && error.message) || error}`);
      }
      return;
    }

    const byFile = new Map();
    for (const item of parseCheckOutput(stdout)) {
      const { start, end } = item.range;
      const diagnostic = new vscode.Diagnostic(
        new vscode.Range(start.line, start.character, end.line, end.character),
        item.message,
        severities[item.severity] || vscode.DiagnosticSeverity.Error,
      );
      diagnostic.source = SOURCE;
      if (item.code !== undefined) diagnostic.code = item.code;
      const list = byFile.get(item.file);
      if (list) list.push(diagnostic);
      else byFile.set(item.file, [diagnostic]);
    }

    collection.clear();
    for (const [file, list] of byFile) collection.set(vscode.Uri.file(file), list);
  };

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(check),
    vscode.workspace.onDidSaveTextDocument(check),
  );
}

function deactivate() {}

module.exports = { activate, deactivate, parseCheckOutput };
