+++
title = "Spojový seznam"
topics = ["Spojový seznam"]
+++

Spojový seznam je řetěz uzlů. Každý uzel nese hodnotu a adresu **dalšího** uzlu, poslední uzel má místo adresy `NULL`. Na začátek ukazuje ukazatel `head` (hlava).

```c
struct Node
{
    int data;            // hodnota
    struct Node *next;   // adresa dalšího uzlu (u posledního NULL)
};
```

```
head -> [10] -> [20] -> [30] -> NULL
```

Na rozdíl od pole nemusí uzly ležet v paměti za sebou. Vložení doprostřed znamená přepojit dva ukazatele, nic se neposouvá. K n-tému prvku se ale nedá skočit indexem, musí se dojít od hlavy.

## Průchod seznamem

```c
void vypis(struct Node *head)
{
    struct Node *current = head;
    while (current != NULL)
    {
        printf("%d -> ", current->data);
        current = current->next;     // posun na další uzel
    }
    printf("NULL\n");
}
```

Stejný cyklus sčítá hodnoty (`suma = suma + current->data`). Prázdný seznam (`head == NULL`) cyklus přeskočí, `vypis(NULL)` vypíše jen `NULL`.

## 9. týden: uzly jako obyčejné proměnné

```c
struct Node a, b, c;
a.data = 10;
b.data = 20;
c.data = 30;
a.next = &b;
b.next = &c;
c.next = NULL;

vypis(&a);                   // 10 -> 20 -> 30 -> NULL
```

### Vložení za uzel

```c
void vloz_za(struct Node *kam, struct Node *novy)
{
    novy->next = kam->next;  // 1. nový ukazuje tam, kam dosud ukazoval kam
    kam->next = novy;        // 2. kam ukazuje na nový
}

struct Node d;
d.data = 15;
d.next = NULL;
vloz_za(&a, &d);             // 10 -> 15 -> 20 -> 30 -> NULL, součet 75
```

> [!key] Pořadí přepojení
> Nejdřív se nový uzel napojí na zbytek seznamu, **potom** se na něj přesměruje předchozí uzel. V opačném pořadí se adresa zbytku seznamu přepíše dřív, než se uloží: `novy->next` pak ukazuje sám na sebe a zbytek seznamu se ztratí.

Takové uzly zanikají samy na konci `main`, `free` se na ně **nesmí** volat (nebyly přidělené přes `malloc`).

## 10. týden: uzly z malloc

```c
struct Node *vytvor_uzel(int data)
{
    struct Node *novy = malloc(sizeof(*novy));
    if (novy != NULL)
    {
        novy->data = data;
        novy->next = NULL;
    }
    return novy;
}
```

### Vložení na konec

```c
struct Node *vloz_na_konec(struct Node *head, int data)
{
    struct Node *novy = vytvor_uzel(data);
    if (novy == NULL)
    {
        return head;
    }

    if (head == NULL)            // prázdný seznam: nový uzel je hlava
    {
        return novy;
    }

    struct Node *current = head;
    while (current->next != NULL) // zastaví se na POSLEDNÍM uzlu
    {
        current = current->next;
    }
    current->next = novy;

    return head;
}

head = vloz_na_konec(head, 10);   // výsledek se musí uložit
```

> [!trap] `while (current != NULL)` při hledání konce
> Takový cyklus skončí až za posledním uzlem, `current` je `NULL` a `current->next = novy` program shodí. Pro připojení na konec se hledá uzel, jehož `next` je `NULL`.

### Smazání prvního uzlu

```c
struct Node *smaz_prvni(struct Node *head)
{
    if (head == NULL)
    {
        return NULL;
    }

    struct Node *nova_hlava = head->next;   // nejdřív uložit druhý uzel
    free(head);                             // pak uvolnit první
    return nova_hlava;
}
```

### Uvolnění celého seznamu

```c
void uvolni(struct Node *head)
{
    while (head != NULL)
    {
        struct Node *dalsi = head->next;    // adresu dalšího si uložit
        free(head);
        head = dalsi;
    }
}
```

Každý uzel má svůj `malloc`, a proto potřebuje svůj `free`. Seznam se 4 uzly = 4 volání `free`.

> [!key] Proč funkce vracejí hlavu
> Funkce dostane **kopii** ukazatele `head`. Když se hlava změní (vložení do prázdného seznamu, smazání prvního uzlu), změní se jen kopie. Nová hlava se proto vrací a ukládá: `head = smaz_prvni(head);`.

Vložení **na začátek** nemusí nic procházet:

```c
novy->next = head;
head = novy;
```

Vložení na konec musí dojít až k poslednímu uzlu. U seznamu se 4 uzly udělá 3 kroky `current = current->next`.

> [!trap] Časté chyby
> - `free(head); head = head->next;` čte uvolněný uzel. Správně nejdřív uložit `head->next`.
> - Zapomenuté `head = …` u volání funkce, která hlavu mění.
> - Opačné pořadí přepojení při vkládání.
> - `free` na uzel, který nevznikl přes `malloc`.

> [!note] Na co se můžou zeptat
> - **Proč funkce vracejí hlavu?** Dostávají jen kopii ukazatele. Novou hlavu musí vrátit a `main` ji uloží.
> - **Proč v `smaz_prvni` nejdřív uložit `next` a pak `free`?** Po `free` se k uzlu už nesmí přistupovat.
> - **Proč u pole stačí jedno free a u seznamu ne?** Pole je jeden blok z jednoho malloc, každý uzel seznamu je samostatný blok.
> - **Rozdíl mezi 9. a 10. týdnem?** Uzly jako proměnné v `main` zaniknou samy a je jich pevný počet. Uzly z malloc vznikají za běhu v libovolném počtu, ale musí se uvolnit.
> - **Jak se pozná konec seznamu?** Poslední uzel má `next == NULL`.
