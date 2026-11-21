+++
title = "Úvod do měření: veličiny a jednotky"
topics = ["Fyzikální veličiny a jednotky"]
+++

Měřit fyzikální veličinu znamená určit její velikost ve zvolené jednotce. Kapitola shrnuje kapitolu 1 skripta *Úvod do měření*: druhy veličin, soustavu SI, předpony a převody jednotek.

## Druhy veličin {#druhy}

Fyzikální veličina popisuje určitou vlastnost nebo stav objektu. Například moment setrvačnosti $J$ je mírou setrvačnosti tělesa při otáčení kolem dané osy.

| druh | co popisuje | při skládání těles | příklady |
|---|---|---|---|
| **extenzivní** (množství) | kvantitu | **sčítají se** (jsou aditivní) | hmotnost, náboj, délka, teplo |
| **intenzivní** (stavové) | kvalitu, stav | **vyrovnávají se** | teplota, tlak, potenciál |
| **protenzivní** | | roste jen jedním směrem | čas |

- U **extenzivní** veličiny se zvolí jedna její hodnota za jednotku a měří se, kolikrát je měřená hodnota větší nebo menší.
- U **intenzivní** veličiny se nejdřív sestaví **stupnice** stavů a stavům se přiřadí čísla. Stupnice se obvykle navazuje na nějakou extenzivní veličinu: teplota se dá definovat tak, aby byla úměrná objemu určitého množství látky.

> [!example] Proč se teploty nesčítají
> Smícháte litr vody o teplotě 20 °C s litrem vody o teplotě 40 °C. Objem (extenzivní) je 2 litry, teplota (intenzivní) se vyrovná zhruba na 30 °C, ne na 60 °C.
>
> **Jak to říct, když se zeptají:** „Objem je extenzivní, při spojení soustav se sčítá. Teplota je intenzivní, popisuje stav, a při spojení se vyrovnává.“

## Hodnota, značka, jednotka {#zapis}

Každá veličina má dohodnutou **značku** (hmotnost $m$, rychlost $v$, proud $I$, často podle anglických názvů: *mass*, *velocity*). Hodnota veličiny je součin číselné hodnoty a jednotky:

$$X = \{X\}\,[X]$$

- $\{X\}$ je **číselná hodnota**, $[X]$ je **jednotka**.
- **Hodnota veličiny na jednotce nezávisí, číselná hodnota ano.** $1{,}5\ \mathrm{m}$ a $150\ \mathrm{cm}$ je táž délka s jinou číselnou hodnotou. Proto se k číslu vždy píše jednotka.
- Podíl veličiny a jednotky je číslo: $\{X\} = X/[X]$. Proto se osy grafů a záhlaví tabulek popisují „$t$ / s“: ve sloupci jsou čísla, která vzniknou vydělením času sekundou.

## Soustava SI {#si}

Platí **Mezinárodní soustava jednotek SI** (přijatá v roce 1960, u nás uzákoněná). Má **sedm základních jednotek**:

| veličina | jednotka | značka |
|---|---|---|
| délka | metr | m |
| hmotnost | kilogram | kg |
| čas | sekunda | s |
| elektrický proud | ampér | A |
| termodynamická teplota | kelvin | K |
| látkové množství | mol | mol |
| svítivost | kandela | cd |

Úvod uvádí klasické definice: **metr** je dráha, kterou světlo urazí ve vakuu za $1/299\,792\,458$ s; **sekunda** je $9\,192\,631\,770$ period záření při přechodu mezi hladinami velmi jemné struktury atomu cesia 133; **kilogram** je hmotnost mezinárodního prototypu v Sèvres; **ampér** je proud, který mezi dvěma rovnoběžnými nekonečně dlouhými vodiči vzdálenými 1 m ve vakuu vyvolá sílu $2\cdot10^{-7}\ \mathrm{N}$ na metr délky; **kelvin** je $1/273{,}16$ termodynamické teploty trojného bodu vody; **mol** obsahuje tolik entit (atomů, molekul, iontů), kolik je atomů ve $0{,}012\ \mathrm{kg}$ uhlíku 12; **kandela** je svítivost zdroje monofrekvenčního záření $540\cdot10^{12}\ \mathrm{Hz}$ se zářivostí $1/683\ \mathrm{W\,sr^{-1}}$.

> [!note] Od roku 2019 jsou definice jiné
> Od 20. 5. 2019 jsou všechny základní jednotky definované **pevnými hodnotami přírodních konstant**: kilogram přes Planckovu konstantu $h = 6{,}626\,070\,15\cdot10^{-34}\ \mathrm{J\,s}$, ampér přes elementární náboj $e = 1{,}602\,176\,634\cdot10^{-19}\ \mathrm{C}$, kelvin přes Boltzmannovu konstantu $k = 1{,}380\,649\cdot10^{-23}\ \mathrm{J\,K^{-1}}$, mol přes Avogadrovu konstantu $N_\mathrm{A} = 6{,}022\,140\,76\cdot10^{23}\ \mathrm{mol^{-1}}$. Metr, sekunda a kandela zůstaly v podstatě stejné. Prototyp kilogramu už jednotku nedefinuje. Velikost jednotek se nezměnila, takže pro výpočty v praktiku je to jedno.

## Odvozené a doplňkové jednotky {#odvozene}

**Odvozené jednotky** vzniknou ze základních pomocí definičních vztahů. Hustota je $\rho = m/V$, takže její jednotka je $\mathrm{kg}/\mathrm{m^3} = \mathrm{kg\,m^{-3}}$. Vyjádření jednotky součinem mocnin základních jednotek se nazývá **rozměr**. Některé odvozené jednotky mají vlastní název, obvykle podle fyzika: newton, joule, volt, ohm.

Základní a odvozené jednotky se souhrnně nazývají **hlavní jednotky**.

| jednotka | z čeho | rozměr |
|---|---|---|
| newton | $F = ma$ | $\mathrm{kg\,m\,s^{-2}}$ |
| joule | $W = Fs$ | $\mathrm{kg\,m^2\,s^{-2}}$ |
| watt | $P = W/t$ | $\mathrm{kg\,m^2\,s^{-3}}$ |
| pascal | $p = F/S$ | $\mathrm{kg\,m^{-1}\,s^{-2}}$ |
| coulomb | $Q = It$ | $\mathrm{A\,s}$ |
| volt | $U = P/I$ | $\mathrm{kg\,m^2\,s^{-3}\,A^{-1}}$ |
| ohm | $R = U/I$ | $\mathrm{kg\,m^2\,s^{-3}\,A^{-2}}$ |

**Doplňkové jednotky** patří bezrozměrným veličinám (jejich rozměr je 1). Rovinný úhel $\varphi = \ell/r$ ($\ell$ délka oblouku, $r$ poloměr) má jednotku **radián**, protože $\mathrm{m}/\mathrm{m} = 1$. Při dosazování do vztahů se rad nepíše. Obdobně **steradián** (sr) pro prostorový úhel.

> [!trap] Úhel do vzorců v radiánech
> Ve vztazích jako $\ell = r\varphi$ nebo $\omega = \varphi/t$ musí být úhel v radiánech: $30^\circ = 30\cdot\pi/180 = 0{,}524\ \mathrm{rad}$. Kalkulačka v režimu DEG počítá s úhlem ve stupních jen u goniometrických funkcí.

## Předpony a vedlejší jednotky {#predpony}

Velmi velké a malé hodnoty se vyjádří **mocninou deseti** nebo **předponou**, přednostně v krocích po $10^3$: $2{,}35\cdot10^{-9}\ \mathrm{s} = 2{,}35\ \mathrm{ns}$. Předpona se píše dohromady s jednotkou.

| předpona | značka | násobek | předpona | značka | násobek |
|---|---|---|---|---|---|
| peta | P | $10^{15}$ | mili | m | $10^{-3}$ |
| tera | T | $10^{12}$ | mikro | µ | $10^{-6}$ |
| giga | G | $10^{9}$ | nano | n | $10^{-9}$ |
| mega | M | $10^{6}$ | piko | p | $10^{-12}$ |
| kilo | k | $10^{3}$ | femto | f | $10^{-15}$ |

Používají se i hekto (h, $10^2$), deka (da, $10$), deci (d, $10^{-1}$) a centi (c, $10^{-2}$), hlavně v cm, dm a hl.

**Vedlejší jednotky** jako minuta, hodina, litr nebo tuna se z praktických důvodů používat smějí, ale **nejsou součástí SI**. Do vzorců je převeďte na hlavní jednotky.

> [!trap] Velké a malé písmeno
> $\mathrm{M}$ (mega) a $\mathrm{m}$ (mili) se liší o $10^9$. $1\ \mathrm{M\Omega} = 10^6\ \Omega$, $1\ \mathrm{m\Omega} = 10^{-3}\ \Omega$. Peta je velké P, piko malé p.

## Převody jednotek {#prevody}

S jednotkami se počítá jako s proměnnými: dosadí se za ně jejich vyjádření a zkrátí se. Protože $1\ \mathrm{min} = 60\ \mathrm{s}$, je $2\ \mathrm{min} = 2\cdot(60\ \mathrm{s}) = 120\ \mathrm{s}$.

> [!example] Rychlost a hustota
> $$36\ \mathrm{km\,h^{-1}} = 36\cdot\frac{1000\ \mathrm{m}}{3600\ \mathrm{s}} = 10\ \mathrm{m\,s^{-1}}$$
> $$0{,}80\ \mathrm{g\,cm^{-3}} = 0{,}80\cdot\frac{10^{-3}\ \mathrm{kg}}{10^{-6}\ \mathrm{m^3}} = 0{,}80\cdot10^{3}\ \mathrm{kg\,m^{-3}} = 800\ \mathrm{kg\,m^{-3}}$$
>
> **Jak to říct, když se zeptají:** „Za kilometr dosadím 1000 metrů a za hodinu 3600 sekund, jednotky se zkrátí. U hustoty je gram $10^{-3}$ kg a krychlový centimetr $(10^{-2}\ \mathrm{m})^3 = 10^{-6}\ \mathrm{m^3}$, takže se číslo vynásobí tisícem.“

> [!example] Kontrola rozměru
> Vyjde-li odpor z $R = U/I$, musí mít výsledek rozměr ohmu. Dosazením rozměrů: $\mathrm{V}/\mathrm{A} = (\mathrm{kg\,m^2\,s^{-3}\,A^{-1}})/\mathrm{A} = \mathrm{kg\,m^2\,s^{-3}\,A^{-2}}$, to je ohm. Rovnice, ve které by rozměry obou stran nesouhlasily, je špatně. Úvod proto žádá, aby rovnice v protokolu byly **rozměrově homogenní**.
>
> **Jak to říct, když se zeptají:** „Obě strany rovnice musí mít stejný rozměr. Když dosadím jednotky a vyjde mi na každé straně něco jiného, mám ve vzorci chybu.“
