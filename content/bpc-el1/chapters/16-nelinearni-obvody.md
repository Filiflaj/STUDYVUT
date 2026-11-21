+++
title = "Cvičení: nelineární obvody"
topics = ["Nelineární obvody", "Nelineární prvky"]
+++

Obvod s diodou, žárovkou nebo termistorem se nedá řešit superpozicí ani maticí s konstantními vodivostmi. Ve cvičeních (PC10, PC11, COZ5) se charakteristika prvku nejdřív **popíše** (tabulka, interpolace, aproximace) a obvod se pak **řeší** analyticky, linearizací, graficky nebo numericky. Statický a dynamický odpor a měření charakteristik jsou v kapitole **Úloha 8: Lineární a nelineární prvky**.

![Statický a dynamický odpor](images/rs-rd.svg "Statický odpor je sklon spojnice s počátkem, dynamický sklon tečny v pracovním bodě")

$$R_\text{s} = \frac{U_\text{P}}{I_\text{P}}, \qquad R_\text{d} = \left.\frac{\mathrm{d}u}{\mathrm{d}i}\right|_\text{P} = \frac{1}{G_\text{d}}, \qquad G_\text{d} = \left.\frac{\mathrm{d}i}{\mathrm{d}u}\right|_\text{P}$$

## Nelineární prvek vyrábí harmonické

Když na prvek s kvadratickou charakteristikou $i = 0{,}1\,u^2$ působí $u = 3 + 2\sin\omega t$ (sbírka 10.1), vyjde s $\sin^2\alpha = \tfrac12(1 - \cos2\alpha)$:
$$i = 0{,}1\,(9 + 12\sin\omega t + 4\sin^2\omega t) = 1{,}1 + 1{,}2\sin\omega t - 0{,}2\cos2\omega t\ \text{A}$$
Proud má stejnosměrnou složku 1,1 A, první harmonickou 1,2 A a navíc **druhou harmonickou** 0,2 A, kterou napětí vůbec nemělo. Lineární obvod nové kmitočty nevytváří, nelineární ano (zkreslení, usměrnění, směšování).

## Popis charakteristiky

### Interpolace polynomem

Polynom 2. stupně $i_\text{a} = a_0 + a_1 u + a_2 u^2$ se proloží **přesně** třemi body. Dosazením bodů vznikne soustava s Vandermondovou maticí:
$$\begin{bmatrix} 1 & u_1 & u_1^2 \\ 1 & u_2 & u_2^2 \\ 1 & u_3 & u_3^2 \end{bmatrix}\begin{bmatrix} a_0 \\ a_1 \\ a_2 \end{bmatrix} = \begin{bmatrix} i_1 \\ i_2 \\ i_3 \end{bmatrix}$$
Dynamická vodivost pak je derivace polynomu: $G_\text{d} = a_1 + 2a_2 u$.

> [!example] PC10, př. 2 (COZ5, př. 5.2): interpolace
> Body $(0;\ 0)$, $(0{,}15\ \text{V};\ 0{,}2\ \text{A})$, $(0{,}3\ \text{V};\ 0{,}9\ \text{A})$. Bod v počátku dá $a_0 = 0$ a soustava se zmenší na 2×2:
> $$\begin{bmatrix} 0{,}15 & 0{,}15^2 \\ 0{,}3 & 0{,}3^2 \end{bmatrix}\begin{bmatrix} a_1 \\ a_2 \end{bmatrix} = \begin{bmatrix} 0{,}2 \\ 0{,}9 \end{bmatrix}$$
> $$\Delta = 0{,}15\cdot0{,}09 - 0{,}0225\cdot0{,}3 = 0{,}0135 - 0{,}006\,75 = 0{,}006\,75$$
> $$\Delta_1 = 0{,}2\cdot0{,}09 - 0{,}0225\cdot0{,}9 = -0{,}002\,25, \qquad \Delta_2 = 0{,}15\cdot0{,}9 - 0{,}3\cdot0{,}2 = 0{,}075$$
> $$a_1 = \frac{-0{,}002\,25}{0{,}006\,75} = -0{,}333, \qquad a_2 = \frac{0{,}075}{0{,}006\,75} = 11{,}11, \qquad i_\text{a} = -0{,}333\,u + 11{,}11\,u^2$$
> Proud pro 0,2 V: $i_\text{a}(0{,}2) = -0{,}333\cdot0{,}2 + 11{,}11\cdot0{,}04 = 0{,}378\ \text{A}$. Dynamický odpor v 0,15 V:
> $$R_\text{d} = \frac{1}{a_1 + 2a_2 u} = \frac{1}{-0{,}333 + 22{,}22\cdot0{,}15} = \frac{1}{3{,}0} = 0{,}333\ \Omega$$
>
> **Jak to říct, když se zeptají:** „Polynom má procházet třemi naměřenými body, takže dosadím body a dostanu tři rovnice pro koeficienty. Díky bodu v počátku je $a_0$ nula a stačí matice 2×2. Dynamický odpor je převrácená hodnota derivace proudu podle napětí.“

Mimo interval naměřených bodů jde o **extrapolaci** (COZ5, př. 5.3), která je méně spolehlivá.

### Aproximace metodou nejmenších čtverců

Přímka $y_\text{a} = a_0 + a_1 x$ se proloží **mezi** $m$ body tak, aby byl součet čtverců odchylek nejmenší. Z nulových parciálních derivací vyjdou normální rovnice:
$$\begin{bmatrix} m & \sum x_j \\ \sum x_j & \sum x_j^2 \end{bmatrix}\begin{bmatrix} a_0 \\ a_1 \end{bmatrix} = \begin{bmatrix} \sum y_j \\ \sum x_j y_j \end{bmatrix}$$
Součty se nejpohodlněji spočítají v tabulce (sloupce $x_j$, $y_j$, $x_j^2$, $x_j y_j$). Pro body $x$ = 0, 1, 3, 5, 6 a $y$ = 5, 3, 3, 2, 1 (sbírka 10.2) je $\sum x = 15$, $\sum y = 14$, $\sum x^2 = 71$, $\sum xy = 28$, $\Delta = 5\cdot71 - 15^2 = 130$ a
$$a_0 = \frac{14\cdot71 - 15\cdot28}{130} = \frac{574}{130} = 4{,}415, \qquad a_1 = \frac{5\cdot28 - 15\cdot14}{130} = \frac{-70}{130} = -0{,}538$$

## Řešení obvodu

### Analyticky

Je-li charakteristika jednoduchá rovnice, dosadí se do 2. KZ. Pro $U_\text{n} = 3I^2 + 2I$ v sérii s $R = 5\ \Omega$ na $U = 10\ \text{V}$ (sbírka 10.4): $3I^2 + 7I - 10 = 0$, kořeny 1 A a −3,33 A, fyzikální smysl má jen $I = 1\ \text{A}$.

### Linearizací

V okolí pracovního bodu se prvek nahradí přímkou, tedy zdrojem napětí $U_\text{d}$ se sériovým $R_\text{d}$ (nebo zdrojem proudu $U_\text{d}/R_\text{d}$ s paralelní $G_\text{d}$). Pak je obvod lineární a řeší se MUN. Platí jen, dokud pracovní bod zůstane na linearizovaném úseku.

![Stabilizátor se Zenerovou diodou](images/cv-stabilizator.svg "Zdroj U, rezistor R1, Zenerova dioda v závěrném směru a zátěž R2; dioda nahrazená zdrojem Ud se sériovým Rd")

> [!example] Sbírka 10.5: linearizovaný stabilizátor
> $U = 10\ \text{V}$, $R_1 = 100\ \Omega$, $R_2 = 250\ \Omega$, model diody $U_\text{d} = 5{,}7\ \text{V}$, $R_\text{d} = 2\ \Omega$. Oba zdroje napětí převedu na zdroje proudu: $I = U/R_1 = 0{,}1\ \text{A}$, $I_\text{d} = U_\text{d}/R_\text{d} = 2{,}85\ \text{A}$; $G_1 = 0{,}01$, $G_2 = 0{,}004$, $G_\text{d} = 0{,}5\ \text{S}$. Jediná rovnice MUN:
> $$(G_1 + G_2 + G_\text{d})\,U_2 = I + I_\text{d} \quad\Rightarrow\quad 0{,}514\,U_2 = 2{,}95 \quad\Rightarrow\quad U_2 = 5{,}739\ \text{V}$$
> **Jak to říct, když se zeptají:** „Diodu v okolí pracovního bodu nahrazuji přímkou, tedy zdrojem 5,7 V s odporem 2 Ω. Obvod je pak lineární a stačí jedna rovnice MUN pro výstupní uzel.“

### Graficky: metoda zatěžovací přímky

Lineární část obvodu se nahradí Théveninovým zdrojem $U_0$, $R_\text{i}$. Jeho charakteristika (zatěžovací přímka) prochází body $(U_0;\ 0)$ a $(0;\ I_\text{k} = U_0/R_\text{i})$. Průsečík s charakteristikou nelineárního prvku je **pracovní bod** P.

![Zatěžovací přímka](images/cv-zatezovaci-primka.svg "Přímka náhradního zdroje od U0 do Ik protíná charakteristiku prvku v pracovním bodě P")

V PC10, př. 3 (COZ5, př. 5.4) je $U = 12\ \text{V}$, $R_1 = 33\ \Omega$, $R_2 = 470\ \Omega$: $U_\text{i} = 12\cdot470/503 = 11{,}2\ \text{V}$, $R_\text{i} = 33\parallel470 = 30{,}83\ \Omega$, $I_\text{k} = 0{,}363\ \text{A}$. Z grafu je pracovní bod (9,15 V; 65 mA) a ztrátový výkon diody $P = 9{,}15\cdot0{,}065 = 0{,}595\ \text{W}$. Posun přímky ukáže, jak se pracovní bod změní se vstupním napětím.

### Numericky: Newtonova metoda

Rovnice obvodu se upraví na tvar $f(x) = 0$ a odhad se zpřesňuje tečnou:
$$x_{(k+1)} = x_{(k)} + \varepsilon_{(k)}, \qquad \varepsilon_{(k)} = -\frac{f(x_{(k)})}{f'(x_{(k)})}$$
Iterace končí, když je oprava $|\varepsilon|$ menší než požadovaná chyba (obvykle 1 mV). Metoda konverguje rychle, ale potřebuje rozumný počáteční odhad.

![Newtonova metoda](images/cv-newton.svg "Tečna v bodě x(k) protne osu v novém odhadu x(k+1)")

> [!example] Sbírka 10.10: Thévenin a Newton
> $U = 45\ \text{V}$, $R_1 = 200\ \Omega$, $R_2 = 50\ \Omega$ (dělič), prvek s $i = 0{,}025u^3 - 0{,}05u$ paralelně k $R_2$, počáteční odhad 2,5 V.
>
> Lineární část: $U_\text{i} = 45\cdot50/250 = 9\ \text{V}$, $R_\text{i} = 200\cdot50/250 = 40\ \Omega$. Podle 2. KZ $u + R_\text{i}\,i - U_\text{i} = 0$:
> $$u + 40\,(0{,}025u^3 - 0{,}05u) - 9 = 0 \quad\Rightarrow\quad f(u) = u^3 - u - 9, \qquad f'(u) = 3u^2 - 1$$
>
> | $k$ | $u_{(k)}$ (V) | $f(u_{(k)})$ | $f'(u_{(k)})$ | $\varepsilon_{(k)}$ (V) |
> |---|---|---|---|---|
> | 0 | 2,5 | 4,125 | 17,75 | −0,232 39 |
> | 1 | 2,267 61 | 0,392 5 | 14,426 | −0,027 21 |
> | 2 | 2,240 40 | 0,005 0 | 14,058 | −0,000 36 |
>
> Oprava je pod 1 mV, takže $u = 2{,}2404\ \text{V}$ a $i = 0{,}025\cdot2{,}2404^3 - 0{,}05\cdot2{,}2404 = 0{,}169\,1\ \text{A}$.
>
> **Jak to říct, když se zeptají:** „Lineární část jsem nahradil Théveninovým zdrojem, dosadil charakteristiku do 2. KZ a dostal kubickou rovnici. Tu řeším Newtonovou metodou: nový odhad je starý minus funkce lomeno derivací. Skončím, když je oprava menší než 1 mV.“

**Metoda půlení intervalu** (bisekce) je pomalejší, ale vždy konverguje: v intervalu $\langle a; b\rangle$, kde $f$ mění znaménko, se vezme střed, ponechá se polovina se změnou znaménka a opakuje se, dokud není polovina intervalu menší než požadovaná chyba. Pro diodu $I_\text{d} = 2\cdot10^{-12}\,\mathrm{e}^{38U_\text{d}}$ s $R = 150\ \Omega$ na 5 V (sbírka 10.11) dá sedm kroků $U_\text{d} = (616{,}4 \pm 0{,}8)\ \text{mV}$. Iterace se pohodlně počítají v tabulce Excelu (PC11).

> [!trap] Na co si dát pozor
> - Proud v charakteristice bývá v mA: dynamický odpor z derivace pak vychází v kΩ, ne v Ω.
> - Ze dvou kořenů kvadratické rovnice platí jen ten fyzikálně možný.
> - Do $f(u)$ patří celá rovnice obvodu převedená na jednu stranu, ne jen charakteristika prvku.
