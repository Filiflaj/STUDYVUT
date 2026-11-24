+++
title = "Vzorce k písemce ze cvičení"
topics = [
  "Úloha 5: nakloněná rovina",
  "Úloha 10: reverzní kyvadlo",
  "Úloha 17: ručkové přístroje",
  "Úloha 18: termistory a PT100",
  "Úloha 21: elektromagnetická indukce",
  "Úloha 25: měrný náboj elektronu",
  "Zpracování měření",
]
+++

Symboly jsou jako v protokolech, jednotky v závorkách, (–) značí bezrozměrnou veličinu. Chyby ($\delta$, $u$, $\delta_\mathrm{r}$, $u_\mathrm{r}$) jsou vysvětlené v posledním oddílu.

## Úloha 5: nakloněná rovina {#u05}

$$N = mg\cos\alpha \qquad F_\mathrm{t} = \mu N \qquad a = g\,(\sin\alpha - \mu\cos\alpha) \qquad \mu = \frac{\sin\alpha - a/g}{\cos\alpha}$$

$$t_i = \sum_{k=1}^{i}\Delta t_k \qquad s_i = \sum_{k=1}^{i}\Delta s_k \qquad v_i = \frac{\Delta s_i}{\Delta t_i} \qquad \tau_i = \frac{t_{i-1} + t_i}{2} \qquad s = \tfrac12 at^2 + v_0t + s_0 \qquad v = at + v_0$$

- $\alpha$ úhel sklonu (°); $m$ hmotnost (kg); $N$, $F_\mathrm{t}$ normálová a třecí síla (N); $\mu$ součinitel smykového tření (–), klouže jen při $\mu < \operatorname{tg}\alpha$; $g = 9{,}81\ \mathrm{m\,s^{-2}}$; $a\ (\mathrm{m\,s^{-2}})$ zrychlení, směrnice přímky $v(t)$ (u paraboly $s(t)$ dvojnásobek koeficientu u $t^2$).
- $\Delta s_i$ (m), $\Delta t_i$ (s) délka úseku mezi branami $i-1$ a $i$ a doba průjezdu; $s_i$, $t_i$ vzdálenost brány $i$ od brány 0 a čas průchodu jí; $v_i\ (\mathrm{m\,s^{-1}})$ průměrná rychlost na úseku, patří k času $\tau_i$ (s) uprostřed průjezdu; $s$, $v$ poloha a rychlost v čase $t$, $s_0$, $v_0$ v čase $t = 0$.

## Úloha 10: reverzní kyvadlo {#u10}

$$T = 2\pi\sqrt{\frac{J}{mgd}} \qquad T = 2\pi\sqrt{\frac{\ell}{g}} \qquad g = \frac{4\pi^2\ell_\mathrm{r}}{T^2} \qquad T = \frac{\overline{50T}}{50} \qquad \delta(T) = \frac{\delta(50T)}{50} \qquad \delta_\mathrm{r}(g) = \delta_\mathrm{r}(\ell_\mathrm{r}) + 2\,\delta_\mathrm{r}(T)$$

- $T$ doba jednoho kmitu (s), $T_\mathrm{O1}$, $T_\mathrm{O2}$ kolem os O1, O2; fyzické kyvadlo: $J\ (\mathrm{kg\,m^2})$ moment setrvačnosti, $m$ hmotnost (kg), $d$ vzdálenost osy od těžiště (m); $\ell$ délka matematického kyvadla (m).
- $\ell_\mathrm{r}$ redukovaná délka (m), délka matematického kyvadla se stejnou dobou kmitu, při $T_\mathrm{O1} = T_\mathrm{O2}$ je to vzdálenost os; $a$ poloha závaží (mm). Postupná metoda: časy po 0–40 ($A$) a 50–90 kmitech ($B$), $B - A$ dá 5 hodnot doby 50 kmitů $50T$ (s). Brno: $g = 9{,}8094\ \mathrm{m\,s^{-2}}$.

## Úloha 17: ručkové přístroje {#u17}

$$U_\text{Měř} = \alpha k,\ \ k = \frac{U_\mathrm{R}}{\alpha_\mathrm{max}} \qquad \delta(U_\text{Měř}) = \frac{T_\mathrm{p}}{100}\,U_\mathrm{R} \qquad \delta_\mathrm{r} = \frac{\delta(U_\text{Měř})}{U_\text{Měř}}\cdot100\ \% \qquad U_\mathrm{M} = c\,U_\text{Měř} \qquad \delta(U_\mathrm{M}) = c\,\delta(U_\text{Měř})$$

$$U_\mathrm{ef} = \sqrt{\frac1T\int_0^T u^2\,\mathrm{d}t} \qquad U_\mathrm{s} = \frac1T\int_0^T u\,\mathrm{d}t \qquad k_\mathrm{t} = \frac{\pi}{2\sqrt2} \doteq 1{,}11$$

| průběh | $U_\mathrm{ef}$ | $U_\mathrm{s}$ | V1 (měří $U_\mathrm{s}$) | V2 (s usměrňovačem) | V3 (měří $U_\mathrm{ef}$) |
|---|---|---|---|---|---|
| harmonický | $U_\mathrm{M}/\sqrt2$ | $0$ | nelze | $U_\mathrm{ef}$, $c = \sqrt2$ | $c = \sqrt2$ |
| jednocestně usměrněný | $U_\mathrm{M}/2$ | $U_\mathrm{M}/\pi$ | $c = \pi$ | $k_\mathrm{t}U_\mathrm{s}$, $c = 2\sqrt2$ | $c = 2$ |
| dvoucestně usměrněný | $U_\mathrm{M}/\sqrt2$ | $2U_\mathrm{M}/\pi$ | $c = \pi/2$ | $U_\mathrm{ef}$, $c = \sqrt2$ | $c = \sqrt2$ |

- $U_\mathrm{M}$ amplituda (V), na osciloskopu rozkmit $2U_\mathrm{M}$; $U_\mathrm{ef}$ efektivní hodnota (stejné teplo), $U_\mathrm{s}$ střední hodnota (stejný náboj), $U_\text{Měř}$ údaj voltmetru, vše ve V; $u$ okamžité napětí (V), $T$ perioda (s).
- $\alpha$, $\alpha_\mathrm{max}$ výchylka a počet dílků stupnice; $U_\mathrm{R}$ rozsah (V); $k$ konstanta rozsahu (V/dílek), nezkráceně, např. 60/120; $T_\mathrm{p}$ třída přesnosti (–), mezní chyba v % **rozsahu**; $k_\mathrm{t}$ činitel tvaru (–); $c$ převod údaje na amplitudu (–).

## Úloha 18: termistory a PT100 {#u18}

$$T = (t + 273{,}15)\ \mathrm{K} \qquad \alpha_R = \frac1R\frac{\mathrm{d}R}{\mathrm{d}T} \qquad R = R_0\,[1 + \alpha_R(T - T_0)] \qquad R_3 = a(T - T_0) + b \qquad \alpha_R = \frac{a}{b}$$

$$R = R_0\exp\left[B\left(\frac1T - \frac1{T_0}\right)\right] \qquad \ln\frac{R}{R_0} = B\left(\frac1T - \frac1{T_0}\right) \qquad \alpha_\mathrm{NTC} = -\frac{B}{T^2}$$

- $t$ teplota (°C), $T$ termodynamická teplota (K), do vzorců vždy $T$; $T_0$ vztažná teplota na začátku měření (5 °C = 278,15 K), $R_0$ odpor při ní (Ω); $R_1$, $R_2$, $R_3$ odpor NTC (kΩ), PTC a PT100 (Ω).
- $\alpha_R\ (\mathrm{K^{-1}})$ teplotní součinitel odporu, relativní změna odporu na 1 K; $a\ (\Omega\,\mathrm{K^{-1}})$ směrnice a $b$ (Ω) úsek přímky pro PT100, $a = R_0\alpha_R$, $b = R_0$; $B$ (K) energiová konstanta NTC, směrnice $\ln R$ proti $1/T - 1/T_0$.

## Úloha 21: elektromagnetická indukce {#u21}

$$u_\mathrm{ind} = -N_\mathrm{i}\frac{\mathrm{d}\Phi}{\mathrm{d}t} \qquad \Phi = BS\cos\theta \qquad U_\mathrm{ind}(\theta) = U_\mathrm{ind}(0)\cos\theta \qquad y_\theta = \frac{U_\mathrm{ind}(\theta)}{U_\mathrm{ind}(0)},\ \ x = \cos\theta \qquad y_d = \alpha\,d^{\,n},\ \ \ln y_d = \ln\alpha + n\ln d$$

$$Y = \frac{U_\mathrm{ind}}{I_\mathrm{b}} = \omega M,\ \ \omega = 2\pi f \qquad k = \frac{M}{\sqrt{\tilde L_\mathrm{b}\tilde L_\mathrm{i}}} \qquad L_\mathrm{W} = \frac{r^2N^2}{9r + 10\ell}\ \mu\mathrm{H} \qquad L_\mathrm{N} = K_\mathrm{N}\,\mu_0\,\frac{N^2\pi r^2}{\ell}$$

$$|Z| = \frac{U_\mathrm{b,ef}}{I_\mathrm{b,ef}},\ \ I_\mathrm{b,ef} = \frac{I_\mathrm{b}}{\sqrt2} \qquad |Z|^2 = R_\mathrm{s}^2 + \omega^2L_\mathrm{b}^2 \qquad \Delta\varphi = 360^\circ f\,\Delta t \qquad \Delta\varphi = \pm90^\circ - \operatorname{arctg}\frac{\omega L_\mathrm{b}}{R_\mathrm{s}} \qquad f_0 = \frac{R_\mathrm{s}}{2\pi L_\mathrm{b}}$$

- $N_\mathrm{b}$, $N_\mathrm{i}$ počet závitů budicí a indukční cívky (470 a 460); $\Phi$ magnetický tok (Wb); $B$ indukce (T); $S$ plocha závitu (m²); $\theta$ natočení cívky (°); $d$ vzdálenost středů (m); $y_\theta$, $y_d$ napětí normované napětím při $\theta = 0$, resp. $d = 0$ (–); $\alpha$ koeficient a $n$ exponent mocninné závislosti (–), daleko (dipól) $n = -3$.
- $U_\mathrm{b}$, $U_\mathrm{ind}$ budicí a indukované napětí (V), amplituda = polovina špička–špička; $I_\mathrm{b}$ amplituda budicího proudu (A); $U_\mathrm{b,ef}$, $I_\mathrm{b,ef}$ jejich efektivní hodnoty; $f$ (Hz), $\omega\ (\mathrm{rad\,s^{-1}})$ frekvence a úhlová frekvence.
- $M$ (H) vzájemná indukčnost, směrnice $Y(\omega)$; $Y$ (Ω) podíl $U_\mathrm{ind}/I_\mathrm{b}$, není to impedance; $k$ koeficient spřažení (–), $0 \le k \le 1$; $\tilde L_\mathrm{b} = 2{,}04\ \mathrm{mH}$, $\tilde L_\mathrm{i} = 5{,}74\ \mathrm{mH}$ z impedančního analyzátoru.
- $L_\mathrm{W}$, $L_\mathrm{N}$ (H) vlastní indukčnost cívky odhadnutá z rozměrů; $N$ počet závitů; $r$, $\ell$ poloměr a délka cívky, u Wheelera v palcích (1 in = 25,4 mm), u Nagaoky v m; $K_\mathrm{N}$ Nagaokův faktor (–), $0 < K_\mathrm{N} < 1$; $\mu_0 = 4\pi\cdot10^{-7}\ \mathrm{H\,m^{-1}}$.
- $|Z|$, $R_\mathrm{s}$ velikost impedance budicí větve a její sériový odpor (Ω), $L_\mathrm{b}$ (H) indukčnost budicí cívky z impedance; z přímky $|Z|^2 = a\omega^2 + b$: $L_\mathrm{b} = \sqrt a$, $R_\mathrm{s} = \sqrt b$; $\Delta t$ časový (s) a $\Delta\varphi$ fázový posun (°); $f_0$ (Hz) posun o $45^\circ$ od $90^\circ$.

## Úloha 25: měrný náboj elektronu {#u25}

$$B = \mu_0\frac{N}{L}\,I_\mathrm{C} = K I_\mathrm{C} \qquad K = \frac{\sum I_iB_i}{\sum I_i^2} \qquad B_0 = K I_0 \qquad F_\mathrm{e} = eE \qquad F_\mathrm{m} = evB$$

$$\frac{e}{m} = \frac{8\,U_\mathrm{A}R_\mathrm{a}^2}{(R_\mathrm{a}^2 - R_\mathrm{k}^2)^2\,B_0^2} = \frac{G\,U_\mathrm{A}}{B_0^2} \qquad \delta_\mathrm{r}(B_0) = \delta_\mathrm{r}(K) + \delta_\mathrm{r}(I_0) \qquad \delta_\mathrm{r}(e/m) = u_\mathrm{r}(U_\mathrm{A}) + 2\,\delta_\mathrm{r}(B_0)$$

- $N$, $L$ počet závitů a délka cívky (–, m); $I_\mathrm{C}$ proud cívkou (A); $K\ (\mathrm{T\,A^{-1}})$ konstanta cívky, směrnice $B(I_\mathrm{C})$ z magnetometru; $U_\mathrm{A}$ anodové napětí (V), $I_\mathrm{A}$ anodový proud (μA); $e/m\ (\mathrm{C\,kg^{-1}})$ měrný náboj, tabulkově $1{,}76\cdot10^{11}$; $e$ náboj elektronu (C), $v$ jeho rychlost ($\mathrm{m\,s^{-1}}$), $E$ intenzita elektrického pole ($\mathrm{V\,m^{-1}}$), $F_\mathrm{e}$, $F_\mathrm{m}$ elektrická a magnetická síla (N).
- $I_0$ (A) kritický proud, inflexní bod sestupné větve $I_\mathrm{A}(I_\mathrm{C})$; $B_0$ (T) kritická indukce, dráha se anody jen dotkne; $R_\mathrm{a} = 5\ \mathrm{mm}$, $R_\mathrm{k} = 3\ \mathrm{mm}$ poloměr anody a katody; $G = 7{,}8125\cdot10^{5}\ \mathrm{m^{-2}}$ geometrický činitel.

## Zpracování měření {#zpracovani}

**Opakované měření** (náhodná chyba, zhruba „typ A“), $P = 95\ \%$, $t_{5;\,0{,}95} = 2{,}968$, $t_{10;\,0{,}95} = 2{,}320$:

$$\bar x = \frac1n\sum_{i=1}^{n}x_i \qquad s_{\bar x} = \sqrt{\frac{\sum(x_i - \bar x)^2}{n\,(n-1)}} \qquad \delta(X) = t_{n,P}\,s_{\bar x} \qquad \delta_\mathrm{r}(X) = \frac{\delta(X)}{\bar x}\cdot100\ \%$$

$x_i$ naměřené hodnoty, $n$ jejich počet, $\bar x$ průměr, $s_{\bar x}$ výběrová směrodatná odchylka průměru, $\delta(X)$ chyba (vše v jednotce veličiny), $\delta_\mathrm{r}$ relativní chyba (%), $t_{n,P}$ koeficient Studentova rozdělení pro pravděpodobnost $P$ (–). Proložení přímkou: $t$ pro $\nu = n - 2$ stupňů volnosti (přímka počátkem $\nu = n - 1$), např. $\nu = 7$: $2{,}365$.

**Chyba měřidla** $u$ (zhruba „typ B“), $u_\mathrm{r} = u/x\cdot100\ \%$: stupnice nejmenší dílek, dohodou jeho polovina (dílek 0,5 mm: $u = 0{,}25\ \mathrm{mm}$); ručkový přístroj $u = T_\mathrm{p}/100\cdot\text{rozsah}$; číslicový údaj rozlišení displeje (např. $u(U_\mathrm{A}) = 0{,}001\ \mathrm{V}$). Obě chyby se **porovnají a dál se počítá s větší**, protokoly je nesčítají. Hodnota odečtená jen jednou má jen chybu měřidla.

**Šíření chyb** pro $V = f(X, Y)$ (zjednodušený zákon, $c_1$, $c_2$, $c$, $k$, $m$ jsou konstanty) a **přímka metodou nejmenších čtverců** $y = ax + b$ (směrnice $a$, úsek $b$), přímka počátkem $y = Kx$:

$$\delta(V) = \left\lvert\frac{\partial f}{\partial x}\right\rvert\delta(X) + \left\lvert\frac{\partial f}{\partial y}\right\rvert\delta(Y) \qquad V = c_1X \pm c_2Y:\ \delta(V) = \lvert c_1\rvert\,\delta(X) + \lvert c_2\rvert\,\delta(Y) \qquad V = cX^kY^m:\ \delta_\mathrm{r}(V) = \lvert k\rvert\,\delta_\mathrm{r}(X) + \lvert m\rvert\,\delta_\mathrm{r}(Y) \qquad a = \frac{\sum(x_i - \bar x)(y_i - \bar y)}{\sum(x_i - \bar x)^2} \qquad b = \bar y - a\bar x \qquad K = \frac{\sum x_iy_i}{\sum x_i^2}$$

**Zaokrouhlení a zápis:** chyba vždy **nahoru** na 1 platnou číslici (na 2, začíná-li jedničkou: $0{,}0121 \to 0{,}013$), výsledek na stejný řád jako chyba. Zápis $X = (\bar x \pm \delta)\ \text{jednotka}$, $n$, $P$, $\delta_\mathrm{r}$. Liší-li se výsledek od tabulkové hodnoty o víc než $3\delta$, jde nejspíš o soustavnou chybu.
