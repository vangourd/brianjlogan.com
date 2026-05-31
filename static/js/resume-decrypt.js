// Decrypts a resume blob fetched from /encrypted-resumes/<id>.bin.
//
// Encrypted file layout (must match resume-tool.html, BRZ2 envelope):
//   [magic:4 = "BRZ2"][salt:16][iv:12][AES-GCM ciphertext]
//
// Decrypted plaintext layout (multi-file bundle):
//   [count:1]
//   for each file:
//     [mime_len:1][mime:utf-8][fn_len:1][fn:utf-8][size:4 BE][bytes:size]
//
// Routing by MIME after decrypt:
//   text/html         → injected inline into #resume-default via innerHTML
//   application/pdf   → download link + "open in new tab" link
//   anything else     → download link only

(function () {
    const ENCRYPTED_PATH = '/encrypted-resumes';
    const PBKDF2_ITERS = 250000;
    const MAGIC = 'BRZ2';

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

    function parseBundle(bytes) {
        const decoder = new TextDecoder();
        const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        const count = bytes[0];
        let off = 1;
        const files = [];
        for (let i = 0; i < count; i++) {
            const mimeLen = bytes[off]; off += 1;
            const mime = decoder.decode(bytes.slice(off, off + mimeLen)); off += mimeLen;
            const fnLen = bytes[off]; off += 1;
            const filename = decoder.decode(bytes.slice(off, off + fnLen)); off += fnLen;
            const size = dv.getUint32(off, false); off += 4;
            const fileBytes = bytes.slice(off, off + size); off += size;
            files.push({ mime, filename, bytes: fileBytes });
        }
        return files;
    }

    function renderBundle(files) {
        const container = document.getElementById('resume-default');
        const html = files.find(f => f.mime === 'text/html');
        const pdf  = files.find(f => f.mime === 'application/pdf');
        const others = files.filter(f => f !== html && f !== pdf);

        // Toolbar at the top with download / open-in-tab actions for the PDF, plus
        // any other (non-html, non-pdf) attachments as download links.
        const links = [];
        if (pdf) {
            const pdfUrl = URL.createObjectURL(new Blob([pdf.bytes], { type: 'application/pdf' }));
            links.push(
                `<a href="${pdfUrl}" download="${pdf.filename}" style="color: var(--accent-cyan);">↓ download pdf</a>`,
                `<a href="${pdfUrl}" target="_blank" rel="noopener" style="color: var(--accent-cyan);">↗ open pdf in new tab</a>`
            );
        }
        for (const f of others) {
            const url = URL.createObjectURL(new Blob([f.bytes], { type: f.mime || 'application/octet-stream' }));
            links.push(`<a href="${url}" download="${f.filename}" style="color: var(--accent-cyan);">↓ download ${f.filename}</a>`);
        }

        const htmlFragment = html
            ? new TextDecoder().decode(html.bytes)
            : '<p style="color: var(--accent-yellow);">(no inline HTML in bundle)</p>';

        container.innerHTML = `
            <div style="display: flex; flex-wrap: wrap; gap: 1rem; margin-bottom: 1.5rem; padding-bottom: 0.75rem; border-bottom: 1px solid var(--border-color);">
                ${links.join('')}
            </div>
            ${htmlFragment}
        `;
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
            status('No resume found for that id.', 'error');
            return;
        }
        const buf = new Uint8Array(await resp.arrayBuffer());
        if (buf.length < 32 || new TextDecoder().decode(buf.slice(0, 4)) !== MAGIC) {
            status('File format not recognized (expected BRZ2 bundle).', 'error');
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

        let files;
        try {
            files = parseBundle(new Uint8Array(plainBuf));
        } catch (e) {
            status('Decrypted bundle is malformed.', 'error');
            return;
        }

        renderBundle(files);
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
