+++
title = "Ukazatele"
topics = ["Ukazatele"]
+++

Každá proměnná leží v paměti na nějaké **adrese**. Ukazatel (pointer) je proměnná, ve které je uložená adresa jiné proměnné.

## & a *

```c
int a = 5;
int *p = &a;      // p ukazuje na a (obsahuje adresu a)
*p = 8;           // zápis přes ukazatel: a je teď 8
printf("%p\n", (void *)p);   // adresa, šestnáctkově
```

- `&a` … adresa proměnné `a` (operátor reference, znáš ho ze `scanf("%d", &x)`),
- `int *p` … v deklaraci: `p` je ukazatel na `int`,
- `*p` … ve výrazu: hodnota na adrese v `p` (dereference).

Typ ukazatele musí odpovídat typu proměnné. Podle něj překladač ví, kolik bajtů na adrese číst a o kolik se posunout při aritmetice.

> [!trap] Hvězdička patří k proměnné
> `int *p, q;` deklaruje ukazatel `p` a **obyčejný int** `q`. Dva ukazatele jsou `int *p, *q;`.

`NULL` je ukazatel, který nikam neukazuje. Používá se jako „zatím nic“ a jako značka konce spojového seznamu. Dereference `NULL` (nebo neinicializovaného ukazatele) program obvykle shodí.

Ukazatel na ukazatel (`int **pp = &p;`) drží adresu ukazatele. `**pp` je pak hodnota původní proměnné.

## Ukazatel jako parametr funkce

Funkce dostává kopie argumentů. Aby mohla změnit proměnnou volajícího, musí dostat její **adresu**:

```c
void prohod(int *x, int *y)
{
    int t = *x;
    *x = *y;
    *y = t;
}

prohod(&a, &b);   // vymění hodnoty a a b
```

Verze `void prohod(int x, int y)` by prohodila jen své kopie, `a` a `b` by zůstaly stejné.

Stejně se z funkce „vrací“ víc hodnot. Funkce `min_max` z úlohy dostane adresy dvou proměnných a výsledky zapíše přes ně:

```c
void min_max(int *p, int n, int *min, int *max)
{
    *min = *p;    // začínáme prvním prvkem (ne 0, kvůli záporným číslům)
    *max = *p;

    for (int i = 1; i < n; i++)
    {
        if (*(p + i) < *min)
        {
            *min = *(p + i);
        }
        if (*(p + i) > *max)
        {
            *max = *(p + i);
        }
    }
}

min_max(cisla, 6, &nejmensi, &nejvetsi);
```

## Ukazatele a pole

Jméno pole se ve výrazu chová jako adresa jeho prvního prvku. Proto se pole do funkce posílá **bez &**: `vypis(cisla, 6)` předá `&cisla[0]`. Funkce dostane jen adresu, ne délku, a proto se délka předává zvlášť (`n`).

```c
void zdvojnasob(int *p, int n)
{
    for (int i = 0; i < n; i++)
    {
        *(p + i) = *(p + i) * 2;    // totéž jako p[i] = p[i] * 2
    }
}
```

Funkce mění **původní pole**, protože pracuje přímo s jeho adresou. V úloze se proto min a max hledají ještě před zdvojnásobením. Po něm by vyšlo −10 a 18 místo −5 a 9.

> [!key] Aritmetika ukazatelů
> `p + i` neposouvá adresu o `i` bajtů, ale o `i` **prvků**, tedy o `i * sizeof(*p)` bajtů. Pro `int` (4 B) je `p + 3` o 12 bajtů dál. Platí `*(p + i) == p[i]`.

```c
int pole[5] = {10, 20, 30, 40, 50};
int *p = pole;
*(p + 2)     // 30: třetí prvek
*p + 2       // 12: první prvek plus 2 (* má přednost před +)
p++;         // p ukazuje na pole[1]
```

> [!trap] sizeof pole ve funkci
> V `main` dá `sizeof(cisla)` velikost celého pole (6 · 4 = 24 B). Ve funkci je parametr jen ukazatel, `sizeof(p)` je velikost adresy (8 B na 64bitovém systému). Počet prvků se uvnitř funkce zjistit nedá.

## Ukazatel na strukturu: šipka

```c
struct Obdelnik
{
    int a;
    int b;
};

int obsah(struct Obdelnik *o)
{
    return o->a * o->b;          // o->a je totéž co (*o).a
}

void prohod_strany(struct Obdelnik *o)
{
    int temp = o->a;
    o->a = o->b;
    o->b = temp;
}

struct Obdelnik obdelnik = {3, 5};
zvetsi(&obdelnik, 2);            // v main je struktura → tečka a &
printf("%d x %d\n", obdelnik.a, obdelnik.b);
```

- **tečka** pro strukturu (`obdelnik.a`), **šipka** pro ukazatel na strukturu (`o->a`),
- `*o.a` je chyba: tečka má přednost, takže se to čte jako `*(o.a)`. Správně `(*o).a` nebo `o->a`.

Výstup úlohy: `Obdelnik 3 x 5, obsah 15`, po zvětšení `6 x 10, obsah 60`, po prohození `10 x 6`.

> [!trap] Časté chyby
> - `scanf("%d", x)` bez `&`: scanf dostane hodnotu místo adresy a zapíše bůhvíkam.
> - `zvetsi(obdelnik, 2)` místo `zvetsi(&obdelnik, 2)`: funkce čeká ukazatel, dostala strukturu.
> - Šipka u obyčejné struktury nebo tečka u ukazatele.
> - Dereference neinicializovaného ukazatele nebo `NULL`.

> [!note] Na co se můžou zeptat
> - **Co je ukazatel?** Proměnná, která uchovává adresu jiné proměnné daného typu.
> - **Proč se pole posílá do funkce bez &?** Jméno pole se samo převede na adresu prvního prvku.
> - **Proč funkce min_max potřebuje ukazatele?** `return` vrátí jen jednu hodnotu. Přes dvě adresy zapíše funkce dvě.
> - **Kdy tečka a kdy šipka?** Tečka u struktury, šipka u ukazatele na strukturu (`o->a == (*o).a`).
> - **O kolik se posune `p + 1`?** O jeden prvek, tedy o `sizeof(*p)` bajtů.
