+++
title = "Úloha 7: Magnetické pole ve vzduchové mezeře"
topics = ["Magnetický obvod"]
+++

Feritové jádro (toroid) se vzduchovou mezerou budí cívka. Hallovou sondou se měří indukce v mezeře v závislosti na budicím proudu a pak napříč mezerou. Hodnoty: vnější poloměr $a$ = 40 mm, vnitřní $b$ = 25 mm, výška $c$ = 22 mm, mezera $\ell_\text{v}$ = 1,7 mm, $N_\text{z}$ = 232 závitů, $\mu_\text{rf}$ feritu podle tabulky (2240 až 3400 podle proudu).

![Magnetický obvod a jeho ekvivalentní obvod](images/magneticky-obvod.svg "Toroid s mezerou a cívkou; vpravo ekvivalentní obvod: magnetické napětí cívky, magnetický odpor feritu a mezery")

## Magnetický obvod

Při zanedbání rozptylu je tok $\Phi$ v celém obvodu stejný, a protože je stejný i průřez $S$, je v jádře i v mezeře **stejná indukce** $B = \Phi/S$. Intenzita se liší, ferit má $\mu_\text{rf}$ v tisících, vzduch $\mu_\text{r} \approx 1$:

$$B = \mu_0\,\mu_\text{rf}\,H_\text{f} = \mu_0\,H_\text{v}$$

Cívka je zdrojem **magnetického napětí**, které se rozdělí na jádro a mezeru (obdoba 2. KZ):

$$U_\text{m} = N_\text{z} I_\text{b} = U_\text{mf} + U_\text{mv} = H_\text{f}\,\ell_\text{f} + H_\text{v}\,\ell_\text{v}$$

**Hopkinsonův zákon** je obdoba Ohmova zákona (magnetické napětí ↔ napětí, tok ↔ proud):

$$U_\text{m} = \Phi R_\text{m}, \qquad R_\text{mf} = \frac{\ell_\text{f}}{\mu_0\,\mu_\text{rf}\,S}, \qquad R_\text{mv} = \frac{\ell_\text{v}}{\mu_0\,S}$$

Dosazením intenzit do rovnice pro $U_\text{m}$ vyjde indukce

$$B = \mu_0\,\frac{N_\text{z} I_\text{b}}{\dfrac{\ell_\text{f}}{\mu_\text{rf}} + \ell_\text{v}}$$

Ferit se chová jako mezera délky $\ell_\text{f}/\mu_\text{rf}$, tady jen 0,06 až 0,09 mm. Když $\ell_\text{v} \gg \ell_\text{f}/\mu_\text{rf}$, lze odpor feritu zanedbat:

$$\tilde{B} \cong \mu_0\,\frac{N_\text{z} I_\text{b}}{\ell_\text{v}}$$

> [!key] Mezera obvod linearizuje
> Ferit je nelineární ($\mu_\text{rf}$ závisí na proudu), ale magnetický odpor vzduchové mezery je asi 24× větší než odpor feritu. Na mezeře je asi 96 % magnetického napětí, takže $B$ je skoro úměrné proudu a $\tilde{B}$ je jen o 3 až 5 % větší než $B$.

Měří se pro **oba směry proudu** a bere se průměr, aby se vyloučila zbytková magnetizace jádra a posun nuly sondy:

$$B_\text{m} = \frac{B_\text{m34} + B_\text{m43}}{2}$$

## Domácí příprava

> [!example] Geometrie a indukce
> $a$ a $b$ jsou poloměry, střední siločára má poloměr $(a+b)/2$ = 32,5 mm a od jejího obvodu se odečte mezera:
> $$\ell_\text{f} = 2\pi\,\frac{a+b}{2} - \ell_\text{v} = \pi\,(40 + 25) - 1{,}7 = 204{,}20 - 1{,}7 = 202{,}50\ \text{mm}$$
> Průřez je obdélník $(a - b) \times c$:
> $$S = (40 - 25)\cdot 22 = 330\ \text{mm}^2 = 3{,}30\cdot10^{-4}\ \text{m}^2$$
> Opakující se konstanta: $\mu_0 N_\text{z} = 4\pi\cdot10^{-7}\cdot 232 = 2{,}9154\cdot10^{-4}$ H/m. Pro $I_\text{b}$ = 0,1 A ($\mu_\text{rf}$ = 2240):
> $$\frac{\ell_\text{f}}{\mu_\text{rf}} = \frac{202{,}50}{2240} = 0{,}0904\ \text{mm}, \qquad B = 2{,}9154\cdot10^{-4}\cdot\frac{0{,}1}{1{,}7904\cdot10^{-3}} = 16{,}3\ \text{mT}$$
> $$\tilde{B} = 2{,}9154\cdot10^{-4}\cdot\frac{0{,}1}{1{,}7\cdot10^{-3}} = 17{,}1\ \text{mT}$$
> $\tilde{B}$ je přímo úměrné proudu: $\tilde{B} = 171{,}5\ \text{mT/A}\cdot I_\text{b}$.
>
> **Jak to říct, když se zeptají:** „Ze součtu magnetických napětí na feritu a na mezeře vyjádřím $B$. Ferit se chová jako mezera $\ell_\text{f}/\mu_\text{rf}$, asi 0,09 mm, proti 1,7 mm skutečné mezery je to málo, proto je $\tilde{B}$ jen o pár procent větší.“

> [!example] Ekvivalentní obvod pro $I_\text{b}$ = 0,6 A ($\mu_\text{rf}$ = 2870, $B$ = 98,79 mT)
> $$R_\text{mf} = \frac{0{,}2025}{4\pi\cdot10^{-7}\cdot 2870\cdot 3{,}30\cdot10^{-4}} = 1{,}701\cdot10^{5}\ \text{H}^{-1}, \qquad R_\text{mv} = \frac{1{,}7\cdot10^{-3}}{4\pi\cdot10^{-7}\cdot 3{,}30\cdot10^{-4}} = 4{,}099\cdot10^{6}\ \text{H}^{-1}$$
> $$U_\text{m} = 232\cdot 0{,}6 = 139{,}2\ \text{A}, \qquad \Phi = B S = 98{,}79\cdot10^{-3}\cdot 3{,}30\cdot10^{-4} = 3{,}260\cdot10^{-5}\ \text{Wb}$$
> $$U_\text{mf} = \Phi R_\text{mf} = 5{,}55\ \text{A}, \qquad U_\text{mv} = \Phi R_\text{mv} = 133{,}6\ \text{A}$$
> Kontrola: $5{,}55 + 133{,}6 = 139{,}2$ A $= U_\text{m}$. Na mezeře je 96 % magnetického napětí, přestože tvoří jen 1,7 mm z 204 mm.

## Měření

1. Zapnout napájení přípravku 24 V; jezdec se sondou se zkalibruje a vrátí na $x$ = 0.
2. Regulovatelný zdroj v režimu konstantního proudu, $I_\text{b}$ = 0,1 až 1,0 A po 0,1 A, zapsat $B_\text{m34}$.
3. Stáhnout proud, **prohodit vodiče na svorkách 3 a 4**, změřit $B_\text{m43}$.
4. Při $I_\text{b}$ = 0,6 A posouvat sondu po 1 mm z $x$ = −15 na +15 mm a zapisovat $B_\text{m}$.

Do sešitu: $\ell_\text{f}$, řádky $B$ a $\tilde{B}$ (doma), $B_\text{m}$ jako průměr (po labu), ekvivalentní obvod pro 0,6 A, graf $B(I_\text{b})$ se třemi průběhy ($B_\text{m}$ body, $B$ a $\tilde{B}$ čarami) a graf $B_\text{m}(x)$ se svislými čarami hran jádra v $x = \pm 7{,}5$ mm (polovina šířky $a - b$ = 15 mm).

> [!example] Příklad z měření
> Při 0,6 A: $B_\text{m} = (102{,}9 + 103{,}3)/2 = 103{,}1$ mT proti vypočteným 98,8 mT, tedy $\delta = (103{,}1 - 98{,}8)/98{,}8\cdot 100 = +4{,}4\ \%$. V celém rozsahu 0,1 až 0,9 A ležely body na přímce se sklonem asi 166 mT/A (stejným jako u $B$), jen posunuté asi o 3 mT nahoru; proto byla relativní odchylka při malých proudech velká. Při 1,0 A indukce skoro nevzrostla (158,7 mT), nejspíš konec rozsahu sondy.
>
> Napříč mezerou bylo pole homogenní pro $x$ = −6 až +6 mm (102 až 105 mT), na hranách jádra kleslo asi na 80 % a mimo jádro rychle ubývalo.

![Indukce v závislosti na poloze sondy](images/b-x-mereni.svg "Příklad z měření: Bm(x) při Ib = 0,6 A, čárkovaně hrany jádra")

> [!trap] Na co si dát pozor
> - $a$, $b$ jsou poloměry, ne průměry: $\ell_\text{f} = \pi(a + b) - \ell_\text{v}$.
> - Jednotky: $\ell$ v metrech, $S$ v m², výsledek v T (×1000 = mT).
> - $\ell_\text{f}/\mu_\text{rf}$ zaokrouhlit na 4 desetinná místa v mm, jinak se rozdíl $B$ a $\tilde{B}$ ztratí.
> - Průměr $B_\text{m}$ se zaokrouhluje na stejné místo jako měřené hodnoty (36,05 → 36,1).

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Proč je $B$ v jádře i v mezeře stejná?** Stejný tok i průřez (bez rozptylu), $B = \Phi/S$.
> - **Proč mezera obvod linearizuje?** Její magnetický odpor je mnohem větší než odpor feritu, nelinearita feritu se skoro neprojeví.
> - **Co říká Hopkinsonův zákon?** $U_\text{m} = \Phi R_\text{m}$, obdoba Ohmova zákona.
> - **Proč se měří pro oba směry proudu?** Kvůli zbytkové magnetizaci jádra a posunu nuly sondy.
> - **Proč $\mu_\text{rf}$ roste s proudem?** Ferit je nelineární, permeabilita závisí na intenzitě pole (magnetizační křivka).
> - **Jak funguje Hallova sonda?** Pole vychyluje nosiče náboje v destičce, kterou teče proud; napříč vznikne Hallovo napětí úměrné $B$.
> - **Proč pole u okraje mezery klesá?** Rozptyl: siločáry se u okraje vyboulí ven do většího prostoru.
