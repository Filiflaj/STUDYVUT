+++
title = "Metoda razítek a simulátor MicroCap"
topics = ["Metoda razítek", "MicroCap"]
+++

Devátý týden má dvě části. První rozšiřuje MUN o **řízené zdroje**, kterými se modelují tranzistory: jejich příspěvek do matice vodivostí se zapisuje jako „razítko“. Procvičuje se ve **VC5**. Druhá část je úvod do obvodového simulátoru **MicroCap**, kterým se dají ověřit výsledky laboratorních úloh i výpočetních cvičení.

## Zdroj proudu řízený napětím (ZPŘN)

Řízený zdroj nemá pevnou hodnotu: jeho proud závisí na napětí jinde v obvodu. **Zdroj proudu řízený napětím** dodává proud

$$I = g_\text{m}\,U_{12}$$

kde $U_{12} = U_{10} - U_{20}$ je řídicí napětí mezi uzly 1 a 2 a $g_\text{m}$ je **přenosová vodivost** (strmost, v S nebo mS). Proud teče z uzlu 3 do uzlu 4 (ven z uzlu 3 a do uzlu 4).

![Modely](images/razitka-modely.svg "a) zdroj proudu řízený napětím U12, b) model unipolárního tranzistoru, c) model bipolárního tranzistoru")

V rovnicích MUN je proud $g_\text{m}(U_{10} - U_{20})$, který z uzlu 3 vytéká, na **levé** straně rovnice uzlu 3 s +, a v uzlu 4 s −. Do matice se tedy přičte:

> [!key] Razítko ZPŘN
> Řádky = výstupní uzly (3, z něj proud vytéká; 4, do něj vtéká), sloupce = řídicí uzly (1 = +, 2 = −):
> $$\begin{array}{c|cc} & 1 & 2 \\ \hline 3 & +g_\text{m} & -g_\text{m} \\ 4 & -g_\text{m} & +g_\text{m} \end{array}$$
> Řádky a sloupce, které patří **referenčnímu uzlu**, se vynechají. Matice s razítkem **přestane být symetrická**.

## Modely tranzistorů

**Unipolární tranzistor (FET)** v zapojení se společným sourcem: hradlo G neodebírá proud (je rozpojené), mezi drainem a sourcem je ZPŘN

$$I_\text{D} = g_\text{m}\,U_\text{GS} \quad (\text{z D do S}),$$

řídicí uzly G (+) a S (−), výstupní uzly D (vytéká) a S (vtéká).

**Bipolární tranzistor (BJT)** v zapojení se společným emitorem: mezi bází a emitorem je odpor $R_\text{BE}$ a mezi kolektorem a emitorem ZPŘN

$$I_\text{C} = g_\text{m}\,U_\text{BE} \quad (\text{z C do E}),$$

řídicí uzly B (+) a E (−), výstupní uzly C (vytéká) a E (vtéká).

## Postup

1. Nakreslit **náhradní (střídavý) model**: tranzistory nahradit modely, napájecí zdroj považovat za zkrat (svorka napájení = referenční uzel), sestavit razítka.
2. Sestavit matici MUN pro **pasivní prvky a nezávislé zdroje** obvyklým způsobem.
3. **Přičíst razítka** řízených zdrojů na jejich pozice.
4. Vyřešit. Pro dvojbran se vstupním zdrojem proudu $I$ v uzlu 1 a výstupem v uzlu $k$:

$$K_U = \frac{U_{k0}}{U_{10}} = \frac{\Delta_k}{\Delta_1}, \qquad R_\text{vst} = \frac{U_{10}}{I} = \frac{\Delta_1}{\Delta\,I}$$

$I$ se při výpočtu přenosu vykrátí, takže se může nechat obecně (nebo dosadit 1).

## Vzorový příklad: emitorový sledovač

Bipolární tranzistor v zapojení se společným kolektorem: vstup do báze (uzel 1) se zdrojem proudu $I$ a odporem $R_\text{B}$ k napájení, výstup na emitoru (uzel 2) s $R_\text{E}$ proti zemi, kolektor na napájení (střídavě zem). $R_\text{B}$ = 56 kΩ, $R_\text{E}$ = 1 kΩ, $g_\text{m}$ = 40 mS, $R_\text{BE}$ = 5 kΩ.

> [!example] Matice
> Vodivosti v mS: $G_\text{B} = 1/56 = 0{,}017857$, $G_\text{BE} = 0{,}2$, $G_\text{E} = 1$.
>
> Razítko: řídicí uzly B = 1 a E = 2; výstup C = 0 (řádek se vynechá) a E = 2 (proud do něj vtéká, řádek 2: $-g_\text{m}$ ve sloupci 1, $+g_\text{m}$ ve sloupci 2).
> $$\begin{bmatrix} G_\text{B} + G_\text{BE} & -G_\text{BE} \\ -G_\text{BE} - g_\text{m} & G_\text{BE} + G_\text{E} + g_\text{m} \end{bmatrix}\cdot\begin{bmatrix} U_1 \\ U_2 \end{bmatrix} = \begin{bmatrix} I \\ 0 \end{bmatrix}, \qquad \begin{bmatrix} 0{,}21786 & -0{,}2 \\ -40{,}2 & 41{,}2 \end{bmatrix}\cdot\begin{bmatrix} U_1 \\ U_2 \end{bmatrix} = \begin{bmatrix} I \\ 0 \end{bmatrix}$$

> [!example] Řešení
> $$\Delta = 0{,}21786\cdot41{,}2 - (-0{,}2)\cdot(-40{,}2) = 8{,}97583 - 8{,}04 = 0{,}93583$$
> $$\Delta_1 = \begin{vmatrix} I & -0{,}2 \\ 0 & 41{,}2 \end{vmatrix} = 41{,}2\,I, \qquad \Delta_2 = \begin{vmatrix} 0{,}21786 & I \\ -40{,}2 & 0 \end{vmatrix} = 40{,}2\,I$$
> $$K_U = \frac{\Delta_2}{\Delta_1} = \frac{40{,}2}{41{,}2} = 0{,}976 \approx 0{,}98, \qquad R_\text{vst} = \frac{\Delta_1}{\Delta\,I} = \frac{41{,}2}{0{,}93583} = 44{,}03\ \text{k}\Omega$$
> (mS v matici, proto $R_\text{vst}$ vychází v kΩ.) Emitorový sledovač má přenos skoro 1 a velký vstupní odpor.
>
> **Jak to říct, když se zeptají:** „Pasivní prvky zapíšu do matice jako obvykle a k nim přičtu razítko řízeného zdroje: do řádku uzlu, do kterého proud zdroje vtéká, píšu $-g_\text{m}$ pod kladný řídicí uzel a $+g_\text{m}$ pod záporný. Kolektor je na napájení, to je střídavě zem, takže jeho řádek odpadne. Přenos je podíl $\Delta_2/\Delta_1$, vstupní odpor $U_1/I$.“

> [!trap] Na co si dát pozor
> - Matice s řízeným zdrojem **není symetrická**. Kdo ji „opraví“ na symetrickou, má chybu.
> - Determinant je rozdíl dvou blízkých čísel (8,976 − 8,04). Vodivosti je proto nutné psát na dost míst; s $G_{11}$ zaokrouhleným na 0,218 vyjde $\Delta = 0{,}9416$ a $R_\text{vst} = 43{,}76$ kΩ místo 44,03 kΩ.
> - Hradlo FET do matice nic nepřidává (nekonečný odpor), báze BJT ano ($G_\text{BE}$).

## Obvodový simulátor MicroCap

Simulátor spočítá obvod podle modelů součástek, bez stavby prototypu. Známé programy: TINA, OrCAD, MicroCap (volně ke stažení).

**Výhody:** není potřeba stavět prototyp (čas, peníze), dají se sledovat veličiny i v místech, kde by se měřit nedalo, a nejsou potřeba drahé přístroje.

**Nevýhody:** výsledek je jen tak dobrý, jak dobré jsou **modely** součástek; těžko se zadávají parazitní vazby, tepelné vazby a prvky s rozprostřenými parametry; většinou se nezahrnují vnější pole; složité obvody jsou výpočetně náročné.

### Typy analýz

| Analýza | Co dělá |
|---|---|
| **BIAS** (pracovní bod) | stejnosměrný pracovní bod, výchozí krok dalších analýz |
| **DC** | opakovaný výpočet pracovního bodu při změně jednoho nebo dvou parametrů (napětí zdroje, teplota, parametr součástky) |
| **AC** | kmitočtová analýza: obvod se v pracovním bodě **linearizuje** a počítají se komplexní napětí a proudy v rozsahu kmitočtů; malosignálová, nelinearitu nepostihne |
| **Transient** | časové průběhy s počátečními podmínkami, respektuje nelinearitu |

U všech analýz jde parametry **rozmítat** (lineárně, logaritmicky, výčtem hodnot), případně i teplotu.

### Praktické ovládání

- Ve schématu musí být **zem** (referenční uzel, REF), jinak simulace nepoběží.
- Hodnoty se zadávají s **desetinnou tečkou** (0.003), ve vědecké notaci (1e-3, 1E+4) nebo s předponami k, p, n, m, MEG; inženýrský zápis 1k2 = 1.2k = 1200.
- Zdroj napětí: Battery, nebo obecný zdroj s hodnotou v poli DC; zdroj proudu obdobně. **Pozor na orientaci** napětí a proudu; když vyjde opačné znaménko, stačí součástku otočit o 180°.
- Napětí přímo na součástce se zobrazí vzorcem v textu, třeba `=V(R3)`.
- **Dynamic DC** ukazuje uzlová napětí, proudy a výkony přímo ve schématu a přepočítává je při každé změně (vybraný rezistor jde měnit šipkami).
- **DC** analýza kreslí závislost, třeba $V(R2) = f(V1)$; **Stepping** rozmítá parametr (např. $R_1$ = 50 až 200 Ω po 50 Ω) a v grafu je křivka pro každou hodnotu.

> [!trap] M neznamená mega
> V simulátorech (stejně jako ve SPICE) je **m = mili** a **MEG = mega**. Rezistor zadaný jako „1M“ má 1 mΩ, ne 1 MΩ.

> [!example] První kroky ze cvičení
> Dělič z baterie 5 V a dvou rezistorů 100 Ω: na každém rezistoru 2,5 V, proud 25 mA, každý rezistor má výkon $0{,}025^2\cdot100 = 62{,}5$ mW a zdroj dodává 125 mW. Simulací se ověřují i výsledky z LC1 (dělič 10 V, 220 Ω a 100 Ω: 6,875 V a 3,125 V), z VC2 (příklad 1: uzlová napětí −2,5 V a 12,5 V) a z LC3, kde se s diodou 1N4001 místo $R_2$ ukáže, že superpozice v nelineárním obvodu neplatí.
