+++
title = "Zásobník a fronta"
topics = ["Zásobník a fronta"]
+++

Zásobník i fronta ukládají hodnoty a zase je vydávají. Liší se jen **pořadím**, ve kterém hodnoty vycházejí ven.

| | Zásobník (stack) | Fronta (queue) |
|---|---|---|
| princip | **LIFO**: poslední dovnitř, první ven | **FIFO**: první dovnitř, první ven |
| vložení | `push` na vrchol | `enqueue` na konec |
| výběr | `pop` z vrcholu | `dequeue` ze začátku |
| příklad | tlačítko Zpět, návrat robota | tisková fronta, příkazy robota |

> [!key] Pořadí
> Vložím 1, 2, 3. Zásobník vrací **3, 2, 1**, fronta **1, 2, 3**.

## Zásobník v poli

```c
#define MAX 50

int stack[MAX];
int top = -1;          // index vrcholu, -1 = prázdný

void push(int value)
{
    if (top == MAX - 1)          // plný
    {
        printf("Zasobnik je plny\n");
        return;
    }
    top++;
    stack[top] = value;
}

int pop(void)
{
    if (top == -1)               // prázdný
    {
        printf("Zasobnik je prazdny\n");
        return -1;
    }
    int value = stack[top];
    top--;
    return value;
}
```

`stackPeek` vrátí `stack[top]` bez odebrání, `stackIsEmpty` vrací výsledek porovnání `top == -1` (1 = prázdný).

## Kruhová fronta

Fronta si drží index začátku `front`, index posledního prvku `rear` a počet prvků `count`. Indexy se posouvají **modulo MAX**, takže po posledním políčku pole pokračují zase od nuly a uvolněná místa na začátku se znovu využijí.

```c
int queue[MAX];
int front = 0;
int rear = -1;
int count = 0;

void enqueue(int value)
{
    if (count == MAX)
    {
        printf("Fronta je plna\n");
        return;
    }
    rear = (rear + 1) % MAX;     // za posledním políčkem zpět na 0
    queue[rear] = value;
    count++;
}

int dequeue(void)
{
    if (count == 0)
    {
        printf("Fronta je prazdna\n");
        return -1;
    }
    int value = queue[front];
    front = (front + 1) % MAX;
    count--;
    return value;
}
```

Příklad pro `MAX = 5`: pět vložení dá `rear = 4`, dva výběry `front = 2`. Další vložení zapíše na `rear = (4 + 1) % 5 = 0`, tedy na uvolněné první políčko.

> [!trap] Návratová hodnota −1
> Při prázdném zásobníku nebo frontě vrací knihovna −1. To jde jen proto, že se ukládají znaky příkazů. Kdyby se ukládala libovolná čísla, −1 by nešlo odlišit od skutečné hodnoty. Před `pop`/`dequeue` je proto lepší testovat `stackIsEmpty()` / `queueIsEmpty()`, jak to dělá robot.

## Knihovna ve dvou souborech

- `stack.h`, `queue.h`: jen **prototypy** (rozhraní) a include guard,
- `stack.c`, `queue.c`: pole a indexy jako globální proměnné a těla funkcí,
- `main1.c`: `#include "stack.h"` a `#include "queue.h"`, pole přímo nevidí, používá jen funkce.

```
gcc main1.c stack.c queue.c -o program
```

`#define MAX 50` je v obou `.c` souborech a nevadí to: makro platí jen v souboru, kde je definované. Globální proměnné ale sdílí celý program, takže dvě inicializované proměnné se stejným jménem ve dvou `.c` souborech linker odmítne (*multiple definition*). Proto mají jména `top`, `front`, `rear`, `count` a navzájem se nekryjí. (Klíčové slovo `static` před globální proměnnou by ji skrylo jen do jejího souboru.)

## Robot: fronta i zásobník v jednom programu

Robot stojí na (0, 0) čelem na sever (N). Příkazy `F` (dopředu), `B` (dozadu), `L`, `R` (otočit vlevo, vpravo) se nejdřív ukládají do **fronty**, dokud nepřijde `U`. Neplatný znak program vypíše a zahodí.

```c
// 2) provádění příkazů v pořadí zadání, každý se uloží i do zásobníku
while (!queueIsEmpty())
{
    prikaz = dequeue();
    robot = provedPrikaz(robot, prikaz);
    push(prikaz);
    krok++;
    vypisStav(krok, prikaz, robot);
}

// 3) návrat: poslední příkaz se ruší první, a to opačným příkazem
while (!stackIsEmpty())
{
    prikaz = opacnyPrikaz(pop());    // F <-> B, L <-> R
    robot = provedPrikaz(robot, prikaz);
    ...
}
```

Funkce robota dostávají strukturu `Robot` **hodnotou** a upravenou kopii vracejí (`robot = posun(robot, 1);`). Souřadnice: sever `y + 1`, jih `y − 1`, východ `x + 1`, západ `x − 1`.

Příklad: z (0, 0, N) příkazy `F R F F L B`:

| příkaz | F | R | F | F | L | B |
|---|---|---|---|---|---|---|
| poloha | (0, 1) N | (0, 1) E | (1, 1) E | (2, 1) E | (2, 1) N | (2, 0) N |

Návrat bere ze zásobníku B, L, F, F, R, F a provede F, R, B, B, L, B. Robot skončí na (0, 0) čelem na sever.

> [!trap] Proč návrat přes zásobník a s opačnými příkazy
> Vrátit se jde jen tak, že se poslední pohyb zruší jako první (LIFO). Kdyby se příkazy rušily v pořadí zadání (fronta), robot by se otáčel v jiných místech a do startu by se nevrátil.

> [!note] Na co se můžou zeptat
> - **Co je LIFO a FIFO?** Zásobník vydá jako první poslední vloženou hodnotu, fronta první vloženou.
> - **Proč je fronta kruhová?** Index se posouvá modulo MAX, takže místa uvolněná na začátku pole se znovu použijí.
> - **Kdy je zásobník prázdný a kdy plný?** Prázdný při `top == -1`, plný při `top == MAX - 1`.
> - **Proč se překládají tři .c soubory najednou?** Těla `push`, `enqueue` atd. jsou v `stack.c` a `queue.c`. Bez nich linker funkce nenajde.
> - **Proč robot používá frontu i zásobník?** Fronta zachová pořadí zadání pro provádění, zásobník obrácené pořadí pro návrat.
