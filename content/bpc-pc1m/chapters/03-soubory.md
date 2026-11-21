+++
title = "Práce se soubory"
topics = ["Soubory"]
+++

S textovým souborem se pracuje ve třech krocích: **otevřít** (`fopen`), **číst nebo zapisovat**, **zavřít** (`fclose`). Mezi tím se soubor drží v proměnné typu `FILE *`.

## fopen a režimy

```c
FILE *soubor = fopen("cisla.txt", "r");

if (soubor == NULL)
{
    printf("Chyba! Soubor cisla.txt nelze otevrit\n");
    return 1;
}
```

| Režim | Co dělá | Když soubor neexistuje |
|---|---|---|
| `"r"` | čtení od začátku | vrátí `NULL` |
| `"w"` | zápis, **starý obsah smaže** | vytvoří nový |
| `"a"` | zápis na konec (append) | vytvoří nový |
| `"r+"` | čtení i zápis | vrátí `NULL` |

> [!key] Vždy kontrolovat NULL
> Když se soubor otevřít nepovede (neexistuje, chybí práva), vrátí `fopen` hodnotu `NULL`. Čtení z `NULL` program shodí, proto hned za `fopen` patří test a ukončení programu.

**Cesta k souboru:** jméno bez cesty (`"cisla.txt"`) se hledá ve složce, **ze které se program spouští** (pracovní adresář), ne tam, kde leží `.c` soubor. Proto se před spuštěním přepíná `cd zadani1_cisla`.

## Čtení čísel: fscanf

```c
while (fscanf(soubor, "%d", &cislo) == 1)
{
    pocet++;
    soucet = soucet + cislo;
    printf("%d. cislo: %d\n", pocet, cislo);
}
fclose(soubor);
```

`fscanf` vrací, **kolik položek načetl**. Podmínka `== 1` skončí na konci souboru (vrátí `EOF`) i na prvním slově, které číslem není (vrátí 0). U souboru `10 25 16 abc 7` se načtou jen 10, 25 a 16.

> [!trap] `!= EOF` místo `== 1`
> S podmínkou `!= EOF` se na nečíselném slově `fscanf` zasekne: vrací pořád 0, nic nepřečte a cyklus běží donekonečna.

Průměr se počítá jako `(float)soucet / pocet`. Přetypuje se **před** dělením, jinak by se dělilo celočíselně: `(float)(7 / 2)` je 3.00, `(float)7 / 2` je 3.50. Prázdný soubor (`pocet == 0`) se ošetří zvlášť, aby se nedělilo nulou.

### Hledání maxima

```c
while (fscanf(soubor, "%d", &cislo) == 1)
{
    if (pocet == 0 || cislo > nejvetsi)   // první číslo bereme jako dosavadní maximum
    {
        nejvetsi = cislo;
    }
    pocet++;
}
```

> [!trap] Maximum začínající nulou
> S `int nejvetsi = 0;` vyjde u souboru se samými zápornými čísly (−15, −3, −42, −8) maximum **0**, které v souboru vůbec není. Správně se začíná prvním načteným číslem.

## Kopírování po znacích: fgetc a fputc

```c
int znak;                                   // int, ne char!
while ((znak = fgetc(vstup)) != EOF)
{
    fputc(znak, kopie);
}
```

`fgetc` vrací přečtený znak jako `int`, nebo `EOF` (záporná konstanta, obvykle −1) na konci souboru. Znak se tak čte i s mezerami, konci řádků a interpunkcí, kopie je přesná.

> [!trap] `char znak`
> Do `char` se `EOF` nedá spolehlivě uložit vedle všech 256 možných bajtů. Podle platformy pak cyklus skončí předčasně na bajtu 255, nebo neskončí vůbec. Proměnná pro `fgetc` musí být `int`.

## Řádky: fputs a fgets

```c
fputs(jmeno, soubor);        // zapíše řetězec, sám konec řádku nepřidá
fputs("\n", soubor);
...
fclose(soubor);              // zapíše vyrovnávací paměť na disk
soubor = fopen("jmena.txt", "r");
while (fgets(radek, sizeof(radek), soubor) != NULL)
{
    poradi++;
    printf("%d. %s", poradi, radek);   // \n už je v radek
}
```

- `fgets` přečte jeden řádek **včetně** `\n` (nejvýš `sizeof(radek) - 1` znaků) a na konci souboru vrátí `NULL`. Proto `printf` nemá vlastní `\n`.
- `fputs` konec řádku nepřidává, musí se zapsat zvlášť.
- `scanf("%49s", jmeno)` čte jen jedno slovo (do mezery) a nejvýš 49 znaků, aby se nepřeteklo pole `char jmeno[50]`.

## fclose

`fclose` zapíše data, která čekají ve vyrovnávací paměti, a soubor uvolní. V úloze se jmény se soubor po zápisu **zavře a znovu otevře** pro čtení. Bez zavření by data mohla být ještě ve vyrovnávací paměti a čtení by je nevidělo.

> [!note] Na co se můžou zeptat
> - **Co vrátí fopen, když soubor neexistuje?** `NULL`, proto se hned testuje.
> - **Jaký je rozdíl mezi „w“ a „a“?** „w“ starý obsah smaže, „a“ připisuje na konec.
> - **Proč `int znak` u fgetc?** `EOF` musí jít odlišit od každého skutečného znaku.
> - **Proč `fscanf(...) == 1`?** fscanf vrací počet načtených položek. Cyklus skončí na konci souboru i na nečíselném textu.
> - **Proč program soubor nenašel?** Byl spuštěn z jiné složky. Relativní jméno se hledá v pracovním adresáři.
> - **Proč maximum nezačíná nulou?** Se samými zápornými čísly by vyšla nula.
