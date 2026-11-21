+++
title = "Funkce a hlavičkové soubory"
topics = ["Funkce"]
+++

Funkce je pojmenovaný kus kódu, který se dá zavolat opakovaně. Program se díky ní dělí na malé části, které jdou samostatně číst a testovat.

## Definice funkce

```c
int secti(int a, int b)   // návratový typ, jméno, parametry
{
    return a + b;         // return vrátí hodnotu a funkci hned ukončí
}
```

- **návratový typ** (`int`, `double`, …) říká, co funkce vrací; `void` znamená, že nevrací nic,
- **parametry** jsou lokální proměnné funkce, naplněné hodnotami z volání; `(void)` znamená „žádné parametry“,
- `return` funkci ukončí. Kód za ním se už neprovede.

> [!key] Předávání hodnotou
> Do funkce jde **kopie** hodnoty. Když funkce změní svůj parametr, proměnná volajícího se nezmění. Chce-li funkce změnit proměnnou volajícího, musí dostat její adresu (kapitola Ukazatele).

```c
void zvys(int x)
{
    x = x + 1;      // mění se jen kopie
}

int a = 5;
zvys(a);            // a je pořád 5
```

## Lokální a globální proměnné

Proměnná deklarovaná uvnitř funkce (nebo bloku `{ }`) existuje jen tam a při každém volání vzniká znovu. Globální proměnná (mimo všechny funkce) je vidět všude v souboru. Lokální proměnná se stejným jménem globální **zakryje**.

## Prototyp: funkce musí být známá dřív, než se zavolá

Překladač čte soubor shora dolů. Když funkci zavoláš dřív, než je definovaná, musí nad voláním být aspoň její **prototyp** (deklarace):

```c
void vypis_hlavicku(void);   // prototyp: hlavička + středník, bez těla
```

Bez prototypu dnešní překladače ohlásí *implicit declaration of function* (od C99 je to chyba).

## Hlavičkový soubor .h

Prototypy a konstanty, které sdílí víc souborů, patří do hlavičkového souboru. Příklad z úlohy s tabulkou mocnin:

```c
// tabulka.h
#ifndef TABULKA_H_INCLUDED
#define TABULKA_H_INCLUDED

#define NADPIS "Tabulka mocnin a odmocnin"
#define DELKA_ODDELOVACE 50
#define MAX_N 10

void vypis_hlavicku(void);
void vypis_oddelovac(void);
void vypis_tabulku(void);

#endif // TABULKA_H_INCLUDED
```

- `#ifndef … #define … #endif` je **include guard**: když se hlavička vloží dvakrát, podruhé se její obsah přeskočí a nevzniknou dvojí definice.
- `#include "tabulka.h"` (uvozovky) hledá soubor nejdřív ve složce se zdrojovým souborem, `#include <stdio.h>` (špičaté závorky) v systémových knihovnách.

Definice funkcí jsou v `tabulka.c`, `main.c` je jen volá:

```c
// main.c
#include <stdio.h>
#include "tabulka.h"

int main()
{
    vypis_hlavicku();
    vypis_tabulku();
    return 0;
}
```

```c
// tabulka.c (výřez)
void vypis_hlavicku(void)
{
    int i;
    int delka = strlen(NADPIS);   // 25 znaků

    printf("%s\n", NADPIS);
    for (i = 0; i < delka; i++)
    {
        putchar('=');             // podtržení přesně na délku nadpisu
    }
    putchar('\n');
}
```

Program vypíše nadpis, 25 rovnítek, záhlaví a řádky `n`, `n^2`, `n^3`, `sqrt(n)` pro n = 1 až 10:

```
   n    n^2    n^3    sqrt(n)
--------------------------------------------------
   1      1      1      1.000
   2      4      8      1.414
```

Formát `%4.0f` znamená: šířka 4 znaky, 0 desetinných míst; `%10.3f` šířka 10, tři desetinná místa.

## #define je náhrada textu

Preprocesor před překladem nahradí každý výskyt jména textem z `#define`. Nekontroluje typy a nepočítá.

> [!trap] Makro bez závorek
> `#define DVA 1 + 1` a pak `DVA * 3` se rozvine na `1 + 1 * 3`, tedy **4**, ne 6. Výraz v makru proto patří do závorek: `#define DVA (1 + 1)`.

## Překlad programu z více souborů

Všechny `.c` soubory se musí přeložit a slinkovat dohromady. Hlavičky se do příkazu nepíšou, vkládá je `#include`:

```
gcc main.c tabulka.c -o tabulka
```

Na Linuxu se k funkcím z `math.h` (`pow`, `sqrt`) přidává ještě `-lm`. Když `tabulka.c` v příkazu chybí, překlad projde, ale linker ohlásí *undefined reference* (na Macu *undefined symbols*) k funkcím, jejichž těla nenašel.

> [!trap] Časté chyby
> - Zapomenutý prototyp nebo `#include` vlastní hlavičky → *implicit declaration*.
> - Zapomenutý `.c` soubor v příkazu `gcc` → chyba linkeru, ne překladače.
> - Středník za hlavičkou definice: `void f(void); { … }` je prototyp a za ním blok mimo funkci, což je chyba překladu.
> - Očekávání, že funkce změní předaný parametr. Mění se jen kopie.

> [!note] Na co se můžou zeptat
> - **Co je prototyp a proč je v .h?** Deklarace funkce (návratový typ, jméno, typy parametrů). Díky ní může funkci volat jakýkoli soubor, který hlavičku vloží.
> - **K čemu je include guard?** Zabrání tomu, aby se obsah hlavičky při vícenásobném `#include` vložil dvakrát.
> - **Jaký je rozdíl mezi `#define MAX_N 10` a `const int`?** `#define` je jen textová náhrada preprocesoru bez typu, `const` je proměnná s typem, kterou kontroluje překladač.
> - **Proč `strlen(NADPIS)` a ne pevné číslo?** Když se nadpis změní, podtržení se samo přizpůsobí.
