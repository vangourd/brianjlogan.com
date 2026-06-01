// Decrypts a resume blob fetched from /encrypted-resumes/<id>.bin.
//
// Encrypted file layout (must match resume-tool.html):
//   [magic:4 = "BRZ1"][salt:16][iv:12][AES-GCM ciphertext]
// Decrypted plaintext layout:
//   [mimeLen:1][mime:utf-8 mimeLen bytes][fnLen:1][filename:utf-8 fnLen bytes][file bytes]

(function () {
    const ENCRYPTED_PATH = '/encrypted-resumes';
    const PBKDF2_ITERS = 250000;
    const MAGIC = 'BRZ1';

    const statusEl = () => document.getElementById('resume-status');
    function status(msg, kind) {
        const el = statusEl();
        if (!el) return;
        el.textContent = msg;
        el.style.color = kind === 'error' ? '#ff7b72'
                      : kind === 'ok'    ? 'var(--accent-green)'
                      : 'var(--accent-yellow)';
    }

    async function deriveKey(passcode, salt) {
        const material = await crypto.subtle.importKey(
            'raw', new TextEncoder().encode(passcode),
            'PBKDF2', false, ['deriveKey']
        );
        return crypto.subtle.deriveKey(
            { name: 'PBKDF2', salt, iterations: PBKDF2_ITERS, hash: 'SHA-256' },
            material,
            { name: 'AES-GCM', length: 256 },
            false, ['decrypt']
        );
    }

    function parsePlaintext(bytes) {
        let off = 0;
        const mimeLen = bytes[off]; off += 1;
        const mime = new TextDecoder().decode(bytes.slice(off, off + mimeLen)); off += mimeLen;
        const fnLen = bytes[off]; off += 1;
        const filename = new TextDecoder().decode(bytes.slice(off, off + fnLen)); off += fnLen;
        const file = bytes.slice(off);
        return { mime, filename, file };
    }

    async function unlock(id, passcode) {
        status('Fetching encrypted file…');
        let resp;
        try {
            resp = await fetch(`${ENCRYPTED_PATH}/${encodeURIComponent(id)}.bin`, { cache: 'no-store' });
        } catch (e) {
            status('Network error fetching the file.', 'error');
            return;
        }
        if (!resp.ok) {
            status(`No resume found for that id.`, 'error');
            return;
        }
        const buf = new Uint8Array(await resp.arrayBuffer());
        if (buf.length < 32 || new TextDecoder().decode(buf.slice(0, 4)) !== MAGIC) {
            status('File format not recognized.', 'error');
            return;
        }
        const salt = buf.slice(4, 20);
        const iv   = buf.slice(20, 32);
        const ct   = buf.slice(32);

        status('Deriving key…');
        const key = await deriveKey(passcode, salt);

        status('Decrypting…');
        let plainBuf;
        try {
            plainBuf = await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, ct);
        } catch (e) {
            status('Wrong passcode (decryption failed).', 'error');
            return;
        }

        const { mime, filename, file } = parsePlaintext(new Uint8Array(plainBuf));
        const blob = new Blob([file], { type: mime || 'application/octet-stream' });
        const url  = URL.createObjectURL(blob);

        const container = document.getElementById('resume-default');
        container.innerHTML = `
            <p style="color: var(--accent-green);">$ decrypted — ${filename}</p>
            <p><a id="resume-download" download style="color: var(--accent-cyan);">↓ download</a></p>
            <iframe id="resume-preview" src="${url}"
                    style="width: 100%; height: 80vh; border: 1px solid var(--border-color); margin-top: 1rem; background: white;"></iframe>
        `;
        const dl = document.getElementById('resume-download');
        dl.href = url;
        dl.download = filename;
        status('', 'ok');
    }

    function init() {
        // Auto-unlock from URL fragment: #<id>.<passcode>
        const hash = window.location.hash.slice(1);
        if (hash) {
            const dot = hash.indexOf('.');
            if (dot > 0) {
                const id = decodeURIComponent(hash.slice(0, dot));
                const key = decodeURIComponent(hash.slice(dot + 1));
                // Strip from URL so the passcode isn't visible after load.
                history.replaceState(null, '', window.location.pathname);
                unlock(id, key);
                return;
            }
        }

        const form = document.getElementById('unlock-form');
        if (!form) return;
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            const id  = document.getElementById('resume-id').value.trim();
            const key = document.getElementById('resume-key').value;
            if (id && key) unlock(id, key);
        });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
