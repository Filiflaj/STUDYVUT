/** Colour scheme: user preference (system/light/dark) and the effective theme. */

import type { ThemePref } from "./types";

export type { ThemePref };

const query = typeof window === "undefined" ? null : window.matchMedia("(prefers-color-scheme: dark)");

class ThemeState {
  pref = $state<ThemePref>("system");
  systemDark = $state(query?.matches ?? false);

  constructor() {
    query?.addEventListener("change", (e) => (this.systemDark = e.matches));
  }

  get effective(): "light" | "dark" {
    if (this.pref === "system") return this.systemDark ? "dark" : "light";
    return this.pref;
  }

  /** Apply synchronously so CSS variables are current before dependents re-render. */
  apply(pref: ThemePref) {
    this.pref = pref;
    const root = document.documentElement;
    if (pref === "system") delete root.dataset.theme;
    else root.dataset.theme = pref;
  }
}

export const theme = new ThemeState();
