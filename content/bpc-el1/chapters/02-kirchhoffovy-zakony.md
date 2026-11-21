+++
title = "Úloha 2: Kirchhoffovy zákony a metoda úměrných veličin"
topics = ["Kirchhoffovy zákony"]
+++

Ověřuje se 1. Kirchhoffův zákon v uzlu, 2. Kirchhoffův zákon ve smyčce a metoda úměrných veličin. Obvod: $R_1$ = 470 Ω, $R_2$ = 1 kΩ, $R_3$ = 820 Ω, $R_4$ = 1,2 kΩ, napětí $U$ = 12 V a podruhé $U'$ podle pokynů na pracovišti. Čtyři miliampérmetry měří proudy větví, jeden voltmetr se přepojuje.

![Obvod úlohy 2](images/kirchhoff-obvod.svg "Obvod se čtyřmi miliampérmetry; zvýrazněná smyčka pro 2. KZ: zdroj, mA, R3, R1, mA1")

## Kirchhoffovy zákony

**1. Kirchhoffův zákon (proudový)** plyne ze zachování náboje: součet proudů v uzlu je nulový.

$$\sum_k I_k = 0$$

Konvence ze skript: proud **do uzlu** se bere se znaménkem **−**, proud **z uzlu** se znaménkem **+**.

**2. Kirchhoffův zákon (napěťový)** plyne ze zachování energie: součet napětí v uzavřené smyčce je nulový (bez proměnného vnějšího magnetického pole).

$$\sum_k U_k = 0$$

Zvolí se směr obíhání. Napětí, jehož šipka míří ve směru obíhání, má **+**, proti směru **−**.

## Metoda úměrných veličin

V **lineárním** obvodu s **jediným nezávislým zdrojem** jsou všechna napětí a proudy úměrné napětí zdroje. Zvětší-li se napětí zdroje $k$-krát, zvětší se $k$-krát vše:

$$k = \frac{X}{X'} = \frac{U}{U'} = \frac{I}{I'} = \frac{I_1}{I_1'} = \dots = \text{konst.}$$

> [!key] Kdy metoda platí
> Lineární obvod + jediný nezávislý zdroj. S nelineárním prvkem (dioda) by poměr $k$ vyšel pro různé veličiny různě.

## Měření a sešit

1. Zapojit podle schématu, nastavit $U$ = 12 V, odečíst $I$, $I_1$, $I_2$, $I_4$ (čtyři ampérmetry najednou).
2. Bez vypínání zdroje přepojovat voltmetr na $U_1$, $U_2$, $U_3$ (voltmetr jen na rezistor, ne přes ampérmetr pod ním).
3. Nastavit $U'$ a zopakovat (řádek $X'$).
4. Kontrola na místě: $I \approx I_1 + I_2 + I_4$, $U_3 + U_1 \approx U$.

Do sešitu: tab. 2.1 (řádky $X$, $X'$ a šedý řádek $k$), 1. KZ pro uzel ①, 2. KZ pro vyznačenou smyčku, příklad výpočtu $k$. Jmenovité hodnoty pro 12 V (bez vlivu ampérmetrů) pro kontrolu: $I \approx$ 20,5 mA, $I_4$ = 10,0 mA, $U_3 \approx$ 8,63 V, $U_1 = U_2 \approx$ 3,37 V.

## Příklad z měření {#priklad}

> [!example] Příklad z měření
> Naměřeno při $U$ = 12,03 V: $I$ = 20,58 mA, $I_1$ = 7,28 mA, $I_2$ = 3,4 mA, $I_4$ = 10,08 mA, $U_1$ = 3,35 V, $U_3$ = 8,6 V; při $U'$ = 6,03 V: $I'$ = 10,32 mA.
>
> **1. KZ pro uzel ①** (do uzlu teče $I$, ven $I_4$ a $I_1 + I_2$):
> $$\sum I = -I + I_1 + I_2 + I_4 = -20{,}58 + 7{,}28 + 3{,}4 + 10{,}08 = 0{,}18\ \text{mA} \approx 0$$
> **2. KZ pro smyčku** (obíhání po směru hodinových ručiček: nahoru přes zdroj, doprava přes $R_3$, dolů přes $R_1$; šipka $U$ míří proti směru obíhání):
> $$\sum U = -U + U_3 + U_1 = -12{,}03 + 8{,}6 + 3{,}35 = -0{,}08\ \text{V} \approx 0$$
> **Poměr $k$** pro proud $I$:
> $$k_I = \frac{I}{I'} = \frac{20{,}58}{10{,}32} = 1{,}99, \qquad k_U = \frac{12{,}03}{6{,}03} = 2{,}00$$
> Všech osm poměrů vyšlo 1,99 až 2,00 (nezaokrouhleně 1,994 až 2,000), metoda úměrných veličin platí.
>
> **Jak to říct, když se zeptají, proč je součet napětí záporný:** „Voltmetr $U$ měří přímo na zdroji, ale ve smyčce jsou ještě ampérmetry mA a mA1 a jejich úbytek se neměří. Úplný zákon je $-U + U_\text{mA} + U_3 + U_1 + U_\text{mA1} = 0$, takže $-U + U_3 + U_1$ musí vyjít záporně. Chybějících 0,08 V při proudech 20,58 a 7,28 mA odpovídá odporu ampérmetru asi 3 Ω. Při poloviční $U'$ chybí poloviční napětí (0,04 V), je to tedy soustavná chyba.“

> [!trap] Na co si dát pozor
> - Znaménka: proud do uzlu −, z uzlu +; napětí po směru obíhání +.
> - Voltmetr pro $U_1$ a $U_2$ patří jen na rezistor, ne přes ampérmetr.
> - $k$ se zaokrouhluje na 3 platné číslice jako měřené hodnoty; rozdíl 1,99 proti 2,00 je až v poslední číslici.
> - Hodnoty s menším počtem desetinných míst (3,4 mA, 8,6 V) se nepřepisují, počítá se s nimi, jak jsou zapsané.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Z čeho plynou Kirchhoffovy zákony?** 1. KZ ze zachování náboje, 2. KZ ze zachování energie.
> - **Proč nevyjde 1. KZ přesně nula?** Každý proud měří jiný ampérmetr s vlastní chybou; 0,18 mA je 0,9 % z $I$.
> - **Kdy platí metoda úměrných veličin?** V lineárním obvodu s jediným nezávislým zdrojem.
> - **Proč jsou $U_1$ a $U_2$ skoro stejné?** $R_1$ a $R_2$ jsou paralelně, liší se jen úbytky na ampérmetrech v jejich větvích.
> - **Proč se voltmetr přepojuje bez vypínání zdroje?** Aby se obvod během měření nezměnil; voltmetr má velký vnitřní odpor a obvod prakticky neovlivní.
