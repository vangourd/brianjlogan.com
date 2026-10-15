// Decrypts a newsletter bundle fetched from /encrypted-newsletters/<slug>.bin.
//
// Encrypted file layout (must match newsletter-encrypt.html, BRZ2 envelope):
//   [magic:4 = "BRZ2"][salt:16][iv:12][AES-GCM ciphertext]
//
// Decrypted plaintext layout (multi-file bundle):
//   [count:1]
//   for each file:
//     [mime_len:1][mime:utf-8][fn_len:1][fn:utf-8][size:4 BE][bytes:size]
//
// HTML is rendered inline; other files are offered as downloads. PDFs also have
// an "open in new tab" link.

(function () {
    const ENCRYPTED_PATH = '/encrypted-newsletters';
    const PBKDF2_ITERS = 250000;
    const MAGIC = 'BRZ2';

    const statusEl = () => document.getElementById('content-status');
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
        if (bytes.length < 1) throw new Error('Empty bundle');

        const decoder = new TextDecoder();
        const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        const count = bytes[0];
        let off = 1;
        const files = [];

        function readString() {
            if (off >= bytes.length) throw new Error('Missing string length');
            const length = bytes[off++];
            if (off + length > bytes.length) throw new Error('String exceeds bundle');
            const value = decoder.decode(bytes.slice(off, off + length));
            off += length;
            return value;
        }

        for (let i = 0; i < count; i++) {
            const mime = readString();
            const filename = readString();
            if (off + 4 > bytes.length) throw new Error('Missing file size');
            const size = dv.getUint32(off, false);
            off += 4;
            if (off + size > bytes.length) throw new Error('File exceeds bundle');
            const fileBytes = bytes.slice(off, off + size);
            off += size;
            files.push({ mime, filename, bytes: fileBytes });
        }

        if (off !== bytes.length) throw new Error('Unexpected data after bundle');
        return files;
    }

    function makeFileLink(file, { openInNewTab = false } = {}) {
        const url = URL.createObjectURL(new Blob([file.bytes], {
            type: file.mime || 'application/octet-stream'
        }));
        const link = document.createElement('a');
        link.href = url;
        link.style.color = 'var(--accent-cyan)';
        link.textContent = openInNewTab
            ? `↗ open ${file.filename} in new tab`
            : `↓ download ${file.filename}`;

        if (openInNewTab) {
            link.target = '_blank';
            link.rel = 'noopener';
        } else {
            link.download = file.filename;
        }
        return link;
    }

    function renderBundle(files) {
        const container = document.getElementById('encrypted-content-default');
        const html = files.find(file => file.mime === 'text/html');
        const pdf = files.find(file => file.mime === 'application/pdf');
        const others = files.filter(file => file !== html && file !== pdf);
        const links = [];

        if (pdf) {
            links.push(makeFileLink(pdf), makeFileLink(pdf, { openInNewTab: true }));
        }
        for (const file of others) links.push(makeFileLink(file));

        container.replaceChildren();
        if (links.length) {
            const toolbar = document.createElement('div');
            toolbar.style.cssText = 'display: flex; flex-wrap: wrap; gap: 1rem; margin-bottom: 1.5rem; padding-bottom: 0.75rem; border-bottom: 1px solid var(--border-color);';
            toolbar.append(...links);
            container.appendChild(toolbar);
        }

        if (html) {
            const article = document.createElement('div');
            article.innerHTML = new TextDecoder().decode(html.bytes);
            container.appendChild(article);
        } else {
            const message = document.createElement('p');
            message.textContent = 'This bundle has no inline article.';
            container.appendChild(message);
        }
    }

    async function unlock(slug, passcode) {
        status('Fetching encrypted article…');
        let resp;
        try {
            resp = await fetch(`${ENCRYPTED_PATH}/${encodeURIComponent(slug)}.bin`, { cache: 'no-store' });
        } catch (e) {
            status('Network error fetching the article.', 'error');
            return;
        }
        if (!resp.ok) {
            status('No article found for that slug.', 'error');
            return;
        }

        const buf = new Uint8Array(await resp.arrayBuffer());
        if (buf.length < 32 || new TextDecoder().decode(buf.slice(0, 4)) !== MAGIC) {
            status('File format not recognized (expected a BRZ2 bundle).', 'error');
            return;
        }
        const salt = buf.slice(4, 20);
        const iv = buf.slice(20, 32);
        const ciphertext = buf.slice(32);

        status('Deriving key…');
        const key = await deriveKey(passcode, salt);

        status('Decrypting…');
        let plainBuf;
        try {
            plainBuf = await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, ciphertext);
        } catch (e) {
            status('Wrong key (decryption failed).', 'error');
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
        const form = document.getElementById('unlock-form');
        if (form) {
            form.addEventListener('submit', (e) => {
                e.preventDefault();
                const slug = document.getElementById('content-slug').value.trim();
                const key = document.getElementById('content-key').value;
                if (slug && key) unlock(slug, key);
            });
        }

        // Auto-unlock from URL fragment: #<slug>.<key>
        const hash = window.location.hash.slice(1);
        const dot = hash.indexOf('.');
        if (dot <= 0) return;

        let slug;
        let key;
        try {
            slug = decodeURIComponent(hash.slice(0, dot));
            key = decodeURIComponent(hash.slice(dot + 1));
        } catch (e) {
            status('The article link is malformed.', 'error');
            return;
        }
        if (!slug || !key) return;

        // Strip the key from the address bar before fetching or rendering content.
        history.replaceState(null, '', window.location.pathname + window.location.search);
        unlock(slug, key);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
