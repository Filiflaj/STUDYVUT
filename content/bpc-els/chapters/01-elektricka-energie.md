+++
title = "Úvod: elektrická energie, její výroba a využití"
topics = ["Elektrická energie"]
+++

První týden semináře je přednáška o energii: odkud se elektřina bere, k čemu je dobrá, kolik jí Česko vyrábí a spotřebuje a proč se vyplatí šetřit malými spotřebiči. K tomu základní veličiny a jednotky, se kterými se pracuje celý semestr.

## Základní veličiny a jednotky

| Veličina | Značka | Jednotka |
|---|---|---|
| náboj | $Q$ | coulomb (C) |
| proud | $I$ | ampér (A) |
| napětí | $U$ | volt (V) |
| výkon | $P$ | watt (W) |
| energie (práce) | $E$, $W$ | joule (J) |
| kmitočet | $f$ | hertz (Hz) |
| odpor / vodivost | $R$ / $G$ | ohm (Ω) / siemens (S) |
| kapacita | $C$ | farad (F) |
| indukčnost | $L$ | henry (H) |

Předpony SI, které se v obvodech objevují nejčastěji:

| p | n | µ | m | k | M | G | T |
|---|---|---|---|---|---|---|---|
| $10^{-12}$ | $10^{-9}$ | $10^{-6}$ | $10^{-3}$ | $10^{3}$ | $10^{6}$ | $10^{9}$ | $10^{12}$ |

Energie a výkon spolu souvisí přes čas: $E = P\,t$. V energetice se energie neudává v joulech, ale v **kilowatthodinách**:

$$1\ \text{kWh} = 1000\ \text{W} \cdot 3600\ \text{s} = 3{,}6\cdot10^{6}\ \text{J}$$

## Odkud se elektřina bere

Elektrická energie **vždy vzniká přeměnou** z jiné formy energie:

- **mechanické**, přes elektromagnetickou indukci (alternátor, dynamo),
- **chemické** v galvanickém článku (první byl Voltův sloup z roku 1800),
- **tepelné** díky Seebeckovu jevu (termočlánek, používá se k měření teploty),
- **světelné** ve fotočlánku.

Galvanický článek vznikne všude, kde se ve vlhku dotýkají dva různé kovy. Proto se třeba **měděný plech nepřišroubuje ocelovými vruty**: vznikne článek a méně ušlechtilý kov (ocel) koroduje. Stejný jev se využívá k ochraně ocelových potrubí a lodních trupů: k chráněné oceli se vodivě připojí **obětovaná elektroda** z méně ušlechtilého kovu (hořčík, hliník, zinek), která koroduje přednostně a po rozpuštění se vymění.

## Účinky elektrického proudu

| Účinek | Kdy | Příklady |
|---|---|---|
| tepelný (Jouleovo teplo) | **vždy** | žehlička, bojler, páječka, tavná pojistka |
| magnetický | **vždy** | relé, stykač, elektromotor, reproduktor, zvonek |
| světelný | v plynech a polovodičích | doutnavka, LED, zářivka, výbojka, oblouk |
| chemický | ve vodivých kapalinách | elektrolýza, galvanické pokovování, nabíjení akumulátoru |
| fyziologický | na živé organismy | ohradník, kardiostimulátor, defibrilátor; úraz proudem |

> [!key] Co platí vždy
> Každý proud vodičem **zahřívá** a kolem sebe vytváří **magnetické pole**. Ostatní účinky se objeví jen v určitém prostředí.

## Trocha historie

Elektrizační soustavy vznikají od poloviny 19. století. Zpočátku se energie přenášela **stejnosměrným proudem**, se kterým ale nešlo vyrábět a přenášet větší výkony. Zlom přinesl střídavý proud spolu se synchronním alternátorem, trojfázovou soustavou, transformátorem a asynchronním motorem.

V českých zemích:

- 1878: první využití elektřiny, obloukové lampy v tkalcovně v Moravské Třebové,
- 1882: **Městské divadlo v Brně** jako první plně elektrifikované divadlo v Evropě (Edisonova soustava, vlastní parní elektrárna asi 300 m od divadla); téhož roku Křižíkovy obloukovky na Staroměstské radnici v Praze,
- 1889: první městská elektrárna (Praha-Žižkov), 1900 první větší elektrárna na střídavý proud (Holešovice),
- 1919: zákon o všeobecné elektrizaci; 1920 zavedena **trojfázová soustava 50 Hz, 3 × 380/220 V** pro místní sítě a 100 kV pro dálková vedení,
- 1926: první velká elektrárna (Ervěnice, 70 MW), 1948 první přečerpávací elektrárna (Štěchovice),
- 1955: elektrifikováno celé území (poslední obec byla Hrčava).

## Energie a hospodářství

Spotřeba energie a HDP spolu rostou: tvorba bohatství energii potřebuje, jde v podstatě o jednu a tutéž věc. Cílem proto není absolutně snižovat spotřebu, ale aby rostoucí ekonomika a kvalita života spotřebovávaly **co nejméně energie na jednotku užitku**, a to v celém řetězci od primárních zdrojů po výrobu, provoz a likvidaci spotřebičů. Budoucí růst poptávky se čeká hlavně v rozvojových zemích (asi 15 % lidí dosud nemá přístup k elektřině).

### Proč právě elektřina

Elektřinu jde snadno a s vysokou účinností přeměnit na jiné formy energie:

| Přeměna na | Účinnost | Příklad |
|---|---|---|
| mechanickou | > 90 % | elektromotory |
| tepelnou | > 90 % | topidla, vařiče |
| elektrickou (jiné napětí) | až 98 % | transformátory, měniče |
| světlo | malá | žárovka < 8 %, výbojka < 40 % |
| chemickou | kolem 90 % | elektrolýza, akumulátory |

Dál: přenáší se bez přenosu hmoty přesně do místa spotřeby, výkon jde regulovat okamžitě a bez setrvačnosti, všechny její parametry jdou snadno měřit a může nést i signál (informaci).

## Výroba a spotřeba v Česku

Čísla z prezentace (zdroje EGÚ Brno, ERÚ):

- roční **spotřeba** elektřiny v ČR 2024: **56,9 TWh**,
- roční **výroba** 2025: **76,4 TWh**, instalovaný výkon 2025: **23 823 MW**,
- ČR je propojená se sousedními státy a elektřinu ve velkém vyváží i dováží.

Struktura výroby v roce 2025:

| Zdroj | Podíl |
|---|---|
| jádro | 42 % |
| hnědé uhlí | 33 % |
| černé uhlí | 1 % |
| zemní plyn | 5 % |
| fotovoltaika | 6 % |
| biomasa | 4 % |
| bioplyn | 3 % |
| vodní | 2 % |
| větrné | 1 % |
| ostatní plyny, přečerpávací | po 1 % |
| **OZE celkem** | **16 %** |

**Přečerpávací vodní elektrárna** v době přebytku (v noci, při slunečném poledni) čerpá vodu do horní nádrže a ve špičce ji pouští zpět přes turbíny. Elektřinu neumí vyrobit „navíc“, ale přesouvá ji v čase a pomáhá vyrovnávat výkon v síti.

### Srovnání zdrojů

Plocha potřebná pro výkon 1000 MW a emise CO₂ během životního cyklu:

| Zdroj | Plocha (km²) | CO₂ (g/kWh) |
|---|---|---|
| větrná | 600 | 10 – 38 |
| fotovoltaická | 130 | 78 – 217 |
| vodní | 32 | 4 – 36 |
| uhelná | 4 | 750 – 1250 |
| biomasa (pěstování) | 6600 | 29 – 62 |
| jaderná | 10 | 5 – 33 |

**Fotovoltaika:** malá hustota toku energie (do 1 kW/m²), komerční články mají účinnost 22 až 24 %, energetická návratnost asi 5 let. Dodávka je nespolehlivá a „v protifázi“ se spotřebou (nejméně vyrábí, když se nejvíc spotřebovává), takže vyžaduje investice do sítí a úložišť.

**Vítr:** také malá hustota toku energie, návratnost asi 6 měsíců, potřebuje rychlost větru nad 6 m/s, hlučnost a vliv na krajinu, nespolehlivá dodávka komplikuje regulaci sítě.

Podle uváděné statistiky úmrtí na vyrobenou TWh elektřiny vychází nejhůř uhlí (161), pak ropa (36), biomasa (12) a zemní plyn (4); solární (0,44), větrné (0,15), vodní (0,10) a jaderné elektrárny (0,04) jsou o řády bezpečnější.

## Hospodaření s energií: malé spotřebiče

- **Stand-by** tvoří až 11 % spotřeby domácností; v EU se odhaduje na 47 TWh ročně, zhruba spotřeba Portugalska. Set-top boxy mají v pohotovosti 2 až 20 W.
- **Klasická žárovka** přemění na světlo jen 3 až 8 % energie. Osvětlení tvoří asi 19 % světové spotřeby elektřiny, náhrada LED a úspornými zdroji šetří hodně.
- **Oběhová čerpadla** topení mívají příkon kolem 100 W, i když fyzikálně stačí pár wattů; podobně ventilátory s účinností kolem 4 %. Nové motory podle směrnic EU (ErP) ušetří až 80 %.

> [!example] Výpočet: kolik stojí stand-by
> Set-top box má v pohotovosti příkon 5 W a je zapojený celý rok (365 · 24 = 8760 h):
> $$E = P\,t = 5\ \text{W}\cdot 8760\ \text{h} = 43\,800\ \text{Wh} = 43{,}8\ \text{kWh}$$
> Při ceně 6 Kč/kWh to je $43{,}8 \cdot 6 = 262{,}8$ Kč za rok za přístroj, který „nic nedělá“.
>
> **Jak to říct, když se zeptají:** „Energie je výkon krát čas. Watty krát hodiny dají watthodiny, tisíc watthodin je kilowatthodina, a ta se platí.“

> [!example] Výpočet: jak moc jsou české elektrárny vytížené
> Kdyby všech 23 823 MW běželo celý rok naplno, vyrobily by
> $$E_\text{max} = 23\,823\ \text{MW} \cdot 8760\ \text{h} = 208\,689\,480\ \text{MWh} \approx 208{,}7\ \text{TWh}$$
> Skutečná výroba 76,4 TWh je z toho
> $$\frac{76{,}4}{208{,}7} = 0{,}366 \approx 37\ \%$$
> Elektrárny tedy v průměru jedou asi na třetinu instalovaného výkonu: odstávky, regulace, a hlavně slunce a vítr, které nefoukají a nesvítí pořád.
>
> **Jak to říct, když se zeptají:** „Instalovaný výkon krát počet hodin v roce je teoretické maximum; podíl skutečné výroby a tohoto maxima je průměrné využití.“

> [!trap] Na co si dát pozor
> - kW je výkon, kWh je energie. „Spotřeba 2 kW“ nedává smysl bez času.
> - 1 kWh = 3,6 MJ, ne 3600 J (zapomenutý tisíc u kilo).
> - TWh = $10^{12}$ Wh = $10^{9}$ kWh.
