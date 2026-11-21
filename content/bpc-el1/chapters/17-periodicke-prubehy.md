+++
title = "Cvičení: časově proměnné veličiny"
topics = ["Periodické průběhy"]
+++

Napětí a proud se v čase mění. Ve cvičení PC12 a v COZ6 se u **periodických** průběhů počítají charakteristické hodnoty (střední, střední absolutní, efektivní), činitel tvaru a výkyvu a činitel harmonického zkreslení.

## Druhy průběhů

- **stacionární** (stejnosměrné): neměnné v čase,
- **periodické**: $f(t + kT) = f(t)$, $T$ je perioda, $f = 1/T$ kmitočet; mezi nimi **střídavé** (střední hodnota nula), **pulzující** (neměnné znaménko) a obecně kmitavé; **harmonický** průběh je sinusovka $u = U_\text{m}\sin(\omega t + \varphi)$, $\omega = 2\pi f$,
- **neperiodické**: přechodné děje, jednotlivé impulzy (řeší se v Elektrotechnice 2).

## Charakteristické hodnoty

| hodnota | značka | definice | význam |
|---|---|---|---|
| střední (stejnosměrná složka, AVG) | $I_0$ | $\dfrac1T\displaystyle\int_0^T i(t)\,\mathrm{d}t$ | stálý proud, který přenese stejný náboj |
| střední absolutní (ARV) | $I_\text{sa}$ | $\dfrac1T\displaystyle\int_0^T \lvert i(t)\rvert\,\mathrm{d}t$ | používá se v měřicí technice |
| efektivní (RMS) | $I$ | $\sqrt{\dfrac1T\displaystyle\int_0^T i^2(t)\,\mathrm{d}t}$ | stálý proud se stejnými tepelnými účinky v rezistoru |
| maximální (vrcholová) | $I_\text{m}$ | $\max\lvert i(t)\rvert$ | |
| činitel tvaru | $k_\text{t}$ | $I/I_\text{sa}$ | |
| činitel výkyvu | $k_\text{v}$ | $I_\text{m}/I$ | |

Geometricky je střední hodnota výška obdélníku o délce $T$ se stejnou plochou, jakou má průběh (plochy pod osou se odečítají). U střední absolutní hodnoty se plochy pod osou překlopí nahoru. Efektivní hodnota se značí prostým velkým písmenem bez indexu.

> [!key] Postup výpočtu
> 1. Rozdělit periodu na intervaly, ve kterých je průběh jedna funkce (konstanta, přímka, sinus).
> 2. Na každém intervalu zapsat funkci. Přímka jdoucí počátkem: $i = k t$, kde směrnice $k$ je přírůstek lomeno délkou intervalu.
> 3. Integrovat po intervalech a sečíst. Pro RMS se integruje **čtverec** a odmocňuje se až nakonec.

![Průběh ze sbírky 9.1](images/cv-prubeh-f.svg "+Im do T/2, −Im/2 od T/2 do 3T/4, +Im od 3T/4 do T")

> [!example] Sbírka 9.1: obdélníkový průběh, $I_\text{m} = 1\ \text{A}$
> Na intervalech $\langle 0; T/2\rangle$, $\langle T/2; 3T/4\rangle$, $\langle 3T/4; T\rangle$ je proud $I_\text{m}$, $-I_\text{m}/2$, $I_\text{m}$.
> $$I_0 = \frac1T\left[I_\text{m}\frac T2 - \frac{I_\text{m}}{2}\cdot\frac T4 + I_\text{m}\frac T4\right] = I_\text{m}\left(\frac12 - \frac18 + \frac14\right) = \frac58 I_\text{m} = 0{,}625\ \text{A}$$
> $$I_\text{sa} = I_\text{m}\left(\frac12 + \frac18 + \frac14\right) = \frac78 I_\text{m} = 0{,}875\ \text{A}$$
> $$I = \sqrt{\frac1T\left[I_\text{m}^2\frac T2 + \frac{I_\text{m}^2}{4}\cdot\frac T4 + I_\text{m}^2\frac T4\right]} = I_\text{m}\sqrt{\frac12 + \frac1{16} + \frac14} = I_\text{m}\sqrt{\frac{13}{16}} = 0{,}901\,4\ \text{A}$$
> $$k_\text{t} = \frac{I}{I_\text{sa}} = \frac{0{,}901\,4}{0{,}875} = 1{,}030, \qquad k_\text{v} = \frac{I_\text{m}}{I} = \frac{1}{0{,}901\,4} = 1{,}109$$
>
> **Jak to říct, když se zeptají:** „Průběh je po částech konstantní, takže integrál je součet obdélníků. Pro efektivní hodnotu umocním každý úsek na druhou, sečtu plochy, vydělím periodou a odmocním. Záporný úsek se ve střední hodnotě odečte, ve střední absolutní přičte.“

U lineárního úseku $i = kt$ je $\int_0^{\tau} kt\,\mathrm{d}t = k\tau^2/2$ a $\int_0^{\tau} k^2t^2\,\mathrm{d}t = k^2\tau^3/3$. Pro pilu, která za $T/2$ vystoupá na $I_\text{m}$ a pak je nulová (sbírka 9.2), tak vyjde $I_0 = I_\text{m}/4$ a $I = I_\text{m}/\sqrt6$.

## Hodnoty běžných průběhů

| průběh | $U_0$ | $U_\text{sa}$ | $U$ | $k_\text{t}$ | $k_\text{v}$ |
|---|---|---|---|---|---|
| sinusový | 0 | $2U_\text{m}/\pi$ | $U_\text{m}/\sqrt2$ | $\pi/(2\sqrt2) \doteq 1{,}11$ | $\sqrt2$ |
| dvoucestně usměrněný sinus | $2U_\text{m}/\pi$ | $2U_\text{m}/\pi$ | $U_\text{m}/\sqrt2$ | 1,11 | $\sqrt2$ |
| jednocestně usměrněný sinus | $U_\text{m}/\pi$ | $U_\text{m}/\pi$ | $U_\text{m}/2$ | $\pi/2$ | 2 |
| trojúhelníkový, pilovitý (souměrný) | 0 | $U_\text{m}/2$ | $U_\text{m}/\sqrt3$ | $2/\sqrt3$ | $\sqrt3$ |
| obdélníkový (souměrný) | 0 | $U_\text{m}$ | $U_\text{m}$ | 1 | 1 |
| obdélníkové impulzy, šířka $t_1$ | $U_\text{m}t_1/T$ | $U_\text{m}t_1/T$ | $U_\text{m}\sqrt{t_1/T}$ | $\sqrt{T/t_1}$ | $\sqrt{T/t_1}$ |

> [!trap] Překlep ve sbírce
> U sinusového a dvoucestně usměrněného průběhu (sbírka 9.4 a 9.10) je jako činitel výkyvu napsáno 1,141. Správně je $k_\text{v} = \sqrt2 \doteq 1{,}414$ (číslice prohozené).

> [!note] Proč na tom záleží při měření
> Levné měřicí přístroje měří střední absolutní hodnotu a stupnici mají násobenou činitelem tvaru sinusovky 1,11. Pro jiný tvar ukazují špatně: u obdélníku ($k_\text{t} = 1$) o 11 % víc. Přístroje označené **TRMS** měří skutečnou efektivní hodnotu, ale jen do určitého činitele výkyvu.

## Harmonická analýza a zkreslení

Periodický průběh jde rozložit (Fourierova řada) na stejnosměrnou složku a harmonické složky s kmitočty $f$, $2f$, $3f$, … Grafem amplitud podle řádu harmonické je **spektrum**. Efektivní hodnota celého průběhu je
$$U = \sqrt{U_0^2 + U_1^2 + U_2^2 + \dots}$$
(se složkami v efektivních hodnotách). Míru zkreslení původně harmonického signálu udává **činitel harmonického zkreslení**:
$$\text{THD}_\text{F} = \frac{\sqrt{U_2^2 + U_3^2 + \dots}}{U_1}, \qquad \text{THD}_\text{R} = \frac{\sqrt{U_2^2 + U_3^2 + \dots}}{\sqrt{U_1^2 + U_2^2 + U_3^2 + \dots}}$$
$\text{THD}_\text{F}$ vztahuje vyšší harmonické k základní (audiotechnika), $\text{THD}_\text{R}$ k efektivní hodnotě střídavé části, snáz se měří a nepřekročí 100 %. Stejnosměrná složka v žádném z nich není.

> [!example] Sbírka 9.5: zkreslení proudu
> $I_0 = 2\ \text{A}$, $I_1 = 10\ \text{A}$, $I_3 = 1{,}5\ \text{A}$, $I_5 = 0{,}6\ \text{A}$:
> $$\text{THD}_\text{F} = \frac{\sqrt{1{,}5^2 + 0{,}6^2}}{10} = \frac{\sqrt{2{,}61}}{10} = \frac{1{,}616}{10} = 16{,}16\ \%$$
> $$\text{THD}_\text{R} = \frac{1{,}616}{\sqrt{10^2 + 2{,}61}} = \frac{1{,}616}{10{,}13} = 15{,}95\ \%$$
> Pro malé zkreslení se obě čísla skoro neliší. $I_0$ se do výpočtu nedosazuje.
