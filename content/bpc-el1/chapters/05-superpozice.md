+++
title = "Úloha 5: Princip superpozice"
topics = ["Princip superpozice"]
+++

Obvod se dvěma zdroji se měří třikrát: s oběma zdroji, jen s $U_\text{z1}$ a jen s $U_\text{z2}$. V lineárním obvodu se dílčí odezvy sečtou na celkovou, s diodou ne. Hodnoty: $R_1$ = 470 Ω, $R_2$ = 150 Ω, $R_3$ = 330 Ω, $R_\text{i1}$ = 330 Ω, $R_\text{i2}$ = 100 Ω; nelineární prvek je dioda 1N4007. Napětí zdrojů se měří v labu, výsledky se srovnávají se simulací v MicroCapu.

![Obvod se dvěma zdroji](images/superpozice-obvod.svg "Zdroje Uz1 (s Ri1) a Uz2 (s Ri2), mezi uzly 1 a 2 lineární R3 nebo nelineární prvek")

## Princip superpozice

Lineární prvek má odpor nezávislý na napětí a proudu ($U = RI$). V obvodu jen z lineárních prvků a nezávislých zdrojů platí:

- **úměrnost:** $k$-krát větší zdroje, $k$-krát větší všechna napětí a proudy (úloha 2),
- **sčítání účinků:** každé napětí i proud je součtem dílčích odezev, kdy působí vždy jen jeden zdroj.

$$U_{10} = U_{10}' + U_{10}'' \qquad (U' \text{ jen od } U_\text{z1},\ U'' \text{ jen od } U_\text{z2})$$

Obecně je odezva lineární kombinací zdrojů, $U_{10} = k_1 U_\text{z1} + k_2 U_\text{z2}$, a koeficienty závisí jen na odporech.

> [!key] Zdroj, který nepůsobí, se nahradí svým vnitřním odporem
> ideální zdroj napětí → **zkrat**, ideální zdroj proudu → **rozpojení**. Vnitřní odpory reálných zdrojů ($R_\text{i1}$, $R_\text{i2}$) v obvodu zůstávají.

![Dílčí obvody](images/superpozice-dilci.svg "a) působí jen Uz1, Uz2 je nahrazený zkratem; b) působí jen Uz2, Uz1 je nahrazený zkratem")

Dílčí obvody se vyřeší sériovým a paralelním řazením a děličem. Pro jen $U_\text{z1}$ je $R_\text{i2}$ paralelně k $R_2$:

$$U_{10}' = U_\text{z1}\,\frac{R_\text{a}}{R_\text{i1}+R_\text{a}}, \quad R_\text{a} = R_1 \parallel \bigl(R_3 + R_2 \parallel R_\text{i2}\bigr), \qquad U_{20}' = U_{10}'\,\frac{R_2 \parallel R_\text{i2}}{R_3 + R_2 \parallel R_\text{i2}}$$

a symetricky pro jen $U_\text{z2}$. Po dosazení ($R_\text{a}$ = 213,1 Ω, $R_\text{b}$ = 116,6 Ω):

$$U_{10} = 0{,}3924\,U_\text{z1} + 0{,}1992\,U_\text{z2}, \qquad U_{20} = 0{,}0604\,U_\text{z1} + 0{,}5383\,U_\text{z2}$$

> [!example] Kontrola stejných koeficientů přes MUN
> Se zdroji převedenými na zdroje proudu (úloha 4) a vodivostmi v mS: $G_\text{i1} = 3{,}030$, $G_1 = 2{,}128$, $G_3 = 3{,}030$, $G_\text{i2} = 10{,}000$, $G_2 = 6{,}667$.
> $$\begin{bmatrix} 8{,}188 & -3{,}030 \\ -3{,}030 & 19{,}697 \end{bmatrix}\cdot\begin{bmatrix} U_{10} \\ U_{20} \end{bmatrix} = \begin{bmatrix} 3{,}030\,U_\text{z1} \\ 10{,}000\,U_\text{z2} \end{bmatrix}$$
> Cramerovo pravidlo (2×2: hlavní diagonála minus vedlejší):
> $$D = 8{,}188\cdot19{,}697 - (-3{,}030)(-3{,}030) = 161{,}279 - 9{,}181 = 152{,}098$$
> $$D_1 = 59{,}682\,U_\text{z1} + 30{,}300\,U_\text{z2}, \qquad D_2 = 9{,}181\,U_\text{z1} + 81{,}880\,U_\text{z2}$$
> $$U_{10} = \frac{D_1}{D} = 0{,}3924\,U_\text{z1} + 0{,}1992\,U_\text{z2}, \qquad U_{20} = \frac{D_2}{D} = 0{,}0604\,U_\text{z1} + 0{,}5383\,U_\text{z2}$$
> První člen je odezva od $U_\text{z1}$, druhý od $U_\text{z2}$: superpozice je vidět přímo ve výsledku.

## Kdy superpozice neplatí

**Nelineární prvek.** Dioda nemá konstantní odpor: v propustném směru vede až od asi 0,6 až 0,7 V a pak je na ní skoro stálé napětí, v závěrném směru nevede. V každém dílčím obvodu pracuje v jiném bodě charakteristiky, takže $f(U' + U'') \neq f(U') + f(U'')$. Typicky: s oběma zdroji nebo jen s $U_\text{z1}$ dioda vede, jen s $U_\text{z2}$ je v závěrném směru.

**Výkon.** Výkon je kvadratická funkce proudu:

$$P = R\,(I' + I'')^2 = R I'^2 + R I''^2 + 2R\,I'I'' \neq P' + P''$$

Výkon se proto počítá až z výsledného proudu nebo napětí.

Platnost se ověří odchylkou

$$\Delta U_{ij} = U_{ij} - (U_{ij}' + U_{ij}'')$$

V lineárním obvodu vyjde $\Delta U \approx 0$, v nelineárním řádově desetiny voltu až volt.

## Měření a simulace

- Polarita voltmetru: $U_{ij}$ s červeným kabelem na uzlu $i$. Záporné hodnoty se zapisují i se znaménkem, kabely se nepřehazují.
- Lineární obvod (přepínač na $R_3$): oba zdroje, pak $U_\text{z2}$ nahradit zkratem, pak $U_\text{z1}$ zkratem. Totéž s nelineárním prvkem.
- Kontrola: v každé trojici $U_{12} \approx U_{10} - U_{20}$ (2. KZ platí vždy, i s diodou).
- MicroCap (soubor `superpozice.cir`, listy Lineární a Nelineární): nastavit zdroje na naměřená napětí (desetinná **tečka**), opsat $U_{10}$, $U_{20}$; $U_{12} = U_{10} - U_{20}$ se dopočítá. V simulaci je dioda paralelně s rezistorem 330 Ω.

> [!example] Příklad z měření (nelineární obvod)
> Oba zdroje: $U_{10}$ = 3,20 V, $U_{20}$ = 2,55 V, $U_{12}$ = 0,65 V. Jen $U_\text{z1}$: 1,62 / 0,94 / 0,68 V. Jen $U_\text{z2}$: 0,66 / 1,79 / −1,12 V.
> $$\Delta U_{10} = 3{,}20 - (1{,}62 + 0{,}66) = 0{,}92\ \text{V}, \qquad \Delta U_{12} = 0{,}65 - \bigl(0{,}68 + (-1{,}12)\bigr) = 1{,}09\ \text{V}$$
> Simulace dala prakticky totéž (1,10 V). V lineárním obvodu vyšly odchylky v setinách voltu, v simulaci nejvýš 0,01 V. Na vodivé diodě bylo vždy asi 0,65 V (0,65 a 0,68 V), jen s $U_\text{z2}$ dioda nevedla a hodnoty $U''$ vyšly stejně jako v lineárním obvodu (proud šel přes paralelní rezistor 330 Ω, tedy stejný jako $R_3$).
>
> **Jak to říct, když se zeptají:** „Dioda je nelineární. S oběma zdroji a s $U_\text{z1}$ vede a drží asi 0,65 V, s $U_\text{z2}$ je v závěrném směru. Pracuje pokaždé jinde na charakteristice, a proto se dílčí odezvy nesčítají a $\Delta U$ vyjde kolem 1 V.“

> [!trap] Na co si dát pozor
> - $U''_{12}$ vychází **záporné** (s jen $U_\text{z2}$ je uzel 2 výš než uzel 1) a se znaménkem se dosazuje i do $\Delta U$.
> - Kirchhoffův test odhalí chybný odečet: v jednom měření lineárního obvodu bylo zapsané $U_{10}$ = 3,200 V, ale $U_{20} + U_{12}$ dávalo přes 4 V a simulace 3,991 V. Taková hodnota se nepřepisuje, vysvětlí se v závěru.
> - Odčítání v řádku Simulace: např. $0{,}663 - 1{,}792 = -1{,}129$, ne −1,29.
> - Pro superpozici má být napětí zdroje ve všech třech měřeních stejné; malé rozdíly (8,48 vs. 8,53 V) dávají setinové odchylky i v lineárním obvodu.
> - Zapomenutý přepínač: zdroj, který má být zkratovaný, je pořád zapojený.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Co říká princip superpozice?** V lineárním obvodu je odezva na více zdrojů součtem odezev na jednotlivé zdroje.
> - **Čím nahradím zdroj, který nepůsobí?** Vnitřním odporem: zdroj napětí zkratem, zdroj proudu rozpojením.
> - **Proč v obvodu zůstávají $R_\text{i1}$, $R_\text{i2}$?** Zkratuje se jen ideální zdroj napětí, vnitřní odpor patří k reálnému zdroji.
> - **Proč neplatí s diodou a pro výkon?** Dioda má odpor závislý na napětí; výkon je kvadratický ($P = U^2/R$).
> - **K čemu je superpozice v praxi?** Vícezdrojový obvod se řeší jako několik jednoduchých obvodů s jedním zdrojem (dělič, sériové a paralelní řazení) a je vidět podíl každého zdroje.
