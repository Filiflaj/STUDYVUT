+++
title = "Úloha 21: Elektromagnetická indukce ve spřažených cívkách"
topics = ["Úloha 21: elektromagnetická indukce"]
+++

**Úkol:** na dvou cívkách spojených jen magnetickou vazbou změřit, jak indukované napětí závisí na natočení a na vzdálenosti cívek, určit vzájemnou indukčnost $M$ a koeficient vazby $k$, odhadnout vlastní indukčnost budicí cívky z rozměrů a z impedance a změřit fázový posun mezi budicím a indukovaným napětím.

## Teorie {#teorie}

**Magnetický tok** plochou $S$ ohraničenou křivkou $\partial S$:

$$\Phi_B = \int_S \vec{B}\cdot\mathrm{d}\vec{S} \qquad \text{v homogenním poli } \Phi_B = BS\cos\theta$$

$\mathrm{d}\vec S$ je orientovaný plošný element (kolmý k ploše), $\theta$ úhel mezi normálou plochy a $\vec B$. Rozhoduje průmět plochy do směru pole. Plocha sama není jednoznačná, podstatná je jen její okrajová křivka $\partial S$.

**Faradayův zákon a Lenzovo pravidlo:**

$$\mathcal{E} = -\frac{\mathrm{d}\Phi_B}{\mathrm{d}t} \qquad \oint_{\partial S}\vec{E}\cdot\mathrm{d}\vec{\ell} = -\frac{\mathrm{d}}{\mathrm{d}t}\int_S\vec{B}\cdot\mathrm{d}\vec{S}$$

Minus je Lenzovo pravidlo: indukovaný proud teče tak, aby **bránil změně toku**, která ho vyvolala. Druhý zápis (Maxwellova–Faradayova rovnice v integrálním tvaru) říká, že proměnné magnetické pole vytváří **vírové elektrické pole**. Jeho oběh po uzavřené křivce není nulový, takže to není elektrostatické (konzervativní) pole.

**Cívka** s $N$ závity: toková vazba $\lambda = N\Phi_B$ a $\mathcal{E} = -N\,\mathrm{d}\Phi_B/\mathrm{d}t$. Indukované napětí roste úměrně počtu závitů.

![Dvě cívky](images/lab-21-civky.svg "a) souosé cívky ve vzdálenosti d, b) indukční cívka natočená o úhel θ")

## Úkol 1: natočení cívek {#natoceni}

Pole budicí cívky prochází indukční cívkou se složkou úměrnou $\cos\theta$, proto
$$U_\mathrm{ind}(\theta) = U_\mathrm{ind}(0)\cos\theta$$
Zpracování: $y = U_\mathrm{ind}(\theta)/U_\mathrm{ind}(0)$ proti $x = \cos\theta$ má být přímka $y = x$ (směrnice 1, úsek 0). Osciloskop ukazuje jen velikost amplitudy, proto se za $90^\circ$ musí hodnotám ručně dát **záporné znaménko**.

## Úkol 2: vzdálenost cívek {#vzdalenost}

Daleko od budicí cívky ($d \gg \ell$) má její pole na ose tvar pole **magnetického dipólu**, $B_z \propto 1/d^3$, a tedy $U_\mathrm{ind} \propto d^{-3}$. Hledá se mocninný zákon
$$y = \alpha\,d^{\,n} \quad\Rightarrow\quad \log y = \log\alpha + n\log d$$
V **log–log grafu** je to přímka, jejíž **směrnice je exponent $n$** (dipól: $n \approx -3$). Koeficient determinace $R^2$ říká, jak dobře body přímka vystihuje. Data mají dva režimy: při zasunutí cívek do sebe napětí klesá jen málo, po oddělení výrazně. Fituje se jen oblast bez překryvu a v protokolu se uvede, ve kterém rozsahu $d$.

## Úkol 3: vzájemná indukčnost {#M}

Toková vazba indukční cívky je úměrná proudu budicí cívkou, $\lambda_\mathrm{ind} = M I_\mathrm{b}$. Pro harmonický proud a osciloskop s velkou vstupní impedancí (indukční cívkou neteče proud):
$$U_\mathrm{ind} \approx \omega M I_\mathrm{b} \qquad Y = \frac{U_\mathrm{ind}}{I_\mathrm{b}} \approx M\,\omega \qquad k = \frac{M}{\sqrt{L_\mathrm{b}L_\mathrm{ind}}}$$
$Y$ má jednotku ohm, ale není to impedance. **Směrnice přímky $Y(\omega)$ je $M$.** Koeficient vazby $0 \le k \le 1$ (1 = ideální transformátor); u vzduchových cívek je při velké vzdálenosti řádu $10^{-3}$, při překryvu desetiny. Amplitudy $U_\mathrm{ind}$ a $I_\mathrm{b}$ se musí brát stejně (obě špičkové, nebo obě efektivní).

## Úkol 4: vlastní indukčnost budicí cívky {#L}

**Z rozměrů.** Wheelerův empirický vztah (pozor, $r$ a $\ell$ **v palcích**, výsledek v μH) a Nagaokův vztah (v metrech):
$$L_\mathrm{W} \approx \frac{r^2N^2}{9r + 10\ell}\ [\mu\mathrm{H}] \qquad L_\mathrm{N} \approx K_\mathrm{N}\,\mu_0\,\frac{N^2\pi r^2}{\ell}$$
Bez korekce ($K_\mathrm{N} = 1$) je to indukčnost dlouhého solenoidu. Nagaokův faktor $0 < K_\mathrm{N} < 1$ ji opravuje na cívku konečné délky a závisí na poměru $r/\ell$: dlouhá cívka $K_\mathrm{N} \to 1$, krátká cívka menší. Wheelerův vztah odpovídá přibližně $K_\mathrm{N} \approx 1/(1 + 0{,}9\,r/\ell)$.

> [!note] Tvar Nagaokova faktoru
> Přesný $K_\mathrm{N}$ se počítá z úplných eliptických integrálů. Ve vzorci (27) v návodu vychází pro $r/\ell \to 0$ hodnota 2 místo 1, proto je dobré dosazení ověřit. Protokol použil standardní tvar s modulem $\kappa = 2\xi/\sqrt{1 + 4\xi^2}$, $\xi = r/\ell$:
> $$K_\mathrm{N} = \frac{4}{3\pi\kappa'}\left[\frac{\kappa'^2}{\kappa^2}\bigl(K(\kappa) - E(\kappa)\bigr) + E(\kappa) - \kappa\right] \qquad \kappa' = \sqrt{1 - \kappa^2}$$
> Kontrola: pro $r/\ell = 0{,}5$ vyjde $K_\mathrm{N} \doteq 0{,}688$.

**Z impedance.** Budicí větev je sériový model $R$–$L$: $|Z| = U_\mathrm{b,ef}/I_\mathrm{b,ef}$ a
$$|Z|^2 = R^2 + \omega^2L_\mathrm{b}^2$$
Graf $|Z|^2$ proti $\omega^2$ je přímka $a\omega^2 + b$: $L_\mathrm{b} = \sqrt{a}$, $R = \sqrt{b}$ ($R$ zahrnuje odpor vinutí i výstupní odpor zdroje). Fituje se jen při vyšších frekvencích (asi nad 4–5 kHz), kde už indukčnost převládá nad odporem.

## Úkol 5: fázový posun {#faze}

$$\Delta\varphi = 360^\circ\cdot f\cdot\Delta t \qquad \Delta\varphi(\omega) = \pm\left(\frac{\pi}{2} - \operatorname{arctg}\frac{\omega L_\mathrm{b}}{R_\mathrm{b}}\right)$$
Při nízkých frekvencích je budicí napětí skoro ve fázi s proudem a indukované napětí ($\propto \mathrm{d}I/\mathrm{d}t$) je posunuté o $90^\circ$. Při vysokých frekvencích jsou obě napětí úměrná $\mathrm{d}I/\mathrm{d}t$ a posun jde k $0^\circ$ (po otočení cívky o 180° k $\pm180^\circ$). Zlom je kolem $f_0 \approx R_\mathrm{b}/(2\pi L_\mathrm{b})$.

> [!trap] Časté chyby
> - Wheelerův vztah s rozměry v mm nebo m (patří tam palce, 1 in = 25,4 mm).
> - $U_\mathrm{pp}$ (špička–špička) místo amplitudy: amplituda je polovina.
> - Fit $n$ přes celý rozsah vzdáleností včetně zasunutých cívek.
> - Ve $\Delta\varphi = 360^\circ f\Delta t$ dosazovat $\Delta t$ v sekundách a $f$ v hertzích.

> [!example] Příklad z měření
> V protokolu vyšel exponent $n = -3{,}9 \pm 0{,}7$ (6 bodů pro $d \ge 120$ mm), tedy slučitelný s dipólovým $-3$, a při úplném zasunutí cívek $M \doteq 0{,}9\ \mathrm{mH}$, $k \doteq 0{,}26$.

## Na co se ptají ve vstupním testu {#test}

> [!note] Okruhy k testu
> - elektromagnetická indukce a indukované napětí, vírové elektrické pole,
> - magnetický tok, orientovaný plošný element, průmět plochy, rozhoduje okrajová křivka $\partial S$,
> - Faradayův zákon, Lenzovo pravidlo (znaménko minus), směr indukovaného proudu,
> - Maxwellova–Faradayova rovnice v integrálním tvaru, nekonzervativní pole,
> - cívka: $\lambda = N\Phi_B$, napětí úměrné $N$,
> - natočení: $\cos\theta$, normalizace $U(\theta)/U(0)$, graf $y$ proti $\cos\theta$,
> - vzdálenost: dipól $B_z \propto 1/d^3$, log–log graf, směrnice = exponent, $R^2$, dva režimy,
> - indukčnost: Wheeler (palce!), Nagaoka a $K_\mathrm{N}(r/\ell)$, impedance $|Z|^2 = R^2 + \omega^2L^2$,
> - fázový posun z $\Delta t$ a $f$, limity pro nízké a vysoké frekvence, otočení cívky o 180°.
