+++
title = "Práce s osciloskopem"
topics = ["Osciloskop"]
+++

Osciloskop kreslí časový průběh napětí, $u = f(t)$. Spolu s multimetrem a generátorem je to základní přístroj elektrotechnické laboratoře. Tahle kapitola vysvětluje, jak z obrazovky odečíst hodnoty, co je spouštění, šířka pásma a sonda 10 : 1 a co znamenají parametry signálu, které osciloskop umí změřit sám. Měří se v laboratorní úloze s osciloskopem (v plánu LC5, návod LC6).

## Typy osciloskopů

- **Analogový**: starší technologie, už se nevyrábí.
- **DSO** (Digital Storage Oscilloscope): číslicový, signál vzorkuje a ukládá do paměti. Umí zachytit jednorázový děj, ukázat signál i **před** okamžikem spuštění, sám změřit parametry, uložit průběh, komunikovat s počítačem (USB, Ethernet), často má i FFT.
- **MSO** (Mixed Signal Oscilloscope): DSO s dalšími binárními (logickými) vstupy, typicky 8 až 64 digitálních kanálů.

## Obrazovka a odečítání hodnot

Obrazovka je rozdělená mřížkou na **dílky** (div, divisions). Svislá vzdálenost dílků odpovídá nastavení **V/dílek**, vodorovná **s/dílek**.

![Obrazovka osciloskopu](images/osciloskop-displej.svg "Sinusový signál: perioda 5 dílků, rozkmit 6 dílků, maximum 4 dílky nad nulou; 1 V/dílek, 1 µs/dílek")

> [!example] Odečtení z obrazovky
> Nastavení 1 V/dílek a 1 µs/dílek.
> - perioda: $T = 5\ \text{dílků}\cdot1\ \mu\text{s/dílek} = 5\ \mu\text{s}$, kmitočet $f = 1/T = 200$ kHz,
> - rozkmit (špička–špička): $U_\text{pp} = 6\ \text{dílků}\cdot1\ \text{V/dílek} = 6$ V,
> - maximum: $U_\text{max} = +4\ \text{dílky}\cdot1\ \text{V/dílek} = +4$ V, minimum $U_\text{min} = U_\text{max} - U_\text{pp} = 4 - 6 = -2$ V.
>
> Signál má tedy amplitudu 3 V a stejnosměrnou složku +1 V.
>
> **Jak to říct, když se zeptají:** „Počet dílků krát nastavené měřítko. Perioda se měří mezi dvěma stejnými body průběhu, třeba mezi sousedními vrcholy, a kmitočet je její převrácená hodnota.“

Místo počítání dílků jde použít **kurzory** (dvojice svislých X a vodorovných Y čar; osciloskop sám ukazuje jejich polohu i rozdíl) nebo **automatické měření** vybraných parametrů.

## Hlavní ovládací prvky

- **Vertical:** měřítko (V/dílek) a svislá poloha průběhu (posun) pro každý kanál.
- **Horizontal:** časová základna (s/dílek, společná všem kanálům) a vodorovná poloha; zoom ukáže zvětšený výřez.
- **Trigger level:** úroveň spouštění.
- **Vstupy BNC** pro sondy.
- **Auto Setup**: osciloskop sám nastaví měřítka, posun i spouštění pro běžné signály. **Default** vrátí výchozí nastavení.
- **Vazba (Coupling):** DC ukazuje celý signál, AC odfiltruje stejnosměrnou složku.

> [!key] Vhodné nastavení
> - V/dílek tak, aby průběh **skoro vyplnil** obrazovku na výšku, a svislým posunem ho vystředit.
> - s/dílek tak, aby bylo vidět **několik málo period**.
> - Úroveň spouštění **blízko středu** průběhu.

## Spouštění (trigger)

Zobrazená křivka se skládá z mnoha snímků po sobě. Aby se skládaly na sebe, musí každý snímek začínat ve **stejném bodě** opakujícího se průběhu, podobně jako cílová fotografie na dostizích. Nejčastěji se spouští na **náběžnou nebo sestupnou hranu** při zvolené úrovni napětí (třeba náběžná hrana při 0 V). Když je úroveň spouštění mimo rozsah signálu, průběh není synchronizovaný a „ujíždí“. U DSO je bod spuštění standardně uprostřed obrazovky; vlevo od něj je „záporný čas“ (signál před spuštěním).

## Důležité parametry DSO

- **Šířka pásma** (bandwidth, BW): nejdůležitější parametr. Osciloskop se chová jako dolní propust; na kmitočtu BW je amplituda zobrazeného signálu o 3 dB menší, tedy asi o 30 %. Obdélník 100 MHz se na osciloskopu s BW 100 MHz zobrazí skoro jako sinus, protože vyšší harmonické neprojdou.
- **Vzorkovací rychlost** (vzorků/s): má být aspoň **4 × BW**.
- **Hloubka paměti:** kolik vzorků se uloží, tedy jak dlouhý úsek jde zaznamenat.
- **Počet kanálů:** obvykle 2 nebo 4 analogové.
- **Obnovovací rychlost** průběhů: čím vyšší, tím spíš je vidět nahodilý jev.
- Kvalita displeje, pokročilé spouštění (šířka impulzu, vzor, video, sériové sběrnice…).

## Sonda 10 : 1

Sonda přenáší signál z měřeného obvodu do osciloskopu. Nejběžnější je **pasivní sonda 10 : 1**: kompenzovaný dělič, který **10× zmenší** amplitudu signálu a zároveň **10× zvětší** vstupní impedanci, takže méně zatěžuje měřený obvod. Existují i sondy 100 : 1, 1000 : 1, aktivní, diferenciální, proudové…

Poměr sondy je nutné nastavit **na sondě i v osciloskopu** (Probe 10×), jinak budou napětí desetkrát špatně.

**Kompenzace sondy:** parazitní kapacity sondy, kabelu a vstupu zkreslují tvar signálu. Před měřením se sonda připojí na kalibrační výstup osciloskopu (obdélník asi 1 V, 1 kHz) a trimrem na sondě se nastaví **dokonalý obdélník**. Překompenzovaná sonda má na hranách překmity, nedokompenzovaná zaoblené hrany. Trimr se točí nekovovým šroubovákem a citlivě.

## Časová a kmitočtová oblast

Periodický signál jde popsat v **časové oblasti** $x(t)$ (měří osciloskop) nebo v **kmitočtové oblasti** $X(f)$, tedy spektrem (měří spektrální analyzátor). Moderní osciloskopy mají spektrální analýzu (FFT) zabudovanou.

## Parametry signálu

![Časové parametry](images/parametry-impulzu.svg "Náběžná hrana tr a sestupná hrana tf mezi 10 % a 90 %, šířka kladného impulzu v polovině výšky, perioda T")

**Časové parametry**

- perioda $T$ a kmitočet $f = 1/T$,
- **doba náběžné hrany** $t_\text{r}$ (Rise): růst z 10 % na 90 % maxima,
- **doba sestupné hrany** $t_\text{f}$ (Fall): pokles z 90 % na 10 %,
- **šířka kladného impulzu** (+Width): od náběžné po sestupnou hranu v polovině výšky; **záporného** (−Width) obráceně,
- **střída** (duty): $\text{střída} = \dfrac{+\text{šířka}}{T}$, obvykle v %.

**Napěťové parametry**

| Parametr | Význam |
|---|---|
| $U_\text{max}$, $U_\text{min}$ | největší a nejmenší hodnota za periodu |
| $U_\text{pp}$ | rozkmit, $U_\text{max} - U_\text{min}$ |
| $U_\text{top}$, $U_\text{base}$ | napětí plochého vrcholu a plochého minima |
| $U_\text{amp}$ | $U_\text{top} - U_\text{base}$ |
| $U_\text{avg}$ | střední hodnota |
| $U_\text{rms}$ | efektivní hodnota |

Pro souměrné průběhy bez stejnosměrné složky s vrcholovou hodnotou $U_\text{m}$ (tedy $U_\text{pp} = 2U_\text{m}$):

| Průběh | $U_\text{avg}$ | $U_\text{rms}$ |
|---|---|---|
| sinus | 0 | $U_\text{m}/\sqrt{2} \approx 0{,}707\,U_\text{m}$ |
| trojúhelník | 0 | $U_\text{m}/\sqrt{3} \approx 0{,}577\,U_\text{m}$ |
| obdélník (střída 50 %) | 0 | $U_\text{m}$ |

> [!trap] Na co si dát pozor
> - Sonda přepnutá na 10×, ale v osciloskopu nastaveno 1× (nebo naopak): napětí vyjdou desetkrát špatně, časy správně.
> - $U_\text{pp}$ je dvojnásobek amplitudy, ne amplituda.
> - S vazbou AC zmizí stejnosměrná složka, střední hodnota pak vyjde skoro nulová.
> - Doba hrany se měří mezi 10 % a 90 %, ne mezi 0 a 100 %.
