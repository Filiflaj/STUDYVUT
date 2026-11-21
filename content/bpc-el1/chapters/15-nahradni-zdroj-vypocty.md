+++
title = "Cvičení: věty o náhradním zdroji"
topics = ["Věty o náhradním zdroji", "Náhradní zdroj", "Přenos výkonu"]
+++

Znění Théveninovy a Nortonovy věty a jejich ověření měřením jsou v kapitole **Úloha 3: Metoda náhradního zdroje**. Ve cvičeních (PC9, COZ4) se věta používá na obvody s více zdroji, na proměnnou zátěž, na můstek a na hledání zátěže s největším výkonem. Je to druhý příklad testu v PC11.

## Postup

Metoda se hodí, když hledám proud, napětí nebo výkon **jedné větve** (zátěže), hlavně pro více hodnot zátěže.

1. Zátěž odpojit, svorky označit a, b.
2. **$U_\text{i}$** = napětí na svorkách naprázdno. Spočítá se jakoukoli metodou (zjednodušení, MUN, MSP, superpozice).
3. **$R_\text{i}$** = odpor do svorek, když se ve zbytku obvodu **zdroje napětí zkratují a zdroje proudu rozpojí** (řízené zdroje se ponechají).
4. Zátěž připojit k náhradnímu zdroji: $I_\text{z} = U_\text{i}/(R_\text{i} + R_\text{z})$, $U_\text{z} = R_\text{z} I_\text{z}$, $P_\text{z} = U_\text{z} I_\text{z}$.

Nortonova varianta: $I_\text{i}$ = proud nakrátko mezi svorkami a, b, $G_\text{i} = 1/R_\text{i}$, a zátěží teče $I_\text{z} = I_\text{i}\,G_\text{z}/(G_\text{i} + G_\text{z})$ (dělič proudu). Obě varianty dávají totéž, protože $I_\text{i} = U_\text{i}/R_\text{i}$. Výhodnější je ta, u které jde snáz spočítat napětí naprázdno, resp. proud nakrátko.

![Obvod s dvěma zdroji a zátěží](images/cv-thevenin-priklad.svg "UZ1 s R1, R2 a zdroj proudu IZ2 paralelně, přes R3 na svorky a, b se zátěží RZ")

> [!example] PC9, př. 1 (COZ4, př. 4.9): Théveninova věta
> $U_\text{Z1} = 20\ \text{V}$, $I_\text{Z2} = 0{,}5\ \text{A}$, $R_1 = 15\ \Omega$, $R_2 = 25\ \Omega$, $R_3 = 35\ \Omega$, $R_\text{Z} = 10\ \Omega$.
>
> **$U_\text{i}$ (MUN, jeden uzel):** bez zátěže neteče $R_3$ proud, takže $U_\text{i}$ je napětí horního uzlu. Zdroj napětí převedu: $I_\text{Z1} = U_\text{Z1}/R_1 = 20/15 = 1{,}3333\ \text{A}$, $G_1 = 0{,}066\,67\ \text{S}$, $G_2 = 0{,}04\ \text{S}$.
> $$(G_1 + G_2)\,U_\text{i} = I_\text{Z1} + I_\text{Z2} \quad\Rightarrow\quad U_\text{i} = \frac{1{,}3333 + 0{,}5}{0{,}066\,67 + 0{,}04} = \frac{1{,}8333}{0{,}106\,67} = 17{,}19\ \text{V}$$
> **$R_\text{i}$ (zjednodušování):** $U_\text{Z1}$ zkrat, $I_\text{Z2}$ rozpojit; $R_1 \parallel R_2$ a k tomu v sérii $R_3$:
> $$R_\text{i} = R_3 + \frac{R_1 R_2}{R_1 + R_2} = 35 + \frac{15\cdot25}{15 + 25} = 35 + 9{,}375 = 44{,}375\ \Omega$$
> **Zátěž:**
> $$I_\text{Z} = \frac{17{,}19}{44{,}375 + 10} = 0{,}316\,1\ \text{A}, \quad U_\text{Z} = 10\cdot0{,}316\,1 = 3{,}161\ \text{V}, \quad P_\text{Z} = 3{,}161\cdot0{,}316\,1 = 0{,}999\ \text{W} \doteq 1\ \text{W}$$
> **Nortonova věta (PC9, př. 2):** se zkratovanými svorkami teče $R_3$ proud. MUN pro horní uzel: $(G_1 + G_2 + G_3)\,U = 1{,}8333$, $U = 1{,}8333/0{,}135\,24 = 13{,}556\ \text{V}$, $I_\text{i} = U/R_3 = 0{,}387\,3\ \text{A}$. S $G_\text{i} = 1/44{,}375 = 0{,}022\,54\ \text{S}$ a $G_\text{Z} = 0{,}1\ \text{S}$:
> $$I_\text{Z} = 0{,}387\,3\cdot\frac{0{,}1}{0{,}022\,54 + 0{,}1} = 0{,}316\,1\ \text{A}$$
>
> **Jak to říct, když se zeptají:** „Naprázdno neteče $R_3$ proud, takže napětí naprázdno je napětí uzlu, které mám z jedné rovnice MUN. Vnitřní odpor: zdroj napětí nahradím zkratem, zdroj proudu rozpojím, zbude $R_1 \parallel R_2$ v sérii s $R_3$.“

> [!trap] Zdroje při určování $R_\text{i}$
> Zdroj napětí → **zkrat** (jeho vnitřní odpor je nula), zdroj proudu → **rozpojit** (jeho vnitřní vodivost je nula). Obráceně je to nejčastější chyba. Rezistor v sérii s ideálním zdrojem proudu na $R_\text{i}$ nemá vliv.

## Proměnná zátěž

Náhradní zdroj se spočítá jednou a pro každou zátěž stačí dělič: $U_R = U_\text{i}\,R/(R_\text{i} + R)$. V PC9, př. 4 (COZ4, př. 4.12) je $R_\text{i} = 8{,}333\ \Omega$, $U_\text{i} = 15\ \text{V}$ a pro $R$ = 10, 20, 40 Ω vyjde $U_R$ = 8,18; 10,59; 12,41 V.

## Můstek

![Můstek](images/cv-mustek.svg "Rezistory R1 až R4, diagonála RG, napájení U mezi levým a pravým vrcholem")

Po odpojení diagonály $R_\text{G}$ jsou to dva nezávislé děliče napětí, $U_\text{i}$ je rozdíl jejich výstupních napětí. Se zkratovaným $U$ se levý a pravý vrchol spojí, takže $R_1 \parallel R_2$ a $R_3 \parallel R_4$ jsou v sérii:
$$U_\text{i} = U\left(\frac{R_2}{R_1 + R_2} - \frac{R_4}{R_3 + R_4}\right), \qquad R_\text{i} = \frac{R_1 R_2}{R_1 + R_2} + \frac{R_3 R_4}{R_3 + R_4}$$
V PC9, př. 3 ($U = 30\ \text{V}$, $R_1 = 2$, $R_2 = 4$, $R_3 = 10$, $R_4 = 5\ \text{k}\Omega$) je $U_\text{i} = 30\,(0{,}6667 - 0{,}3333) = 10\ \text{V}$ a $R_\text{i} = 1{,}333 + 3{,}333 = 4{,}667\ \text{k}\Omega$. Galvanometr s odporem 10 až 50 Ω je proti $R_\text{i}$ tak malý, že proud skoro nezmění: 2,14 mA, 2,13 mA, 2,12 mA. Můstek je vyvážený ($U_\text{i} = 0$), když $R_1 R_4 = R_2 R_3$.

## Největší výkon do zátěže

Výkon do zátěže je největší při **výkonovém přizpůsobení** $R_\text{z} = R_\text{i}$:
$$P_\text{max} = \frac{U_\text{i}^2}{4 R_\text{i}}, \qquad \eta = 50\ \%$$
Grafy a měření jsou v kapitole **Úloha 6: Přenos výkonu**.

> [!example] PC9, př. 5 (COZ4, př. 4.13)
> Pro zadaný obvod vyjde $R_\text{i} = 20\ \Omega$ (paralelní $R_1 + R_2$, $R_3$, $R_4$ a k nim v sérii $R_5$) a z MUN $U_\text{i} = 16{,}67\ \text{V}$. Zátěž $R = R_\text{i} = 20\ \Omega$:
> $$I_R = \frac{U_\text{i}}{2R_\text{i}} = \frac{16{,}67}{40} = 0{,}416\,7\ \text{A}, \quad U_R = 0{,}416\,7\cdot20 = 8{,}333\ \text{V}, \quad P_R = \frac{8{,}333^2}{20} = 3{,}472\ \text{W}$$

> [!note] Kdy věta neplatí
> Nahrazovaná část musí být lineární (zátěž lineární být nemusí, viz grafická metoda v kapitole **Cvičení: nelineární obvody**). Superpozice, kterou jde $U_\text{i}$ obvodu s více zdroji spočítat po částech, má vlastní kapitolu **Úloha 5: Princip superpozice**.
