<p align="center">
  <img src="src-tauri/icons/icon.png" alt="" height="72">
</p>

<h1 align="center">STUDYVUT</h1>

<p align="center">
  Učivo a kvízy ze všech předmětů na jednom místě.<br>
  Desktopová aplikace pro macOS, Windows a Linux.
</p>

<p align="center">
  <a href="https://github.com/Filiflaj/STUDYVUT/releases/latest"><b>Stáhnout</b></a> ·
  <a href="CHANGELOG.md">Co je nového</a> ·
  <a href="docs/manual/cs.md">Návod</a> ·
  <a href="docs/content.md">Formát obsahu</a>
</p>

## Co umí

- **Učivo** každého předmětu: kapitoly se vzorci (KaTeX), obrázky, které se
  přebarví podle světlého a tmavého vzhledu, a boxy „zapamatuj si“, „pozor“
  a „příklad“.
- **Kvízy**, jak je znáš z trenažéru BFY1: jedna otázka po druhé, hned vidíš,
  co bylo správně, proč jsou ostatní možnosti špatně a jak se to počítá.
  Číselné otázky s tolerancí. Na konci výsledek a zopakování chybných.
- **Opakování s rozestupy**: co zkazíš, vrátí se zítra; co umíš, za 2, 4, 8
  a 16 dní. Sada **K opakování** a **Moje chyby** u každého předmětu.
- **Nástěnka** s výběrem oboru a ročníku: vidíš jen své předměty a co dnes
  čeká na opakování.
- Česky a anglicky, světlý i tmavý vzhled. Pokrok zůstává jen u tebe
  v počítači (SQLite) a jde exportovat.

## Předměty

1. semestr MET (FEKT VUT), ZS 2026/2027: BPC-EL1, BPC-ELS, BPC-FY1, BPC-FYS,
BPC-MA1, BPC-MAS, BPC-MPE, BPC-PC1M, BPC-TDE. Co který obsahuje, ukáže
aplikace na nástěnce; přehled změn je v [CHANGELOG.md](CHANGELOG.md).

## Instalace

Stáhni soubor pro svůj systém z [Releases](https://github.com/Filiflaj/STUDYVUT/releases/latest).
Aplikace není podepsaná certifikátem vývojáře, proto ji systém napoprvé
nechce spustit:

- **macOS** (`.dmg`): přetáhni do Aplikací, pak na ni **klikni pravým
  tlačítkem → Otevřít → Otevřít**. Stačí jednou.
- **Windows** (`-setup.exe`): SmartScreen ukáže varování, klikni na **Další
  informace → Přesto spustit**.
- **Linux** (`.AppImage`): `chmod +x studyvut-*.AppImage` a spusť.

## Pro vývoj

Rust (stable) a Node 22.12+:

```sh
npm ci
npm run tauri dev                       # aplikace, obsah čte přímo z content/
npm run dev                             # náhled v prohlížeči: http://localhost:1420/preview.html
cargo run -p studyvut-cli -- check      # kontrola obsahu
```

Architektura, příkazy a pravidla jsou v [AGENTS.md](AGENTS.md), verze a vydání
v [docs/versioning.md](docs/versioning.md). Nový obsah: originály do
`inbox/<předmět>/` (git je ignoruje), zpracovaný obsah do `content/<předmět>/`
podle [docs/content.md](docs/content.md).

Licence: [MIT](LICENSE), cizí části v [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).
