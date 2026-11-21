+++
title = "Dynamická paměť"
topics = ["Dynamická paměť"]
+++

Obyčejné pole má velikost pevně danou při překladu a lokální proměnné zanikají s koncem funkce. Dynamická paměť se přiděluje **za běhu** v potřebné velikosti a žije, dokud ji program sám neuvolní. Funkce jsou ve `stdlib.h`.

| Funkce | Co dělá |
|---|---|
| `malloc(bajty)` | přidělí blok, obsah **není nastavený** („smetí“) |
| `calloc(pocet, velikost)` | přidělí blok pro `pocet` prvků a **vynuluje** ho |
| `realloc(p, nove_bajty)` | změní velikost bloku, původní obsah zachová, nová část nenastavená |
| `free(p)` | blok vrátí systému |

Všechny přidělovací funkce vracejí ukazatel na blok, nebo `NULL`, když se přidělení nepovede.

## malloc a sizeof(*p)

```c
int *vytvor(int n)
{
    int *p = malloc(n * sizeof(*p));   // místo pro n čísel int
    if (p == NULL)
    {
        return NULL;                   // alokace se nepovedla
    }

    for (int i = 0; i < n; i++)
    {
        p[i] = (i + 1) * (i + 1);      // 1, 4, 9, 16, 25
    }
    return p;
}
```

> [!key] Velikost bloku: `pocet * sizeof(*p)`
> `*p` je jeden prvek (`int` = 4 B), takže pro 5 prvků 20 B. `sizeof(p)` je velikost **adresy** (8 B na 64bitovém systému) a s daty nesouvisí. Zápis `sizeof(*p)` zůstane správný, i když se typ ukazatele později změní.

S přiděleným blokem se pracuje jako s polem: `p[i]` nebo `*(p + i)`. Počet prvků si program musí pamatovat sám (proměnná `n`), `sizeof` ho z ukazatele nezjistí.

Funkce smí vrátit ukazatel na blok z `malloc`, protože blok funkci přežije. Vrátit adresu **lokálního** pole nesmí: pole s koncem funkce zanikne a ukazatel by mířil do neplatné paměti.

## realloc přes pomocný ukazatel

```c
int *zvetsi(int *p, int n, int novy_n)
{
    int *pom = realloc(p, novy_n * sizeof(*pom));
    if (pom == NULL)
    {
        return NULL;          // původní blok p zůstává platný
    }

    for (int i = n; i < novy_n; i++)
    {
        pom[i] = 0;           // nové prvky realloc nenuluje
    }
    return pom;
}
```

A v `main`:

```c
int *pom = zvetsi(pole, n, 8);
if (pom == NULL)
{
    printf("Zvetseni se nezdarilo\n");
    free(pole);               // původní blok je pořád náš, uvolníme ho
    return 1;
}
pole = pom;                   // teprve teď přepíšeme původní ukazatel
n = 8;
```

Výstup úlohy: `Pole: 1 4 9 16 25` a `Po zvetseni: 1 4 9 16 25 0 0 0`. `realloc` může blok přesunout jinam, staré prvky zkopíruje sám.

> [!trap] `pole = realloc(pole, …)`
> Když realloc selže, vrátí `NULL`, ale původní blok zůstane přidělený. Přiřazení rovnou do `pole` přepíše jedinou adresu bloku na `NULL` a blok už nejde uvolnit: **únik paměti**. Proto výsledek nejdřív do `pom`, kontrola, a až pak `pole = pom`.

## free a co po něm

```c
free(pole);     // vrátí blok systému
pole = NULL;    // ukazatel už nikam neukazuje
```

- Každý `malloc`/`calloc` potřebuje právě jeden `free`. Pole z jednoho `malloc` je jeden blok, stačí jedno `free`.
- **Únik paměti** (memory leak): blok se neuvolní a ztratí se poslední ukazatel na něj.
- **Visící ukazatel** (dangling pointer): ukazatel na už uvolněný blok. Čtení nebo zápis přes něj je nedefinované chování.
- Dvojí `free` stejného bloku je chyba. `free(NULL)` je povolené a nic neudělá. Proto se po `free` píše `pole = NULL`.

Kontrola úniků na Macu: `leaks --atExit -- ./program` (na Linuxu `valgrind ./program`).

> [!trap] Časté chyby
> - `malloc(n * sizeof(p))` místo `sizeof(*p)`.
> - Chybí test na `NULL` po `malloc`/`realloc`.
> - Čtení nenastavených prvků po `malloc` nebo `realloc`.
> - Použití ukazatele po `free`.

> [!note] Na co se můžou zeptat
> - **Proč pomocný ukazatel u realloc?** Při neúspěchu vrátí `NULL` a původní blok zůstane. Kdyby se přepsal jediný ukazatel, blok by se nedal uvolnit.
> - **Proč se nové prvky nulují?** malloc ani realloc paměť nenastavují. Nuly dává jen calloc.
> - **Proč `sizeof(*p)`, ne `sizeof(p)`?** `*p` je jeden prvek (int, 4 B), `p` je adresa (8 B).
> - **Proč si držet n?** Velikost přiděleného bloku z ukazatele zjistit nejde.
> - **Proč `pole = NULL` po free?** Aby se omylem nepoužila uvolněná paměť a aby případné další `free(NULL)` nic neudělalo.
