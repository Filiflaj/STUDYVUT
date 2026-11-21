+++
title = "Nelineární prvky a obvody"
topics = ["Nelineární obvody"]
+++

Nelineární prvek nemá konstantní odpor: proud není úměrný napětí. Typicky dioda, žárovka, varistor, termistor nebo cívka s železným jádrem. Jeho ampérvoltová (AV) charakteristika bývá dána **tabulkou naměřených bodů** nebo grafem. V takovém obvodu neplatí superpozice a soustava rovnic přestane být lineární. Procvičuje se ve **VC4**.

## Statický a dynamický odpor

- **Statický odpor** v bodě $[u; i]$: $R = u/i$ (sklon spojnice bodu s počátkem).
- **Dynamický (diferenciální) odpor**: převrácená hodnota derivace charakteristiky, $R_\text{d} = 1/G_\text{d}$, kde $G_\text{d} = \dfrac{\text{d}i}{\text{d}u}$. Popisuje, jak prvek reaguje na malé změny kolem pracovního bodu.

## Interpolace charakteristiky polynomem

Aby se s charakteristikou dalo počítat, proloží se naměřenými body polynom. Pro tři body stačí kvadratický:

$$i(u) = a_0 + a_1 u + a_2 u^2$$

Dosazením bodů vznikne soustava lineárních rovnic pro $a_0, a_1, a_2$ (Vandermondova matice). Když charakteristika prochází **počátkem** $[0; 0]$, je $a_0 = 0$ a soustava se zmenší na 2 × 2.

> [!example] Příklad z prezentace
> Naměřeno: $[0; 0]$, $[0{,}15\ \text{V}; 0{,}2\ \text{A}]$, $[0{,}3\ \text{V}; 0{,}9\ \text{A}]$. Díky počátku $a_0 = 0$ a zbývá
> $$\begin{bmatrix} 0{,}15 & 0{,}15^2 \\ 0{,}3 & 0{,}3^2 \end{bmatrix}\cdot\begin{bmatrix} a_1 \\ a_2 \end{bmatrix} = \begin{bmatrix} 0{,}2 \\ 0{,}9 \end{bmatrix}$$
> $$\Delta = 0{,}15\cdot0{,}09 - 0{,}0225\cdot0{,}3 = 0{,}00675, \quad \Delta_1 = 0{,}2\cdot0{,}09 - 0{,}0225\cdot0{,}9 = -0{,}00225, \quad \Delta_2 = 0{,}15\cdot0{,}9 - 0{,}3\cdot0{,}2 = 0{,}075$$
> $$a_1 = \frac{-0{,}00225}{0{,}00675} = -0{,}333, \qquad a_2 = \frac{0{,}075}{0{,}00675} = 11{,}11, \qquad i(u) = -0{,}333\,u + 11{,}11\,u^2$$
> **Proud pro $u$ = 0,2 V:** $i = -0{,}333\cdot0{,}2 + 11{,}11\cdot0{,}04 = 0{,}378$ A.
>
> **Dynamický odpor pro $u$ = 0,15 V:** $G_\text{d} = a_1 + 2a_2 u = -0{,}333 + 22{,}22\cdot0{,}15 = 3{,}0$ S, takže $R_\text{d} = 1/3 = 0{,}333\ \Omega$.

## Tři způsoby řešení

Úloha: zdroj $U$, rezistor $R$ a nelineární prvek v sérii. Hledá se proud.

1. **Analyticky:** dosadit charakteristiku do Kirchhoffova zákona a vyřešit rovnici. Pro $u_n = k i^2$ vyjde kvadratická rovnice $k i^2 + R i = U$.
2. **Graficky (metoda zatěžovací přímky):** do jednoho grafu se nakreslí AV charakteristika prvku a přímka zbytku obvodu, $i = (U - u)/R$, od bodu $[U; 0]$ do $[0; U/R]$. **Průsečík je pracovní bod.**
3. **Numericky:** rovnice se upraví na tvar $f(x) = 0$ a kořen se najde iterací, nejčastěji **Newtonovou metodou**.

![Zatěžovací přímka](images/zatezovaci-primka.svg "Charakteristika Rn (černě) a přímka náhradního zdroje Ui = 4,8 V, Ri = 12 Ω (modře) z VC4; průsečík je pracovní bod asi 3,10 V a 0,142 A")

### Newtonova iterační metoda

Kořen rovnice $f(x) = 0$ se hledá postupným zpřesňováním odhadu:

$$x^{(k+1)} = x^{(k)} - \frac{f\big(x^{(k)}\big)}{f'\big(x^{(k)}\big)}$$

Geometricky: v bodě odhadu se vede tečna a její průsečík s osou $x$ je nový odhad. Kořenů může být víc, proto se nejdřív zvolí interval s jediným kořenem (separace kořenů) a rozumný počáteční odhad. Iteruje se, dokud se odhad přestane měnit (nebo $f$ je skoro nula).

> [!example] Příklad z prezentace
> $U$ = 10 V, $R$ = 10 Ω, nelineární prvek $u_n = 20\,i^2$. Z 2. KZ $20 i^2 + 10 i - 10 = 0$, tedy
> $$f(i) = 20 i^2 + 10 i - 10, \qquad f'(i) = 40 i + 10$$
> Odhad $i^{(0)} = 1$ A:
>
> | $k$ | $i^{(k)}$ (A) | oprava $-f/f'$ |
> |---|---|---|
> | 0 | 1 | −0,4 |
> | 1 | 0,6 | −0,0941 |
> | 2 | 0,5059 | −0,0059 |
> | 3 | 0,5000 | −2,3 · 10⁻⁵ |
>
> Proud je $I$ = 0,5 A (kontrola: $20\cdot0{,}25 + 5 - 10 = 0$).

## Nelineární prvek a náhradní zdroj

Když je v obvodu **jen jeden** nelineární prvek, celý zbytek obvodu je lineární a dá se nahradit Théveninovým zdrojem $U_\text{i}$, $R_\text{i}$. Pak stačí jediná rovnice:

$$\frac{U_\text{i} - u}{R_\text{i}} = i(u)$$

což je přesně průsečík zatěžovací přímky s charakteristikou.

## Vzorový příklad: VC4

![Obvod VC4](images/vc4-obvod.svg "Zdroj U, rezistor R1, paralelně R2 a nelineární Rn; hledá se U2")

$U$ = 12 V, $R_1$ = 30 Ω, $R_2$ = 20 Ω. Charakteristika $R_\text{n}$ změřená ve třech bodech: $[0; 0]$, $[4\ \text{V}; 0{,}23\ \text{A}]$, $[6\ \text{V}; 0{,}5\ \text{A}]$.

> [!example] 1. Interpolace
> Počátek dává $a_0 = 0$. Pro $i = a_1 u + a_2 u^2$:
> $$\begin{bmatrix} 4 & 16 \\ 6 & 36 \end{bmatrix}\cdot\begin{bmatrix} a_1 \\ a_2 \end{bmatrix} = \begin{bmatrix} 0{,}23 \\ 0{,}5 \end{bmatrix}$$
> $$\Delta = 4\cdot36 - 16\cdot6 = 48, \quad \Delta_1 = 0{,}23\cdot36 - 16\cdot0{,}5 = 0{,}28, \quad \Delta_2 = 4\cdot0{,}5 - 0{,}23\cdot6 = 0{,}62$$
> $$a_1 = \frac{0{,}28}{48} = 0{,}005833, \qquad a_2 = \frac{0{,}62}{48} = 0{,}01292, \qquad i(u) = 0{,}01292\,u^2 + 0{,}005833\,u$$
> Ověření: $i(4) = 0{,}2067 + 0{,}0233 = 0{,}23$ A, $i(6) = 0{,}4651 + 0{,}0350 = 0{,}50$ A.

> [!example] 2. Náhradní zdroj a kvadratická rovnice
> Vzhledem ke svorkám $R_\text{n}$ je zbytek dělič $R_1$, $R_2$:
> $$R_\text{i} = \frac{R_1 R_2}{R_1 + R_2} = \frac{30\cdot20}{50} = 12\ \Omega, \qquad U_\text{i} = U\,\frac{R_2}{R_1 + R_2} = 12\cdot\frac{20}{50} = 4{,}8\ \text{V}$$
> Proud z náhradního zdroje teče celý do $R_\text{n}$:
> $$\frac{4{,}8 - U_2}{12} = 0{,}01292\,U_2^2 + 0{,}005833\,U_2$$
> Vynásobit dvanácti a převést na jednu stranu ($12\cdot0{,}01292 = 0{,}15504$, $12\cdot0{,}005833 = 0{,}069996$):
> $$0{,}15504\,U_2^2 + 1{,}069996\,U_2 - 4{,}8 = 0$$
> $$D = 1{,}069996^2 + 4\cdot0{,}15504\cdot4{,}8 = 1{,}144891 + 2{,}976768 = 4{,}121659, \qquad \sqrt{D} = 2{,}030187$$
> $$U_2 = \frac{-1{,}069996 + 2{,}030187}{2\cdot0{,}15504} = \frac{0{,}960191}{0{,}31008} = 3{,}0966\ \text{V}$$
> Druhý kořen ($-9{,}998$ V) je záporný a fyzikálně nemá smysl: zdroj s kladným $U_\text{i}$ nemůže na pasivním prvku vytvořit záporné napětí.

> [!example] 3. Totéž Newtonovou metodou
> $$f(U_2) = 0{,}15504\,U_2^2 + 1{,}069996\,U_2 - 4{,}8, \qquad f'(U_2) = 0{,}31008\,U_2 + 1{,}069996$$
> Počáteční odhad: napětí naprázdno $U^{(0)} = 4{,}8$ V (víc než $U_\text{i}$ na $R_\text{n}$ být nemůže).
>
> | $k$ | $U^{(k)}$ (V) | $f$ | $f'$ | oprava |
> |---|---|---|---|---|
> | 0 | 4,8 | 3,9081 | 2,5584 | −1,5276 |
> | 1 | 3,2724 | 0,3618 | 2,0847 | −0,1735 |
> | 2 | 3,0989 | 0,0047 | 2,0309 | −0,0023 |
> | 3 | 3,096592 | 0,000001 | 2,0302 | −0,000001 |
> | 4 | 3,096591 | | | |
>
> $U_2$ = 3,096591 V; s odhadem 3 V stačí dvě iterace.
>
> **Jak to říct, když se zeptají:** „Zbytek obvodu kromě $R_\text{n}$ je lineární, tak ho nahradím Théveninem: $R_1 \parallel R_2$ a napětí děliče. Proud náhradního zdroje teče do nelineárního prvku, takže dám do rovnosti proud z přímky a proud z polynomu. Vyjde kvadratická rovnice; záporný kořen zahodím. Newton jen hledá tentýž kořen iteračně: odhad mínus funkce děleno derivací.“

> [!note] Výsledek v zadání
> Zadání uvádí $U_2$ = 3,0965 V a Newtonem 3,096585 V. S koeficienty zaokrouhlenými tak, jak jsou v zadání (0,01292 a 0,005833), vyjde 3,09659 V; zadání číslo usekává. S přesnými zlomky ($a_1 = 7/1200$, $a_2 = 31/2400$) by rovnice byla $0{,}155\,U_2^2 + 1{,}07\,U_2 - 4{,}8 = 0$ s kořenem 3,0968 V. Rozdíly jsou v desetitisícinách voltu a na postupu nic nemění.

Proud nelineárním prvkem v pracovním bodě: $I_\text{n} = (4{,}8 - 3{,}0966)/12 = 0{,}142$ A.
