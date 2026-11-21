+++
title = "Úloha 25: Měrný náboj elektronu (magnetron)"
topics = ["Úloha 25: měrný náboj elektronu"]
+++

**Úkol:** určit měrný náboj elektronu $e/m$ magnetronovou metodou a porovnat ho s tabulkovou hodnotou. Návod pro porovnání bere $e = 1{,}6\cdot10^{-19}\ \mathrm{C}$ a $m = 9{,}1\cdot10^{-31}\ \mathrm{kg}$, tedy $e/m \doteq 1{,}76\cdot10^{11}\ \mathrm{C\,kg^{-1}}$.

## Síly na náboj v poli {#sily}

$$F_e = eE \qquad \vec F_m = q\,\vec v\times\vec B,\quad F_m = evB\ \ (\vec v\perp\vec B)$$

- Elektrická síla na elektron míří **proti** směru intenzity, tedy od katody k anodě (elektron je urychlován k anodě).
- Magnetická (Lorentzova) síla je **kolmá na rychlost**: mění jen směr, ne velikost rychlosti, a nekoná práci. Právě ona vychyluje elektrony z radiálního směru.
- V homogenním poli s $\vec v \perp \vec B$ je magnetická síla dostředivá a částice obíhá po **kružnici**:
$$evB = \frac{mv^2}{r} \quad\Rightarrow\quad r = \frac{mv}{eB} \qquad T = \frac{2\pi m}{eB}$$
Doba oběhu nezávisí na rychlosti. Má-li rychlost i složku podél $\vec B$, je dráha šroubovice.
- Rychlost z urychlovacího napětí: $\tfrac12mv^2 = eU$, $v = \sqrt{2eU/m}$.

**Pole uvnitř dlouhé cívky** (zanedbáme okraje): $N$ závitů na délce $L$, proud $I_\mathrm{C}$:
$$B = \mu_0\frac{N}{L}I_\mathrm{C} = K\,I_\mathrm{C} \qquad \mu_0 = 4\pi\cdot10^{-7}\ \mathrm{H\,m^{-1}}$$

## Magnetron {#magnetron}

Vakuová dioda s válcovou katodou (poloměr $R_\mathrm{k}$) a souosou válcovou anodou ($R_\mathrm{a}$) uvnitř cívky. Elektrické pole mezi elektrodami je radiální, magnetické pole cívky míří podél osy, obě pole jsou na sebe kolmá.

![Magnetron](images/lab-25-magnetron.svg "Dráhy elektronu (schematicky) a pokles anodového proudu s indukcí")

- Bez pole letí elektrony radiálně na anodu. Se slabým polem se dráhy zakřiví, ale na anodu stále dopadají.
- Při **kritické indukci $B_0$** se dráha anody jen dotkne. Pro větší $B$ se elektrony vracejí ke katodě a **anodový proud klesá**.
- Pokles není skokový: elektrony vyletují z katody s nenulovými a různými rychlostmi (Boltzmannovo rozdělení) a geometrie není ideální. Podle návodu odpovídá $B_0$ **inflexnímu bodu sestupné větve** $I_\mathrm{A}(I_\mathrm{C})$.

Ze zachování momentu hybnosti a energie ($\tfrac12mv^2 = eU_\mathrm{a}$) pro dráhu, která se dotkne anody, plyne:

$$\frac{e}{m} = \frac{8\,U_\mathrm{a}R_\mathrm{a}^2}{(R_\mathrm{a}^2 - R_\mathrm{k}^2)^2\,B_0^2}$$

Vztah platí za zjednodušujících předpokladů (nekonečně dlouhé souosé elektrody, nulová počáteční rychlost).

## Měření {#mereni}

1. Po zapnutí počkat asi 11 minut, než se katoda nažhaví a anodový proud ustálí.
2. **Kalibrace cívky:** režim magnetometru, sonda ve středu cívky, 5–10 bodů $B(I_\mathrm{C})$. Lineární regrese dá konstantu $K$ ($B = K I_\mathrm{C}$).
3. Při konstantním $U_\mathrm{a}$ (8–12 V) zvyšovat $I_\mathrm{C}$ a odečítat $I_\mathrm{A}$; v oblasti poklesu po malých krocích. Celé pro tři anodová napětí.
4. Rozměry elektrod z návodu ($R_\mathrm{k}$, $R_\mathrm{a}$).

## Zpracování {#zpracovani}

1. Z kalibrace $K$ a jeho chyba.
2. Z každé charakteristiky $I_\mathrm{A}(I_\mathrm{C})$ kritický proud $I_0$ v inflexním bodě sestupné větve (graficky nebo proložením vhodné funkce), $B_0 = K I_0$.
3. $e/m$ pro každé $U_\mathrm{a}$. Relativní chyba podle šíření chyb: $\delta_\mathrm{r}(e/m) = \delta_\mathrm{r}(U_\mathrm{a}) + 2\,\delta_\mathrm{r}(B_0)$.
4. Když se výsledky pro různá napětí znatelně liší, **neprůměrují se**; uvede se, které napětí dalo nejpřesnější výsledek, a porovná se s tabulkovou hodnotou.

> [!trap] Časté chyby
> - $R_\mathrm{a}$, $R_\mathrm{k}$ v mm místo v m; $B$ v mT místo v T.
> - $B_0$ je ve vzorci na druhou: chyba určení $I_0$ se do výsledku promítne dvakrát.
> - V testových příkladech bývají zadané i veličiny, které k výpočtu nepotřebujete (např. délka cívky, když je zadané $B$).

> [!example] Příklad z měření
> Kalibrace cívky dala v protokolu $K = (15{,}55 \pm 0{,}03)\ \mathrm{mT/A}$ a nejpřesnější výsledek (při 10 V) byl $e/m = (3{,}57 \pm 0{,}14)\cdot10^{11}\ \mathrm{C\,kg^{-1}}$. Je asi dvakrát větší než tabulková hodnota a rozdíl je mnohem větší než $3\delta$, takže jde o soustavnou chybu. Příčinou je nejspíš zjednodušený model nebo nepřesně známé rozměry elektrod.

## Na co se ptají ve vstupním testu {#test}

> [!note] Okruhy k testu
> - teorie z návodu, magnetické pole, magnetron,
> - indukce uvnitř cívky $B = \mu_0 N I/L$: výpočty ze zadaných hodnot,
> - síla na náboj v elektrickém, magnetickém a kombinovaném poli, tvar dráhy a její parametry ($r = mv/(eB)$): výpočty,
> - pozor na zadané hodnoty, které nejsou k řešení potřeba,
> - měrný náboj z $e = 1{,}6\cdot10^{-19}\ \mathrm{C}$ a $m = 9{,}1\cdot10^{-31}\ \mathrm{kg}$: $1{,}76\cdot10^{11}\ \mathrm{C\,kg^{-1}}$,
> - otázky k zamyšlení: která síla vychyluje elektrony z radiálního směru (magnetická), proč anodový proud neklesne skokem.
