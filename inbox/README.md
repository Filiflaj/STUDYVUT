# Inbox: podklady ke zpracování

Sem patří **originály** studijních materiálů: PDF ze skript a přednášek,
prezentace, fotky tabule a starých testů, zadání, vlastní poznámky, protokoly.
Claude z nich udělá obsah aplikace v `content/<předmět>/` (učivo a kvízy podle
[docs/content.md](../docs/content.md)).

**Nic z této složky se necommituje** (kromě tohoto README a prázdných
`.gitkeep`), protože repozitář je veřejný a originály mají svá autorská práva.
Soubory tu zůstávají jen na tvém disku.

| Složka | Předmět |
|---|---|
| `bpc-els/` | Elektrotechnický seminář (BPC-ELS) |
| `bpc-el1/` | Elektrotechnika 1 (BPC-EL1) |
| `bpc-fy1/` | Fyzika 1 (BPC-FY1), přednášky i laboratoře |
| `bpc-fys/` | Fyzikální seminář (BPC-FYS) |
| `bpc-mas/` | Matematický seminář (BPC-MAS) |
| `bpc-ma1/` | Matematika 1 (BPC-MA1) |
| `bpc-mpe/` | Materiály pro elektrotechniku (BPC-MPE) |
| `bpc-pc1m/` | Počítače a programování 1 (BPC-PC1M) |
| `bpc-tde/` | Technická dokumentace (BPC-TDE) |

Uvnitř předmětu si soubory klidně rozděl do podsložek (např. `prednasky/`,
`cviceni/`, `testy/`). Když něco přidáš, napiš Claudovi, co to je a na co se
má zaměřit (např. „tohle jsou okruhy ke zkoušce, udělej z nich kvíz“).

Nový předmět (další semestr): založ `inbox/<kod>/` a řekni Claudovi, ať
k němu přidá `content/<kod>/subject.toml`.
