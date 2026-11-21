+++
title = "Metoda smyčkových proudů (MSP)"
topics = ["Metoda smyčkových proudů"]
+++

Univerzální metoda pro obvody s více zdroji. Místo proudů všech větví se zavedou **smyčkové proudy** v nezávislých smyčkách (okách) a pro každé oko se napíše 2. Kirchhoffův zákon. Soustava se sestaví rovnou ze schématu jako matice a vyřeší Cramerovým pravidlem. Procvičuje se ve **VC1**.

## Princip

- MSP vychází z **2. Kirchhoffova zákona**.
- V obvodu smějí být jen **zdroje napětí**. Reálné zdroje proudu se převedou na zdroje napětí: $U_\text{z} = I_\text{z} R_\text{i}$ (kde $R_\text{i} = 1/G_\text{i}$) s tímtéž $R_\text{i}$ v sérii. Ideální zdroj proudu bez paralelního odporu převést nejde.
- Řeší se maticová rovnice $\mathbf{R}\cdot\mathbf{I}_\text{S} = \mathbf{U}$.
- Počet rovnic = počet nezávislých smyček (ok) = počet větví − (počet uzlů − 1).

> [!key] Převod zdroje proudu na zdroj napětí
> $$U_\text{z} = \frac{I_\text{z}}{G_\text{i}} = I_\text{z} R_\text{i}$$
> Šipka proudu zdroje proudu míří ke svorce, která bude u zdroje napětí **kladná**. Šipka napětí míří od + k −, takže šipka $U_\text{z}$ míří **proti** šipce $I_\text{z}$.

## Přímé sestavení matice

1. Zvol nezávislé smyčky, nejlépe **oka** sítě.
2. Vyznač smyčkové proudy $I_\text{S1}, I_\text{S2}, \dots$, **všechny stejným směrem** (třeba po směru hodinových ručiček).
3. Připrav maticový zápis $\mathbf{R}\cdot\mathbf{I}_\text{S} = \mathbf{U}$.
4. Zapiš odporovou matici:
   - **diagonála** $R_{kk}$: součet všech odporů ve smyčce $k$, znaménko **+**,
   - **mimo diagonálu** $R_{jk}$: odpor společný smyčkám $j$ a $k$, znaménko **−** (smyčkové proudy jím tečou proti sobě); matice je symetrická.
5. Zapiš vektor napětí zdrojů: v řádku $k$ součet napětí zdrojů ve smyčce $k$,
   - **+**, když smyčkový proud prochází zdrojem **proti** šipce napětí (zdroj ho „pohání“, proud vstupuje do − a vystupuje z +),
   - **−**, když prochází **po** šipce.
6. Vyřeš soustavu (Cramerovo pravidlo).
7. Dopočítej proudy větví a napětí: proud větve, kterou teče jen jeden smyčkový proud, je přímo on (nebo jeho opak); proud společné větve je rozdíl smyčkových proudů.

> [!trap] Znaménko zdroje
> Nejčastější chyba MSP. Šipka napětí míří od + k −. Když jde smyčkový proud zdrojem **proti** šipce, prochází zdrojem od − k + a zdroj ho pohání: **+U**. Když jde **po** šipce: **−U**.

## Cramerovo pravidlo

Pro soustavu 2 × 2:

$$\begin{bmatrix} R_{11} & R_{12} \\ R_{21} & R_{22} \end{bmatrix}\cdot\begin{bmatrix} I_\text{S1} \\ I_\text{S2} \end{bmatrix} = \begin{bmatrix} U_1 \\ U_2 \end{bmatrix}, \qquad I_\text{S1} = \frac{\Delta_1}{\Delta}, \quad I_\text{S2} = \frac{\Delta_2}{\Delta}$$

kde $\Delta$ je determinant matice soustavy a $\Delta_k$ determinant matice, ve které je $k$-tý **sloupec** nahrazený vektorem pravé strany. Determinant 2 × 2 je „hlavní diagonála minus vedlejší“:

$$\begin{vmatrix} a & b \\ c & d \end{vmatrix} = ad - bc$$

Pro 3 × 3 se v semináři používá **Sarrusovo pravidlo**: součet tří součinů ve směru hlavní diagonály minus součet tří součinů ve směru vedlejší diagonály.

$$\begin{vmatrix} a_{11} & a_{12} & a_{13} \\ a_{21} & a_{22} & a_{23} \\ a_{31} & a_{32} & a_{33} \end{vmatrix} = a_{11}a_{22}a_{33} + a_{12}a_{23}a_{31} + a_{13}a_{21}a_{32} - a_{13}a_{22}a_{31} - a_{11}a_{23}a_{32} - a_{12}a_{21}a_{33}$$

Stejný výsledek dá rozvoj podle prvního řádku.

## Vzorový příklad z prezentace

Obvod: zdroj proudu $I_\text{Z1}$ = 200 mA s paralelním $R_1$ = 2,4 kΩ, za ním $R_3$ = 1,6 kΩ, příčný $R_4$ = 5 kΩ a větev $R_2$ = 6 kΩ se zdrojem $U_\text{Z2}$ = 300 V. Určete proudy všech rezistorů.

**Převod zdroje proudu:** $U_\text{Z1} = I_\text{Z1} R_1 = 0{,}2\cdot2400 = 480$ V v sérii s $R_1$. Šipka $I_\text{Z1}$ míří dolů, kladná svorka náhradního zdroje je tedy dole a šipka $U_\text{Z1}$ míří nahoru.

![Obvod po převodu zdroje](images/msp-priklad.svg "Zdroj Uz1 = 480 V s R1, rezistory R3, R4, R2 a zdroj Uz2; smyčkové proudy IS1 a IS2 po směru hodinových ručiček")

> [!example] Sestavení matice
> Smyčka 1: $R_1 + R_3 + R_4$. Proud $I_\text{S1}$ jde zdrojem $U_\text{Z1}$ nahoru, **po** šipce: $-U_\text{Z1}$.
>
> Smyčka 2: $R_2 + R_4$. Proud $I_\text{S2}$ jde zdrojem $U_\text{Z2}$ dolů, šipka míří nahoru, tedy **proti** šipce: $+U_\text{Z2}$.
>
> Společný je $R_4$, mimo diagonálu $-R_4$:
> $$\begin{bmatrix} R_1 + R_3 + R_4 & -R_4 \\ -R_4 & R_2 + R_4 \end{bmatrix}\cdot\begin{bmatrix} I_\text{S1} \\ I_\text{S2} \end{bmatrix} = \begin{bmatrix} -U_\text{Z1} \\ U_\text{Z2} \end{bmatrix}, \qquad \begin{bmatrix} 9000 & -5000 \\ -5000 & 11\,000 \end{bmatrix}\cdot\begin{bmatrix} I_\text{S1} \\ I_\text{S2} \end{bmatrix} = \begin{bmatrix} -480 \\ 300 \end{bmatrix}$$

> [!example] Řešení Cramerovým pravidlem
> $$\Delta = 9000\cdot11\,000 - (-5000)\cdot(-5000) = 99\cdot10^{6} - 25\cdot10^{6} = 7{,}4\cdot10^{7}$$
> $$\Delta_1 = \begin{vmatrix} -480 & -5000 \\ 300 & 11\,000 \end{vmatrix} = -5{,}28\cdot10^{6} + 1{,}5\cdot10^{6} = -3{,}78\cdot10^{6}$$
> $$\Delta_2 = \begin{vmatrix} 9000 & -480 \\ -5000 & 300 \end{vmatrix} = 2{,}7\cdot10^{6} - 2{,}4\cdot10^{6} = 3\cdot10^{5}$$
> $$I_\text{S1} = \frac{-3{,}78\cdot10^{6}}{7{,}4\cdot10^{7}} = -51{,}08\ \text{mA}, \qquad I_\text{S2} = \frac{3\cdot10^{5}}{7{,}4\cdot10^{7}} = 4{,}054\ \text{mA}$$

> [!example] Proudy větví
> Šipky proudů v původním obvodu: $I_{R3}$ a $I_{R2}$ míří doleva, $I_{R4}$ a $I_{R1}$ nahoru. $I_\text{S1}$ teče přes $R_3$ doprava a přes $R_4$ dolů, $I_\text{S2}$ přes $R_2$ doprava a přes $R_4$ nahoru.
> $$I_{R3} = -I_\text{S1} = 51{,}08\ \text{mA}, \qquad I_{R2} = -I_\text{S2} = -4{,}054\ \text{mA}, \qquad I_{R4} = I_\text{S2} - I_\text{S1} = 4{,}054 + 51{,}08 = 55{,}14\ \text{mA}$$
> **Pozor na $R_1$:** proud rezistorem $R_1$ v náhradním obvodu **není** proud $R_1$ v původním obvodu (převodem zdroje se změnila jeho větev). Dopočítá se z 1. KZ pro horní uzel původního obvodu ($I_\text{Z1}$ z uzlu vytéká do zdroje, $I_{R3}$ a $I_{R1}$ do uzlu vtékají):
> $$I_\text{Z1} - I_{R3} - I_{R1} = 0 \quad\Rightarrow\quad I_{R1} = 200 - 51{,}08 = 148{,}9\ \text{mA}$$
> **Kontrola:** $I_{R1} + I_{R3} - I_\text{Z1} = 148{,}9 + 51{,}08 - 200 \approx 0$ a $I_{R2} + I_{R4} - I_{R3} = -4{,}054 + 55{,}14 - 51{,}08 \approx 0$.
>
> **Jak to říct, když se zeptají:** „Zdroj proudu jsem převedl na zdroj napětí 480 V se stejným odporem 2,4 kΩ v sérii. Diagonála je součet odporů ve smyčce, mimo diagonálu je se záporným znaménkem společný $R_4$. Na pravou stranu dávám napětí zdrojů: plus, když jde smyčkový proud proti šipce zdroje. Proud $R_1$ beru z 1. KZ v původním obvodu, protože v náhradním je to jiný proud.“

## Soustava 3 × 3 a Sarrusovo pravidlo (VC1, příklad 4)

![Obvod VC1 příklad 4](images/vc1-p4.svg "Tři oka: levé (Uz1, R1, R3), pravé (R2, Uz2, R5) a dolní (R3, R5, R4)")

$U_\text{z1}$ = 24 V, $U_\text{z2}$ = 30 V, $R_1$ = 20 Ω, $R_2$ = 10 Ω, $R_3$ = 250 Ω, $R_4$ = 150 Ω, $R_5$ = 100 Ω, oba zdroje mají šipku dolů. Tři oka, všechny smyčkové proudy po směru hodinových ručiček.

> [!example] Matice
> - oko 1 ($U_\text{z1}$, $R_1$, $R_3$): $I_\text{S1}$ jde zdrojem nahoru proti šipce, $+24$; s okem 3 sdílí $R_3$,
> - oko 2 ($R_2$, $U_\text{z2}$, $R_5$): $I_\text{S2}$ jde zdrojem dolů po šipce, $-30$; s okem 3 sdílí $R_5$,
> - oko 3 ($R_5$, $R_4$, $R_3$): bez zdroje, $0$; oka 1 a 2 nemají společný prvek, proto 0.
> $$\begin{bmatrix} R_1 + R_3 & 0 & -R_3 \\ 0 & R_2 + R_5 & -R_5 \\ -R_3 & -R_5 & R_3 + R_4 + R_5 \end{bmatrix} = \begin{bmatrix} 270 & 0 & -250 \\ 0 & 110 & -100 \\ -250 & -100 & 500 \end{bmatrix}, \qquad \mathbf{U} = \begin{bmatrix} 24 \\ -30 \\ 0 \end{bmatrix}$$

> [!example] Determinanty Sarrusovým pravidlem
> $$\Delta = 270\cdot110\cdot500 + 0 + 0 - (-250)\cdot110\cdot(-250) - 270\cdot(-100)\cdot(-100) - 0 = 14\,850\,000 - 6\,875\,000 - 2\,700\,000 = 5\,275\,000$$
> $$\Delta_1 = \begin{vmatrix} 24 & 0 & -250 \\ -30 & 110 & -100 \\ 0 & -100 & 500 \end{vmatrix} = 1\,320\,000 + 0 - 750\,000 - 0 - 240\,000 - 0 = 330\,000$$
> $$\Delta_2 = \begin{vmatrix} 270 & 24 & -250 \\ 0 & -30 & -100 \\ -250 & 0 & 500 \end{vmatrix} = -4\,050\,000 + 600\,000 + 0 - (-1\,875\,000) - 0 - 0 = -1\,575\,000$$
> $$\Delta_3 = \begin{vmatrix} 270 & 0 & 24 \\ 0 & 110 & -30 \\ -250 & -100 & 0 \end{vmatrix} = 0 + 0 + 0 - (-660\,000) - 810\,000 - 0 = -150\,000$$
> $$I_\text{S1} = \frac{330\,000}{5\,275\,000} = 62{,}56\ \text{mA}, \quad I_\text{S2} = \frac{-1\,575\,000}{5\,275\,000} = -298{,}6\ \text{mA}, \quad I_\text{S3} = \frac{-150\,000}{5\,275\,000} = -28{,}44\ \text{mA}$$

> [!example] Proudy větví podle šipek v zadání
> $$I_1 = I_\text{S1} = 62{,}56\ \text{mA}, \quad I_2 = -I_\text{S2} = 298{,}6\ \text{mA}, \quad I_3 = I_\text{S1} - I_\text{S3} = 91{,}00\ \text{mA}$$
> $$I_4 = I_\text{S3} = -28{,}44\ \text{mA}, \quad I_5 = I_\text{S3} - I_\text{S2} = -28{,}44 + 298{,}58 = 270{,}1\ \text{mA}$$
> Shoduje se s výsledky zadání. Záporné $I_4$ znamená, že proud teče rezistorem $R_4$ doprava, proti šipce.
>
> **Jak to říct, když se zeptají:** „Proud větve je smyčkový proud, když jí teče jen jeden; když jí tečou dva, je to jejich rozdíl, a znaménko určím podle toho, který z nich teče ve směru šipky větve.“

Ostatní příklady VC1 jsou i s celým postupem v kvízu **vc1**.
