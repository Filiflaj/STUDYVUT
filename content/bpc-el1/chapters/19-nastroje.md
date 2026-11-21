+++
title = "Cvičení: kalkulačka, LinRov, MicroCap a Excel"
topics = ["Nástroje: kalkulačka, LinRov, MicroCap"]
+++

Ruční Cramerovo pravidlo je potřeba umět (v testu se píše postup), ale kontrolovat a počítat větší soustavy se vyplatí nástroji. PC3 je věnované kalkulačce a LinRovu, MicroCap se používá v laborkách (superpozice) a Excel v PC11.

## Kalkulačka

Pro EL1 se hodí vědecká kalkulačka s **maticemi** a **komplexními čísly** (Casio fx-991EX, fx-991ES Plus, fx-570ES Plus, Canon F-789SGA a podobné). Postup na Casiu:

1. Reset a řádkové zobrazení (aby výsledky nebyly zlomky).
2. Režim MAT: zadat MatA (matice soustavy) a MatB (vektor pravých stran) s rozměry $n\times n$ a $n\times1$.
3. Řešení $\mathbf{X} = \mathbf{A}^{-1}\mathbf{B}$: MatA⁻¹ × MatB. Determinant: det(MatA).
4. Mezivýsledky (čísla, ne matice) do pamětí A, B, C, D, X, Y, M.

Režim EQN řeší soustavu rovnou, ale neukáže determinanty. Pro Cramerovo pravidlo se determinanty $\Delta_k$ spočítají jako det matic s nahrazeným sloupcem.

## LinRov

Jednoduchý program pro soustavy 1. až 5. řádu ve tvaru $\mathbf{K}\cdot\mathbf{X} = \mathbf{Y}$ (ke stažení z e-learningu předmětu). Zadá se řád, prvky matice a pravé strany, tlačítko **Výpočet** ukáže řešení i **determinanty** pro Cramerovo pravidlo. U symetrické matice (MUN, MSP bez řízených zdrojů) stačí zadat horní trojúhelník a tlačítkem Kopíruj ho zrcadlit. Čísla jde psát i ve vědeckém tvaru (1,6625E-5).

## MicroCap

Obvodový simulátor: obvod se nakreslí a program spočítá napětí a proudy. Výhoda: žádný prototyp, vidět jsou i veličiny, které by šly změřit jen těžko. Nevýhoda: výsledek je jen tak dobrý jako modely součástek; parazitní vazby, teplo a rušení model nezná.

| analýza | co dělá |
|---|---|
| **pracovní bod (BIAS)** | stejnosměrný ustálený stav, výchozí bod ostatních analýz |
| **DC** | opakovaný pracovní bod při měnícím se napětí zdroje, teplotě nebo parametru (charakteristiky) |
| **AC** | kmitočtová analýza v linearizovaném obvodu (malé signály) |
| **Transient** | časové průběhy, s nelinearitami a počátečními podmínkami |

U všech analýz jde parametry **rozmítat** (Stepping): lineárně, logaritmicky nebo výčtem hodnot.

> [!trap] Hodnoty v MicroCapu
> - Desetinná **tečka**, ne čárka: `0.003`, `1e-3`, `1E+4`.
> - Předpony: `k`, `m`, `u`, `n`, `p`; **`m` je mili**, mega se píše **`MEG`**. Rezistor zadaný jako `1M` má 1 mΩ, ne 1 MΩ.
> - Inženýrský zápis: `1k2` = 1,2 kΩ.
> - Ve schématu musí být **zem** (referenční uzel), jinak se analýza nespustí.
> - Pozor na orientaci zdrojů napětí a proudu při vkládání.

Užitečné: **Dynamic DC** zobrazí uzlová napětí, proudy a výkony přímo ve schématu a reaguje na změny hodnot; text se vzorcem `=V(R3)` ukáže napětí na rezistoru R3; **Slider** mění hodnotu součástky posuvníkem; v DC analýze se vykreslí závislost, třeba $V(R2) = f(V1)$.

## Excel pro nelineární obvody

Iterační metody (Newtonova, půlení intervalu) a aproximace metodou nejmenších čtverců se pohodlně počítají v tabulce: jeden řádek na iteraci, sloupce $k$, $u_{(k)}$, $f(u_{(k)})$, $f'(u_{(k)})$, $\varepsilon_{(k)}$ a vzorec, který vezme hodnotu z řádku nad sebou. Soubor ze cvičení 11 tak řeší příklady sbírky 10.14 (přímka metodou nejmenších čtverců), 10.6 (linearizace Zenerovy diody), 10.11 (půlení intervalu) a 10.17 (Newtonova metoda pro diodu, z odhadu 0,7 V vyjde $U_\text{d} = 0{,}6407\ \text{V}$).
