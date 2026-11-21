+++
title = "Úloha 18: Teplotní závislost odporu termistorů a PT100"
topics = ["Úloha 18: termistory a PT100"]
+++

**Úkol:** v rozsahu asi 5–80 °C změřit odpor NTC termistoru, PTC termistoru a platinového snímače PT100 (aspoň 15 teplot), závislosti vynést do grafů a určit teplotní součinitel odporu $\alpha_R$ platiny a energiovou konstantu $B$ NTC termistoru.

## Proč odpor závisí na teplotě {#proc}

S rostoucí teplotou se v krystalu dějí dvě věci proti sobě:

1. **roste koncentrace volných nosičů** náboje (elektronů, u polovodičů i děr), což odpor **snižuje**,
2. atomy mřížky víc kmitají, nosiče se častěji srážejí (**kratší střední doba mezi srážkami**), což odpor **zvyšuje**.

- **Kovy:** volných elektronů je dost i za nízkých teplot a s teplotou jich nepřibývá. Převládají srážky, **odpor roste**, přibližně lineárně.
- **Polovodiče:** převládá nárůst počtu nosičů, **odpor klesá**, exponenciálně. U 0 K nejsou volné nosiče žádné a polovodič je izolant.
- **Díra** je prázdné místo po elektronu, chová se jako volný kladný náboj. Polovodič typu n i p je jako celek elektricky neutrální.

![Teplotní závislost odporu](images/lab-18-teplotni-zavislost.svg "Tvar závislostí R(t) pro NTC, PTC a PT100")

## Kovový snímač PT100 {#pt100}

$$R_T = R_0\,[1 + \alpha_R\,(T - T_0)] \qquad \alpha_R = \frac{1}{R}\frac{\mathrm{d}R}{\mathrm{d}T}$$

$R_0$ je odpor při vztažné teplotě $T_0$. $\alpha_R$ je relativní změna odporu na 1 K, jednotka $\mathrm{K^{-1}}$. Většina kovů má asi $4\cdot10^{-3}\ \mathrm{K^{-1}}$, platina $3{,}925\cdot10^{-3}\ \mathrm{K^{-1}}$. PT100 má 100 Ω při 0 °C.

**Zpracování:** graf $R = f(T - T_0)$ ($T_0$ = teplota, kde měření začalo), lineární regrese $R = a\,(T - T_0) + b$. Porovnáním se vztahem: $b = R_0$, $a = R_0\alpha_R$, tedy
$$\alpha_R = \frac{a}{b}$$

## Termistory {#termistory}

Termistory jsou polovodičové součástky s velkou teplotní závislostí: $|\alpha_R|$ je 5 až 50krát větší než u kovů.

- **NTC** (negativní): odpor s teplotou monotónně klesá.
- **PTC** (pozitivní): odpor roste, nad mezní teplotou prudce. Vyrábí se z feroelektrik, závislost je nelineární.

Pro NTC termistor (ohřev měřicím proudem zanedbáme):

$$R_T = R_0\,\mathrm{e}^{B\left(\frac1T - \frac1{T_0}\right)} \qquad \ln\frac{R_T}{R_0} = B\left(\frac1T - \frac1{T_0}\right) \qquad \alpha_R = -\frac{B}{T^2}$$

$B$ je **energiová konstanta** termistoru (v kelvinech, pro klesající odpor kladná). Všechny teploty v **kelvinech**: $T = t + 273{,}15$.

**Zpracování:** $\ln R$ proti $\left(\frac1T - \frac1{T_0}\right)$ je přímka. Směrnice je $B$, úsek $\ln R_0$. V tabulkovém procesoru stačí exponenciální regrese v semilogaritmickém grafu, exponent u $x$ je přímo $B$.

> [!key] Dva body stačí na B
> $$B = \frac{\ln(R_1/R_2)}{\dfrac1{T_1} - \dfrac1{T_2}}$$
> A naopak, z $B$ a jednoho bodu: $\dfrac1T = \dfrac1{T_0} + \dfrac{\ln(R/R_0)}{B}$.

> [!trap] Časté chyby
> - Teplota ve °C místo v K v exponentu nebo v $\alpha_R = -B/T^2$.
> - $\alpha_R$ NTC termistoru závisí na teplotě (klesá s $1/T^2$), není to konstanta jako u kovu.
> - Rozdíl teplot je v K i ve °C stejný: $\Delta T = \Delta t$.
> - U PTC termistoru čekat na ustálení odporu a průběžně kontrolovat, že se údaj multimetru mění (zapnutý HOLD „zamrazí“ hodnotu).

## Měření {#mereni}

Senzory jsou v komoře s regulátorem teploty (nastavuje se otočným prvkem, displej ukazuje aktuální a nastavenou teplotu). Každý senzor má vlastní multimetr v režimu měření odporu. Začíná se na 5 °C, hodnoty se zapisují po ustálení teploty a odporu, končí se na 80 °C.

> [!example] Příklad z měření
> V protokolu vyšlo pro PT100 $\alpha_R = (3{,}83 \pm 0{,}05)\cdot10^{-3}\ \mathrm{K^{-1}}$ (návod uvádí $3{,}925\cdot10^{-3}\ \mathrm{K^{-1}}$, rozdíl 2,4 %) a pro NTC termistor $B = (2340 \pm 90)\ \mathrm{K}$. Teplotní součinitel NTC termistoru u 5 °C, $\alpha_R = -B/T^2 \doteq -3\cdot10^{-2}\ \mathrm{K^{-1}}$, je asi osmkrát větší než u platiny.

## Na co se ptají ve vstupním testu {#test}

> [!note] Okruhy k testu
> - Ohmův zákon; teplo, teplota, teplotní stupnice,
> - polovodiče: elektrony a díry, měrná vodivost a koncentrace nosičů, závislost počtu nosičů a odporu na teplotě, odpor u 0 K,
> - vedení proudu v kovech, $R_T = R_0[1 + \alpha_R(T - T_0)]$,
> - NTC termistor: $R_T = R_0\,\mathrm{e}^{B(1/T - 1/T_0)}$, logaritmus odporu, energiová konstanta $B$, $\alpha_R = \frac1R\frac{\mathrm{d}R}{\mathrm{d}T} = -B/T^2$,
> - výpočty $R$, $B$ nebo $T$, když jsou ostatní veličiny zadané.
>
> Příklady z návodu: měď 10 Ω při 20 °C a 12,352 Ω při 80 °C $\Rightarrow \alpha_\mathrm{Cu} = 3{,}92\cdot10^{-3}\ \mathrm{K^{-1}}$; termistor 22 kΩ při 20 °C a 8,1 kΩ při 100 °C $\Rightarrow B \doteq 1370\ \mathrm{K}$.
