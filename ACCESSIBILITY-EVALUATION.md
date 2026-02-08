# WCAG Accessibility Evaluation
## brianjlogan.com

**Evaluation Date:** 2026-02-08
**WCAG Version:** 2.1
**Conformance Target:** AA

---

## Executive Summary

The site has several **critical accessibility barriers** that would prevent it from meeting WCAG 2.1 AA standards. The primary issues involve keyboard navigation, focus management, form labels, and semantic markup for interactive elements.

**Estimated Conformance Level:** Currently fails WCAG 2.1 Level A

---

## Critical Issues (Must Fix)

### 1. ❌ **Keyboard Navigation - Navigation Menu** (WCAG 2.1.1)
**Severity:** Critical
**Location:** `themes/terminal/templates/base.html:40, 50`

**Issue:**
- Navigation toggle uses `onclick` on `<div>` elements instead of `<button>`
- Folder headers use `onclick` on `<div>` elements
- Not accessible via keyboard (Tab key won't focus these elements)

**Code:**
```html
<div class="nav-header" onclick="toggleQuickMenu()">
<div class="nav-folder-header" onclick="toggleFolder(event, 'tech')">
```

**Fix Required:**
```html
<button class="nav-header" onclick="toggleQuickMenu()" aria-expanded="false" aria-controls="quick-menu-links">
<button class="nav-folder-header" onclick="toggleFolder(event, 'tech')" aria-expanded="false" aria-controls="submenu-tech">
```

---

### 2. ❌ **Form Labels Missing** (WCAG 1.3.1, 3.3.2)
**Severity:** Critical
**Location:** `base.html:34, 91`

**Issue:**
- Shell input field lacks associated `<label>`
- Vim command input lacks associated `<label>`
- Search input needs verification

**Code:**
```html
<input type="text" id="shell-input" class="shell-input" autocomplete="off" style="display: none;" />
<input type="text" id="vim-command-input" class="vim-command-input" autocomplete="off" />
```

**Fix Required:**
```html
<label for="shell-input" class="visually-hidden">Terminal command input</label>
<input type="text" id="shell-input" class="shell-input" autocomplete="off" style="display: none;" aria-label="Terminal command input" />

<label for="vim-command-input" class="visually-hidden">Vim command input</label>
<input type="text" id="vim-command-input" class="vim-command-input" autocomplete="off" aria-label="Vim command" />
```

---

### 3. ❌ **No Skip Navigation Link** (WCAG 2.4.1)
**Severity:** High
**Location:** Missing from `base.html`

**Issue:**
- No way for keyboard users to skip repetitive navigation
- Screen reader users must tab through entire header on every page

**Fix Required:**
Add at the very top of `<body>`:
```html
<a href="#main-content" class="skip-link">Skip to main content</a>
```

CSS:
```scss
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: var(--accent-cyan);
  color: var(--bg-primary);
  padding: 8px;
  text-decoration: none;
  z-index: 100;

  &:focus {
    top: 0;
  }
}
```

---

### 4. ❌ **Image Alt Text Not Descriptive** (WCAG 1.1.1)
**Severity:** Medium
**Location:** `base.html:29`

**Issue:**
- Portrait image has alt="Portrait" which doesn't convey meaningful information

**Code:**
```html
<img src="{{ get_url(path="me.png") }}" alt="Portrait" class="portrait" id="portrait-img">
```

**Fix Required:**
```html
<img src="{{ get_url(path="me.png") }}" alt="Brian Logan's profile photo" class="portrait" id="portrait-img">
```

---

### 5. ⚠️ **Focus Indicators Not Visible** (WCAG 2.4.7)
**Severity:** High
**Location:** `themes/terminal/sass/style.scss`

**Issue:**
- No visible focus indicators defined in CSS
- Keyboard users cannot see where focus is

**Fix Required:**
Add to SCSS:
```scss
*:focus {
  outline: 2px solid var(--accent-cyan);
  outline-offset: 2px;
}

a:focus,
button:focus {
  outline: 2px solid var(--accent-cyan);
  outline-offset: 2px;
}
```

---

### 6. ⚠️ **Links with JavaScript URLs** (WCAG 2.4.4)
**Severity:** Medium
**Location:** `index.html:69`

**Issue:**
- Year filter links use `href="javascript:void(0)"` which is not semantic

**Code:**
```html
<a href="javascript:void(0)" class="year-link">
```

**Fix Required:**
```html
<button type="button" class="year-link" data-year="${year}">
```

---

### 7. ⚠️ **ARIA States Missing** (WCAG 4.1.2)
**Severity:** Medium
**Location:** Navigation menus and collapsible sections

**Issue:**
- Collapsible navigation lacks `aria-expanded` state
- Folders lack proper ARIA attributes

**Fix Required:**
```html
<button class="nav-header"
        onclick="toggleQuickMenu()"
        aria-expanded="false"
        aria-controls="quick-menu-links">
  <span>Quick Menu</span>
  <span class="nav-toggle" aria-hidden="true">▼</span>
</button>
```

Update JavaScript to toggle `aria-expanded`:
```javascript
function toggleQuickMenu() {
    const menu = document.getElementById('quick-menu');
    const button = menu.querySelector('.nav-header');
    const isExpanded = button.getAttribute('aria-expanded') === 'true';

    button.setAttribute('aria-expanded', !isExpanded);
    menu.classList.toggle('collapsed');
}
```

---

## Moderate Issues

### 8. ⚠️ **Color Contrast May Be Insufficient** (WCAG 1.4.3)
**Severity:** Medium
**Needs Testing**

**Potential Issues:**
- Secondary text `--text-secondary: #8b949e` on `--bg-primary: #0d1117`
  - **Ratio needed:** 4.5:1 for normal text, 3:1 for large text
  - **Actual ratio:** Needs calculation

- Accent yellow `--accent-yellow: #ffa657` on backgrounds
- Link colors on various backgrounds

**Action Required:**
Test with contrast checker tool:
- Chrome DevTools Contrast Checker
- WebAIM Contrast Checker (https://webaim.org/resources/contrastchecker/)

**Colors to verify:**
```
Primary: #e6edf3 on #0d1117
Secondary: #8b949e on #0d1117
Green: #7ee787 on #0d1117
Cyan: #79c0ff on #0d1117
Yellow: #ffa657 on #0d1117
```

---

### 9. ⚠️ **Typewriter Effect Not Accessible** (WCAG 2.2.2)
**Severity:** Low
**Location:** `base.html:228-234`

**Issue:**
- Animated typewriter effect may be distracting
- No way to pause or disable animation
- Click-to-activate shell is not keyboard accessible

**Fix Required:**
```html
<button id="typewriter-btn"
        class="typewriter-text"
        aria-label="Activate terminal prompt">
  <span id="typewriter"><span class="cursor">_</span></span>
</button>
```

Add prefers-reduced-motion support:
```scss
@media (prefers-reduced-motion: reduce) {
  .cursor {
    animation: none;
  }
}
```

---

### 10. ℹ️ **Language Attribute** (WCAG 3.1.1)
**Status:** ✅ PASS
**Location:** `base.html:2`

Correctly implements:
```html
<html lang="{{ lang | default(value="en") }}">
```

---

### 11. ℹ️ **Page Title** (WCAG 2.4.2)
**Status:** ✅ PASS
**Location:** `base.html:6`, `page.html:3`

Correctly implements descriptive titles.

---

### 12. ℹ️ **Semantic HTML Structure** (WCAG 1.3.1)
**Status:** ✅ MOSTLY PASS

**Good:**
- Proper use of `<header>`, `<nav>`, `<main>`, `<footer>`, `<article>`
- Heading hierarchy appears correct
- List markup for navigation and posts

**Needs Improvement:**
- Interactive divs should be buttons
- Some sections could use `<section>` with headings

---

### 13. ℹ️ **Responsive/Mobile Accessibility** (WCAG 1.4.10)
**Status:** ✅ PASS

Good responsive design with proper viewport and media queries.

---

## Recommendations for Improvement

### High Priority
1. Convert all `onclick` divs to `<button>` elements
2. Add labels to all form inputs
3. Add skip navigation link
4. Implement visible focus indicators
5. Add ARIA states to collapsible elements

### Medium Priority
6. Verify and fix color contrast ratios
7. Improve image alt text descriptions
8. Convert javascript: links to buttons
9. Add reduced motion support

### Low Priority (Nice to Have)
10. Add landmark roles for older screen readers
11. Consider adding a dark/light mode toggle for users with light sensitivity
12. Add aria-live regions for dynamic content
13. Consider adding heading structure outline in docs

---

## Testing Recommendations

### Automated Testing Tools
- **axe DevTools** (Browser extension)
- **WAVE** (Web Accessibility Evaluation Tool)
- **Lighthouse** (Chrome DevTools)
- **pa11y** (Command line testing)

### Manual Testing Required
1. **Keyboard Navigation**
   - Tab through all interactive elements
   - Verify all functionality accessible without mouse
   - Test Escape key to close modals/menus

2. **Screen Reader Testing**
   - NVDA (Windows, free)
   - JAWS (Windows, commercial)
   - VoiceOver (macOS/iOS, built-in)

3. **Color Contrast**
   - WebAIM Contrast Checker
   - Chrome DevTools Contrast tool

4. **Zoom Testing**
   - Test at 200% zoom
   - Verify no horizontal scrolling
   - Ensure all text remains readable

---

## Quick Wins Checklist

- [ ] Add skip navigation link
- [ ] Convert navigation divs to buttons
- [ ] Add `aria-expanded` to collapsible sections
- [ ] Add labels to shell/vim inputs
- [ ] Improve portrait alt text
- [ ] Add visible focus indicators
- [ ] Add `@media (prefers-reduced-motion: reduce)` support
- [ ] Convert javascript:void(0) links to buttons
- [ ] Test color contrast ratios
- [ ] Run automated accessibility scanner

---

## Estimated Effort

**To reach WCAG 2.1 Level A:** 8-12 hours
**To reach WCAG 2.1 Level AA:** 16-24 hours

Most issues are straightforward HTML/CSS/JS fixes that don't require major architectural changes.

---

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WebAIM](https://webaim.org/)
- [A11y Project Checklist](https://www.a11yproject.com/checklist/)
- [MDN Accessibility](https://developer.mozilla.org/en-US/docs/Web/Accessibility)
