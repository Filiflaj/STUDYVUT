import katex from "katex";
import { Marked, type Tokens, type TokenizerAndRendererExtension } from "marked";
import { contentUrl } from "./api";

/**
 * Markdown of chapters, quiz texts and the manual, as described in
 * docs/content.md: GFM, KaTeX math (`$…$`, `$$…$$`), callouts
 * (`> [!key] Title`), images relative to the subject folder (SVGs inlined so
 * they follow the theme), heading ids (`## Title {#id}`).
 */
export interface RenderContext {
  /** The subject folder that image paths are relative to. */
  subject?: string;
  /** SVG files by path (from the backend), to inline. */
  svgs?: Record<string, string>;
}

export interface Section {
  id: string;
  title: string;
  level: number;
}

export interface Rendered {
  html: string;
  sections: Section[];
}

/** Callout kinds and the title they get when the content gives none (the content is Czech). */
const CALLOUTS: Record<string, string> = { key: "Zapamatuj si", trap: "Pozor", example: "Příklad", note: "Poznámka" };

const escape = (s: string) =>
  s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c] ?? c);

function tex(text: string, display: boolean): string {
  return katex.renderToString(text, { displayMode: display, throwOnError: false, strict: "ignore" });
}

const mathBlock: TokenizerAndRendererExtension = {
  name: "mathBlock",
  level: "block",
  start(src: string) {
    return /(^|\n)\$\$/.exec(src)?.index;
  },
  tokenizer(src: string) {
    const m = /^\$\$([\s\S]+?)\$\$[ \t]*(?:\n+|$)/.exec(src);
    if (m) return { type: "mathBlock", raw: m[0], text: m[1].trim() };
    return undefined;
  },
  renderer(token) {
    return tex(String(token.text), true) + "\n";
  },
};

const mathInline: TokenizerAndRendererExtension = {
  name: "mathInline",
  level: "inline",
  start(src: string) {
    const i = src.indexOf("$");
    return i < 0 ? undefined : i;
  },
  tokenizer(src: string) {
    const display = /^\$\$([\s\S]+?)\$\$/.exec(src);
    if (display) return { type: "mathInline", raw: display[0], text: display[1].trim(), display: true };
    const inline = /^\$((?:\\.|[^\\$\n])+?)\$/.exec(src);
    if (inline) return { type: "mathInline", raw: inline[0], text: inline[1].trim(), display: false };
    return undefined;
  },
  renderer(token) {
    return tex(String(token.text), Boolean(token.display));
  },
};

/** An SVG of the content, ready to inline: no prolog, scaled with the column, labelled. */
export function cleanSvg(svg: string, alt: string): string {
  let s = svg
    .replace(/<\?xml[\s\S]*?\?>/g, "")
    .replace(/<!DOCTYPE[\s\S]*?>/gi, "")
    .replace(/<!--[\s\S]*?-->/g, "")
    .trim();
  if (!/^<svg\b[^>]*\bclass="[^"]*\bfig\b/.test(s)) s = s.replace(/^<svg\b/, '<svg class="fig"');
  if (alt && !/^<svg\b[^>]*\baria-label=/.test(s)) s = s.replace(/^<svg\b/, `<svg role="img" aria-label="${escape(alt)}"`);
  return s;
}

/** Heading ids from their text: lower case, no diacritics, dashes. */
export function slug(text: string): string {
  return (
    text
      .normalize("NFD")
      .replace(/[̀-ͯ]/g, "")
      .toLowerCase()
      .replace(/<[^>]+>/g, "")
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "") || "sekce"
  );
}

/** `> [!kind] Title` block quotes become callout boxes (Markdown inside stays Markdown). */
export function callouts(md: string, inline: (s: string) => string): string {
  const lines = md.split("\n");
  const out: string[] = [];
  for (let i = 0; i < lines.length; i++) {
    const m = /^>\s*\[!(\w+)\]\s*(.*)$/.exec(lines[i]);
    const kind = m?.[1].toLowerCase();
    if (!m || !kind || !(kind in CALLOUTS)) {
      out.push(lines[i]);
      continue;
    }
    const body: string[] = [];
    while (i + 1 < lines.length && /^>/.test(lines[i + 1])) body.push(lines[++i].replace(/^>\s?/, ""));
    const title = m[2].trim() || CALLOUTS[kind];
    out.push(
      "",
      `<div class="callout ${kind}">`,
      ...(title ? [`<p class="callout-title">${inline(title)}</p>`] : []),
      "",
      ...body,
      "",
      "</div>",
      "",
    );
  }
  return out.join("\n");
}

function makeMarked(ctx: RenderContext, sections: Section[] | null): Marked {
  const used = new Set<string>();
  const marked = new Marked({ gfm: true });
  // Captions may hold math and emphasis too.
  const inline = (s: string) => marked.parseInline(s, { async: false });
  marked.use({
    extensions: [mathBlock, mathInline],
    renderer: {
      image({ href, title, text }: Tokens.Image) {
        const svg = ctx.svgs?.[href];
        const caption = title ? `<figcaption>${inline(title)}</figcaption>` : "";
        if (svg !== undefined) return `<figure class="fig-box">${cleanSvg(svg, text || title || "")}${caption}</figure>`;
        if (/^[a-z]+:/i.test(href) || !ctx.subject) return "";
        const src = contentUrl(`${ctx.subject}/${href}`);
        return `<figure><img src="${escape(src)}" alt="${escape(text)}" />${caption}</figure>`;
      },
      heading({ tokens, depth }: Tokens.Heading) {
        let html = this.parser.parseInline(tokens);
        const explicit = /\s*\{#([\w-]+)\}\s*$/.exec(html);
        if (explicit) html = html.slice(0, explicit.index);
        let id = explicit?.[1] ?? slug(html);
        for (let n = 2; used.has(id); n++) id = `${explicit?.[1] ?? slug(html)}-${n}`;
        used.add(id);
        if (sections && depth >= 2 && depth <= 3) {
          sections.push({ id, title: html.replace(/<[^>]+>/g, "").replace(/&amp;/g, "&"), level: depth });
        }
        return `<h${depth} id="${id}">${html}</h${depth}>\n`;
      },
      link({ href, title, tokens }: Tokens.Link) {
        const text = this.parser.parseInline(tokens);
        const t = title ? ` title="${escape(title)}"` : "";
        return `<a href="${escape(href)}"${t}>${text}</a>`;
      },
    },
  });
  return marked;
}

/** Block Markdown (chapters, question texts, explanations). */
export function render(md: string, ctx: RenderContext = {}): Rendered {
  const sections: Section[] = [];
  const marked = makeMarked(ctx, sections);
  const inline = (s: string) => marked.parseInline(s, { async: false });
  const html = marked
    .parse(callouts(md, inline), { async: false })
    // A figure on its own line must not sit inside a paragraph.
    .replace(/<p>\s*(<figure[\s\S]*?<\/figure>)\s*<\/p>/g, "$1");
  return { html, sections };
}

/** Inline Markdown (options, notes): no paragraphs; figures stay inline. */
export function renderInline(md: string, ctx: RenderContext = {}): string {
  return makeMarked(ctx, null)
    .parseInline(md, { async: false })
    .replace(/<figure class="fig-box">([\s\S]*?)<\/figure>/g, '<span class="fig-inline">$1</span>');
}
