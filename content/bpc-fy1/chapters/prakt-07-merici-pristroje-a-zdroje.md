+++
title = "Úvod do měření: přístroje a zdroje"
topics = ["Měřicí přístroje a zdroje"]
+++

Napětí a proud se měří **ručkovými (analogovými)** nebo **číslicovými (digitálními)** přístroji. Kapitola shrnuje kapitolu 7 skripta *Úvod do měření* a část kapitoly 3 o chybách přístrojů: ampérmetr a voltmetr, změnu rozsahu, třídu přesnosti, číslicové přístroje, potenciometr a laboratorní zdroj.

## Ručkový přístroj a jeho rozsah {#rucka}

Ručkový přístroj převádí proud na výchylku silovým působením magnetického pole. Jeho **měřicí systém** je popsaný **vlastní spotřebou**: vnitřním odporem a hodnotou, která vyvolá plnou výchylku. Například magnetoelektrický systém **60 mV / 10 Ω** má plnou výchylku při napětí $60\ \mathrm{mV}$, tedy při proudu $60\ \mathrm{mV}/10\ \Omega = 6\ \mathrm{mA}$. To je nejmenší rozsah. Tentýž systém se s připojenými rezistory stane ampérmetrem nebo voltmetrem s větším rozsahem.

## Ampérmetr a bočník {#ampermetr}

- Zapojuje se **do série** se spotřebičem.
- Má mít **co nejmenší vnitřní odpor** $R_\mathrm{A}$, aby měřený proud co nejméně změnil.
- Rozsah se zvětšuje **bočníkem** $R_\mathrm{b}$ zapojeným **paralelně**. Pro rozsah $n$-krát větší ($I = nI_\mathrm{A}$):

$$R_\mathrm{b} = \frac{R_\mathrm{A}}{n - 1}$$

Odvození: na bočníku i systému je stejné napětí $R_\mathrm{A}I_\mathrm{A}$, bočníkem teče zbytek proudu $(n-1)I_\mathrm{A}$, tedy $R_\mathrm{b} = R_\mathrm{A}I_\mathrm{A}/\big((n-1)I_\mathrm{A}\big)$.

## Voltmetr a předřadník {#voltmetr}

- Připojuje se **paralelně** k měřenému prvku.
- Má mít **co největší vnitřní odpor** $R_\mathrm{V}$, aby z obvodu odebíral co nejmenší proud.
- Rozsah se zvětšuje **předřadníkem** $R_\mathrm{p}$ zapojeným **sériově**. Pro rozsah $n$-krát větší ($U = nU_\mathrm{V}$):

$$R_\mathrm{p} = R_\mathrm{V}\,(n - 1)$$

Odvození: systémem i předřadníkem teče stejný proud $U_\mathrm{V}/R_\mathrm{V}$, na předřadníku musí zůstat $(n-1)U_\mathrm{V}$.

![Bočník a předřadník](images/prakt-bocnik-predradnik.svg "Bočník se k ampérmetru zapojuje paralelně, předřadník k voltmetru sériově")

> [!example] Bočník a předřadník pro jeden systém
> Ze systému 60 mV / 10 Ω chceme ampérmetr s rozsahem 60 mA a voltmetr s rozsahem 6 V. Plná výchylka nastane při $I_\mathrm{A} = 6\ \mathrm{mA}$ a $U_\mathrm{V} = 60\ \mathrm{mV}$.
> $$\text{ampérmetr: } n = \frac{60\ \mathrm{mA}}{6\ \mathrm{mA}} = 10 \qquad R_\mathrm{b} = \frac{10\ \Omega}{10 - 1} = 1{,}11\ \Omega$$
> $$\text{voltmetr: } n = \frac{6\ \mathrm{V}}{0{,}060\ \mathrm{V}} = 100 \qquad R_\mathrm{p} = 10\ \Omega\cdot(100 - 1) = 990\ \Omega$$
>
> **Jak to říct, když se zeptají:** „Při plné výchylce teče systémem 6 mA. Má-li přístroj měřit 60 mA, musí zbylých 54 mA, devětkrát víc, téct bočníkem. Ten proto musí mít devětkrát menší odpor než systém. U voltmetru má na předřadníku zůstat 99 % napětí, proto má předřadník 99krát větší odpor než systém.“

> [!trap] Proč ne obráceně
> Voltmetr zapojený do série obvod skoro rozpojí (velký odpor). Ampérmetr připojený paralelně ke zdroji je téměř zkrat: proud může přístroj zničit.

## Třída přesnosti {#trida}

Ručkový přístroj má na stupnici **třídu přesnosti** $T_\mathrm{p}$ (např. 0,5; 1; 1,5). Udává **mezní chybu v procentech zvoleného rozsahu**. Na celém rozsahu je tato chyba stejná:

$$u(X) = \frac{T_\mathrm{p}}{100}\cdot X_\text{rozsah} \qquad \delta_\mathrm{r}(X) = \frac{u(X)}{x_\mathrm{N}}$$

- Do této meze se podle výrobce vejdou všechny dílčí soustavné vlivy dohromady (nepřesnost výroby, ohřev vlastní spotřebou, stárnutí, tření) i největší náhodná chyba.
- Chyba z třídy přesnosti bývá větší než desetina nejmenšího dílku. Odečítejte přesto co nejpečlivěji, aspoň na polovinu nejjemnějšího dílku, a pozor na **paralaxu** (dívejte se kolmo, u stupnice se zrcátkem tak, aby ručka zakryla svůj odraz).
- Na různých rozsazích je chyba různá. **Absolutní chyba je na rozsahu konstantní, relativní roste, čím menší je výchylka.**

![Relativní chyba podle výchylky](images/prakt-trida-presnosti.svg "Voltmetr třídy 1,5 na rozsahu 60 V: relativní chyba v závislosti na naměřeném napětí")

> [!key] Ručka do poslední třetiny stupnice
> Rozsah volte tak, aby výchylka byla v **poslední třetině**, nebo aspoň ve druhé polovině stupnice. Jen tam je relativní chyba jen o málo větší než třída přesnosti.

> [!example] Volba rozsahu
> Měříte napětí asi 4,5 V voltmetrem třídy 1,5 s rozsahy 6 V, 15 V a 30 V.
>
> | rozsah | $u = 0{,}015\cdot$ rozsah | $\delta_\mathrm{r}$ při 4,5 V |
> |---|---|---|
> | 6 V | 0,09 V | 2 % |
> | 15 V | 0,225 V | 5 % |
> | 30 V | 0,45 V | 10 % |
>
> Nejlepší je rozsah 6 V: 4,5 V je tři čtvrtiny stupnice. Stejně tak ampérmetr třídy 1 na rozsahu 300 mA má chybu 3 mA: při 240 mA je to 1,25 %, při 60 mA už 5 %.
>
> **Jak to říct, když se zeptají:** „Třída přesnosti je procento z rozsahu, takže absolutní chyba je na celém rozsahu stejná. Čím menší část stupnice využiju, tím větší je relativní chyba. Proto beru nejmenší rozsah, na kterém se hodnota ještě vejde.“

## Číslicové přístroje {#cislicove}

U číslicových přístrojů mezní chybu neurčuje třída přesnosti. Výrobce ji obvykle udává jako **součet dvou částí**:

- **procento z naměřené hodnoty** (anglicky *of reading*),
- **procento z plného rozsahu** (*of range*), kde se projeví hlavně kvantování. Často se tahle část píše jako **počet digitů**, tedy jednotek posledního zobrazeného místa.

> [!example] Multimetr
> Údaj výrobce: $\pm(0{,}5\ \%\ \text{z hodnoty} + 0{,}1\ \%\ \text{z rozsahu})$. Na rozsahu 20 V ukazuje 7,36 V.
> $$u = 0{,}005\cdot7{,}36\ \mathrm{V} + 0{,}001\cdot20\ \mathrm{V} = 0{,}0368 + 0{,}020 = 0{,}0568\ \mathrm{V} \to 0{,}06\ \mathrm{V}$$
> $U = (7{,}36 \pm 0{,}06)\ \mathrm{V}$, $\delta_\mathrm{r} \doteq 0{,}8\ \%$. Kdyby výrobce uvedl „+ 2 digity“, byla by druhá část $2\cdot0{,}01\ \mathrm{V} = 0{,}02\ \mathrm{V}$ (poslední místo displeje jsou setiny voltu).
>
> **Jak to říct, když se zeptají:** „Chyba číslicového přístroje má dvě části: jedna je úměrná naměřené hodnotě, druhá je pevná a závisí na rozsahu nebo na rozlišení displeje. Sečetl jsem je a zaokrouhlil nahoru.“

> [!trap] Rozlišení není přesnost
> Že displej ukazuje čtyři číslice, neznamená, že jsou všechny platné. Bez údaje výrobce berte chybu aspoň jako jednotku posledního místa.

## Potenciometr: dělič a reostat {#potenciometr}

Napětí nebo proud jiný, než dává zdroj, se nejjednodušeji nastaví **potenciometrem**, rezistorem s posuvným jezdcem. Jezdec ho dělí na části $R_1$ a $R_2$.

![Dělič napětí a reostat](images/prakt-delic.svg "Potenciometr zapojený jako dělič napětí se zátěží a jako reostat")

**Dělič napětí** (regulace napětí směrem dolů). Výstupní napětí naprázdno a se zátěží $R_\mathrm{Z}$:

$$U_2 = U\,\frac{R_2}{R_1 + R_2} \qquad U_2 = U\,\frac{\dfrac{R_2R_\mathrm{Z}}{R_2 + R_\mathrm{Z}}}{R_1 + \dfrac{R_2R_\mathrm{Z}}{R_2 + R_\mathrm{Z}}}$$

Zátěž je k $R_2$ paralelně, zmenší tedy dolní část děliče a výstupní napětí klesne.

**Reostat** (regulace proudu): proud teče jen částí $R_1$, část $R_2$ se neuplatní:

$$I = \frac{U}{R_1 + R_\mathrm{A} + R_\mathrm{Z}}$$

> [!example] Zatížený dělič
> Zdroj 10 V, potenciometr nastavený na $R_1 = R_2 = 1\ \mathrm{k\Omega}$. Naprázdno je $U_2 = 10\cdot\dfrac{1}{2} = 5{,}0\ \mathrm{V}$. Se zátěží $R_\mathrm{Z} = 1\ \mathrm{k\Omega}$ je paralelní kombinace $R_2$ a $R_\mathrm{Z}$ rovna $500\ \Omega$:
> $$U_2 = 10\ \mathrm{V}\cdot\frac{500}{1000 + 500} = 3{,}33\ \mathrm{V}$$
>
> **Jak to říct, když se zeptají:** „Zátěž je paralelně k dolní části děliče, takže ta má místo 1 kΩ jen 500 Ω. Napětí se dělí v poměru 500 : 1000, na výstupu je třetina napětí zdroje. Dělič se proto nastavuje až se zapojenou zátěží a napětí se měří voltmetrem.“

## Stabilizovaný zdroj BS 525 {#zdroj}

- Dva **nezávislé** stabilizované zdroje, každý plynule **0 až 30 V**.
- Každý má nastavitelnou **proudovou ochranu** (omezení výstupního proudu, zhruba 0,05 A až 1,1 A). Když ochrana omezuje, svítí kontrolka u ovládacího prvku.
- Vestavěný ukazatel ukazuje podle přepínače výstupní napětí, nebo odebíraný proud. Je jen orientační, **pro přesné napětí použijte vnější voltmetr**.
- Každý zdroj má tři zdířky: výstupní napětí je mezi dvěma krajními. Prostřední **zelená** vede jen na **kostru** přístroje. Se dvěma rezistory připojenými zvenku se s ní dá udělat symetrické napětí.
- Pro napětí nad 30 V se zdroje spojí **do série**. Jednu z výstupních svorek je pak nutné propojit s kostrou a obě jednotky musí mít nastavený **stejný omezovací proud**.

Další číslicové přístroje a zdroje jsou popsané u jednotlivých úloh.
