+++
title = "Cvičení: řízené zdroje a metoda razítek"
topics = ["Řízené zdroje v MUN"]
+++

Tranzistor se pro malé signály nahrazuje lineárním modelem se **zdrojem proudu řízeným napětím (ZPŘN)**. Metodou uzlových napětí se pak spočítá napěťový přenos $K_U$ a vstupní odpor $R_\text{vst}$ zesilovače. ZPŘN se do matice vodivostí dopíše „razítkem“ (PC8, COZ4).

## Řízené zdroje

| zdroj | výstup | řídicí veličina | parametr |
|---|---|---|---|
| ZNŘN | napětí | napětí | zesílení $\mu$ (–) |
| ZNŘP | napětí | proud | přenosový odpor $r$ (Ω) |
| ZPŘN | proud | napětí | přenosová vodivost (strmost) $g_\text{m}$ (S) |
| ZPŘP | proud | proud | proudový zesilovací činitel $\beta$ (–) |

ZPŘN sedí MUN nejlépe: jeho proud je vodivost krát napětí, takže patří do matice $\mathbf{G}$ jako další vodivosti. Ostatní řízené zdroje se řeší modifikovanou MUN.

![ZPŘN a modely tranzistorů](images/cv-zprn.svg "ZPŘN mezi uzly c, d řízený napětím U mezi uzly a, b; model FET (jen ZPŘN) a bipolárního tranzistoru (ZPŘN a RBE)")

- **Unipolární tranzistor (FET)**: hradlo nevede proud, mezi D a S je ZPŘN s proudem $I_\text{D} = g_\text{m} U_\text{GS}$.
- **Bipolární tranzistor**: mezi B a E je odpor $R_\text{BE}$ (převádí proud báze na napětí $U_\text{BE}$), mezi C a E je ZPŘN s proudem $I_\text{C} = g_\text{m} U_\text{BE}$.
- Výpočetní schéma je **malosignálové**: napájecí zdroj se nahradí zkratem (jeho vnitřní odpor je nulový), takže jeho svorka splyne s referenčním uzlem.

## Razítko ZPŘN

ZPŘN s proudem $g_\text{m} U_{ab}$, který teče z uzlu $c$ do uzlu $d$ (uvnitř zdroje od $c$ k $d$), přidá do matice $\mathbf{G}$ čtyři čísla: **řádky** jsou výstupní uzly $c$, $d$, **sloupce** řídicí uzly $a$, $b$.

| | sloupec $a$ | sloupec $b$ |
|---|---|---|
| řádek $c$ | $+g_\text{m}$ | $-g_\text{m}$ |
| řádek $d$ | $-g_\text{m}$ | $+g_\text{m}$ |

Řádek a sloupec referenčního uzlu se vynechají. Proč: v rovnici uzlu $c$ (1. KZ, proudy z uzlu +) přibude proud zdroje $+g_\text{m}(U_{a0} - U_{b0})$; protože závisí na neznámých napětích, patří na levou stranu.

> [!key] Postup
> 1. Sestavit matici MUN pro pasivní prvky a nezávislé zdroje proudu.
> 2. Dopsat razítko každého ZPŘN.
> 3. Na vstup připojit zdroj proudu $I$ (libovolné hodnoty, třeba 1 A). Je to jediný nezávislý zdroj, takže pravá strana je $(I;\ 0;\ \dots)$.
> 4. Cramerovým pravidlem: $U_1 = \Delta_1/\Delta$, $U_2 = \Delta_2/\Delta$, a z nich
> $$K_U = \frac{U_2}{U_1} = \frac{\Delta_2}{\Delta_1}, \qquad R_\text{vst} = \frac{U_1}{I} = \frac{\Delta_1}{I\,\Delta}$$

Matice s razítkem **není symetrická** (řízený zdroj není reciproký prvek). Řízený zdroj se nikdy nepíše na pravou stranu.

![Výpočetní schéma zesilovače s FET](images/cv-fet-zesilovac.svg "Zdroj I do uzlu 1, G1 a G3 k zemi, G2 mezi uzly 1 a 2, ZPŘN gm·U10 z uzlu 2 k zemi")

> [!example] Sbírka 6.6: zesilovač s FET v zapojení SE
> $R_1 = 10\ \text{k}\Omega$, $R_2 = 120\ \text{k}\Omega$, $R_3 = 2{,}7\ \text{k}\Omega$, $g_\text{m} = 100\ \text{mS}$. Hradlo je uzel 1, kolektor (drain) uzel 2, source je uzemněný. ZPŘN $g_\text{m} U_{10}$ teče z uzlu 2 k zemi, razítko tedy přidá jen $+g_\text{m}$ do řádku 2, sloupce 1.
>
> $G_1 = 1\cdot10^{-4}\ \text{S}$, $G_2 = 8{,}333\cdot10^{-6}\ \text{S}$, $G_3 = 3{,}704\cdot10^{-4}\ \text{S}$:
> $$\begin{bmatrix} G_1 + G_2 & -G_2 \\ -G_2 + g_\text{m} & G_2 + G_3 \end{bmatrix}\begin{bmatrix} U_1 \\ U_2 \end{bmatrix} = \begin{bmatrix} I \\ 0 \end{bmatrix} \quad\Rightarrow\quad \begin{bmatrix} 1{,}083\cdot10^{-4} & -8{,}333\cdot10^{-6} \\ 9{,}999\cdot10^{-2} & 3{,}787\cdot10^{-4} \end{bmatrix}\begin{bmatrix} U_1 \\ U_2 \end{bmatrix} = \begin{bmatrix} I \\ 0 \end{bmatrix}$$
> $$\Delta = 1{,}083\cdot10^{-4}\cdot3{,}787\cdot10^{-4} - (-8{,}333\cdot10^{-6})\cdot9{,}999\cdot10^{-2} = 4{,}101\cdot10^{-8} + 8{,}332\cdot10^{-7} = 8{,}742\cdot10^{-7}$$
> $$\Delta_1 = \begin{vmatrix} I & -8{,}333\cdot10^{-6} \\ 0 & 3{,}787\cdot10^{-4} \end{vmatrix} = 3{,}787\cdot10^{-4}\,I, \qquad \Delta_2 = \begin{vmatrix} 1{,}083\cdot10^{-4} & I \\ 9{,}999\cdot10^{-2} & 0 \end{vmatrix} = -9{,}999\cdot10^{-2}\,I$$
> $$K_U = \frac{\Delta_2}{\Delta_1} = \frac{-9{,}999\cdot10^{-2}}{3{,}787\cdot10^{-4}} = -264{,}0, \qquad R_\text{vst} = \frac{\Delta_1}{I\,\Delta} = \frac{3{,}787\cdot10^{-4}}{8{,}742\cdot10^{-7}} = 433{,}2\ \Omega$$
> Zesilovač obrací fázi (záporné $K_U$) a má malý vstupní odpor, protože zpětnovazební $R_2$ je ze vstupu „vidět“ zmenšený zesílením.
>
> **Jak to říct, když se zeptají:** „Tranzistor jsem nahradil zdrojem proudu řízeným napětím hradla. Matici pro rezistory jsem doplnil razítkem: řádek výstupního uzlu, sloupec řídicího uzlu, $+g_\text{m}$. Na vstup dám zdroj proudu $I$; jeho velikost se při dělení determinantů zkrátí.“

## Další zapojení

- **SE s bipolárním tranzistorem** (sbírka 6.7): přibude $G_\text{BE}$ mezi bází a zemí, jinak stejné. Vyjde $K_U = -325{,}2$, $R_\text{vst} \approx 392\ \Omega$.
- **SC, emitorový sledovač** (PC8, př. 2; sbírka 6.8): řídí napětí $U_{12}$ mezi bází a emitorem, oba uzly jsou nezávislé, takže razítko má všechna čtyři čísla. Přenos je těsně pod 1 a vstupní odpor velký.
- **SB** (sbírka 6.21): báze je uzemněná, řídicí napětí je $U_{01} = -U_{10}$; přenos je kladný a vstupní odpor malý (jednotky ohmů).
- **Dva stupně** (PC8, př. 4; sbírka 6.9): matice 3×3 se dvěma razítky.

> [!note] Záporný vstupní odpor
> V PC8, př. 3 vyjde $R_\text{vst} = -81{,}16\ \Omega$: vstupní napětí a proud mají opačný směr, obvod do zdroje signálu energii vrací. Reálné dvojbrany se zpětnou vazbou to umějí, není to chyba výpočtu.

> [!trap] Na co si dát pozor
> - Razítko: **řádky = výstup** (kam teče proud zdroje), **sloupce = vstup** (které napětí ho řídí). Prohození dá jiný, chybný výsledek.
> - Řídicí napětí $U_\text{BE} = U_\text{B} - U_\text{E}$; když je emitor nezávislý uzel, dostane razítko i sloupec emitoru.
> - Napájecí zdroj v malosignálovém schématu je zkrat. Rezistor připojený k napájení je tedy připojený k zemi.
