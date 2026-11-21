+++
title = "Struktury, enum a union"
topics = ["Struktury"]
+++

Struktura (`struct`) spojí několik proměnných různých typů do jednoho celku. Kniha má název, počet stran a cenu. Místo tří oddělených polí stačí jedno pole knih.

## struct a typedef

```c
typedef struct
{
    char nazev[40];
    int pocetStran;
    float cena;
} Kniha;              // díky typedef stačí psát Kniha, ne struct …
```

Bez `typedef` se musí při každé deklaraci psát celé `struct Jmeno`:

```c
struct Obdelnik { int a; int b; };
struct Obdelnik o = {3, 5};    // správně
Obdelnik o2;                   // chyba: bez typedef jméno Obdelnik samo nestačí
```

K položkám se přistupuje tečkou: `knihy[i].cena`, `knihy[2].nazev[0]` (první znak názvu třetí knihy).

## Pole struktur: úloha s knihami

```c
Kniha knihy[4] =
{
    {"C", 240, 349.0f},
    {"Algoritmy", 520, 699.0f},
    {"Git", 180, 299.0f},
    {"Linux", 410, 549.0f}
};
int nejdrazsi = 0;               // index zatím nejdražší knihy

knihy[1].cena = knihy[1].cena + 50;   // Algoritmy: 749 Kč

for (int i = 1; i < 4; i++)
{
    if (knihy[i].cena > knihy[nejdrazsi].cena)
    {
        nejdrazsi = i;
    }
}
printf("Nejdrazsi: %s (%.2f Kc)\n", knihy[nejdrazsi].nazev, knihy[nejdrazsi].cena);
```

Výstup posledního řádku: `Nejdrazsi: Algoritmy (749.00 Kc)`. Pamatuje se **index**, ne cena, takže se pak dá vypsat celý záznam.

> [!key] Přiřazení struktury kopíruje
> `Bod b = a;` zkopíruje všechny položky. Změna `b.x` pak `a` neovlivní. Stejně tak struktura předaná do funkce hodnotou je kopie, proto se ve funkcích předává ukazatel (`&obdelnik`).

> [!trap] Řetězec do pole nejde přiřadit
> `k.nazev = "C";` je chyba překladu (pole není přiřaditelné). Řetězec jde zapsat jen při inicializaci (`{"C", 240, 349.0f}`) nebo funkcí `strcpy(k.nazev, "C")`.

## enum: pojmenované celočíselné konstanty

```c
typedef enum
{
    VYPNUTO,      // 0
    ZAPNUTO,      // 1
    CHYBA         // 2
} Stav;
```

Hodnoty se číslují od 0 po jedné, pokud se nenapíše jinak. `typedef enum { CELE = 1, DESETINNE } TypHodnoty;` dá `DESETINNE = 2`: číslování pokračuje od poslední zadané hodnoty.

Enum se hodí do `switch`. Úloha se zařízeními vypíše stav slovy a spočítá chyby:

```c
switch (zarizeni[i].stav)
{
    case VYPNUTO:
        printf("vypnuto\n");
        break;
    case ZAPNUTO:
        printf("zapnuto\n");
        break;
    case CHYBA:
        printf("chyba\n");
        break;
}
if (zarizeni[i].stav == CHYBA)
{
    pocetChyb++;
}
```

> [!trap] Zapomenutý break
> Bez `break` program po nalezené větvi **propadne** do dalších a provede i je. Pro `ZAPNUTO` by se vypsalo `zapnuto chyba`.

## Vyhledávání v poli struktur

Úloha s osobami prochází pole a porovnává `osoby[i].id` se zadaným ID. Jestli se našlo, si pamatuje příznak `nalezeno`. Hláška `Nenalezeno` se vypíše až **po** cyklu, když příznak zůstal 0. Kdyby byla ve větvi `else` uvnitř cyklu, vypsala by se u každé osoby, která nesedí.

## union: více typů na stejném místě v paměti

```c
typedef union
{
    int cele;
    float desetinne;
} Hodnota;            // sizeof(Hodnota) = 4: obě položky sdílejí stejné 4 bajty

typedef struct
{
    TypHodnoty typ;   // co v unionu právě je
    Hodnota hodnota;
} Data;
```

Union je velký jako jeho **největší** položka a v jednu chvíli smysluplně drží jen tu, která se zapsala naposledy. Proto se k němu přidává položka `typ`, podle které program ví, kterou část číst:

```c
if (data.typ == CELE)
{
    printf("Ulozeno cele: %d\n", data.hodnota.cele);
}
else if (data.typ == DESETINNE)
{
    printf("Ulozeno desetinne: %.2f\n", data.hodnota.desetinne);
}
```

Struktura se stejnými dvěma položkami by měla 8 bajtů, protože každá položka má svoje místo.

> [!note] Na co se můžou zeptat
> - **Rozdíl mezi struct a union?** Ve struktuře má každá položka vlastní paměť a platí všechny najednou. V unionu sdílí položky jedno místo, platí jen ta naposledy zapsaná.
> - **K čemu je typedef?** Dá typu kratší jméno, místo `struct Kniha` stačí `Kniha`.
> - **Proč se hledá index nejdražší knihy, ne jen cena?** Z indexu se dá vypsat název i cena.
> - **Proč je `Nenalezeno` až za cyklem?** Teprve po projití všech osob je jisté, že hledané ID v poli není.
> - **Jaké hodnoty má enum?** Celá čísla od 0, pokud se nepřiřadí jinak.
