+++
title = "Cvičení: základní pojmy, prvky R, L, C a zdroje"
topics = ["Základní pojmy a zdroje", "Prvky R, L, C"]
+++

Kapitoly od 10 dál patří k **početním cvičením** (PC3 až PC13) a k pracovním sešitům **COZ1 až COZ6**. Laboratorní úlohy jsou v kapitolách 00 až 08 a tady se na ně jen odkazuje. Ve cvičeních se píšou dva testy po 35 minutách: v PC7 postupné zjednodušování a MSP s maticí 2×2 (12 bodů), v PC11 MUN s maticí 2×2 a metoda náhradního zdroje (13 bodů).

## Veličiny, se kterými se počítá

| veličina | značka | jednotka | vztah |
|---|---|---|---|
| náboj | $Q$ | C | $Q = I t$ pro stálý proud |
| proud | $I$, $i(t)$ | A | $i = \mathrm{d}q/\mathrm{d}t$ |
| napětí | $U$, $u(t)$ | V | práce na přenesení jednotkového náboje, rozdíl potenciálů |
| výkon | $P$ | W | $P = U I$ |
| práce (energie) | $W$ | J, kWh | $W = P t$, $1\ \text{kWh} = 3{,}6\ \text{MJ}$ |
| účinnost | $\eta$ | – | $\eta = P_2/P_1$, ztráty $P_\text{z} = P_1 - P_2$ |

Malá písmena ($u$, $i$) značí okamžité hodnoty, velká ($U$, $I$) stálé nebo efektivní hodnoty. Účinnosti řetězce zařízení (třeba motor pohánějící generátor) se násobí: $\eta = \eta_1 \eta_2$.

Obvod se skládá z **větví** (část mezi dvěma uzly), **uzlů** (vodivé spojení prvků, i dlouhý drát bez prvku je jediný uzel) a **smyček**. Smyčka, kterou nerozděluje žádná větev, je **oko**.

## Čítací šipky

Šipky napětí a proudu určují, jak se veličina počítá v rovnicích, ne nutně skutečný směr. Šipka napětí míří od + k −.

![Spotřebičová a zdrojová orientace](images/cv-citaci-sipky.svg "U spotřebiče míří šipky U a I stejně, u zdroje proti sobě")

- **spotřebičová orientace**: šipky $U$ a $I$ souhlasně, $P = UI > 0$ znamená, že prvek energii odebírá (u rezistoru vždy),
- **zdrojová orientace**: šipky proti sobě, $P = UI > 0$ znamená, že prvek energii dodává.

Když šipky nejsou zadané, zvolí se libovolně a výsledek to řekne sám: **kladné** číslo znamená, že skutečný směr souhlasí se šipkou, **záporné**, že je opačný. Zdroj může v obvodu s více zdroji pracovat i jako spotřebič (nabíjený akumulátor).

> [!key] Ohm a Kirchhoff
> $U = R I$ (pro šipky souhlasně). 1. KZ: algebraický součet proudů v uzlu je nula. 2. KZ: algebraický součet napětí podél uzavřené smyčky je nula. Podrobně s měřením v kapitole **Úloha 2: Kirchhoffovy zákony**.

## Pasivní prvky R, L, C

| prvek | vztah $u$–$i$ | opačně | energie |
|---|---|---|---|
| rezistor | $u = R\,i$ | $i = G\,u$ | mění se v teplo: $W = \int R i^2\,\mathrm{d}t$ |
| induktor (cívka) | $u = L\,\dfrac{\mathrm{d}i}{\mathrm{d}t}$ | $i(t) = i(0) + \dfrac{1}{L}\displaystyle\int_0^t u\,\mathrm{d}t$ | $W_\text{m} = \tfrac12 L i^2$ |
| kapacitor | $i = C\,\dfrac{\mathrm{d}u}{\mathrm{d}t}$ | $u(t) = u(0) + \dfrac{1}{C}\displaystyle\int_0^t i\,\mathrm{d}t$ | $W_\text{e} = \tfrac12 C u^2$ |

Pro úsečkové průběhy (COZ1) to znamená: derivace úsečky je její **směrnice**, integrál je **plocha** pod průběhem. Napětí na cívce je tedy po úsecích konstantní tam, kde proud roste nebo klesá lineárně; proud kapacitorem je konstantní tam, kde napětí roste lineárně.

> [!trap] Co se nemůže změnit skokem
> Proud cívkou a napětí na kapacitoru jsou spojité (energie nemůže skočit). Skok proudu cívkou by znamenal nekonečné napětí. Napětí na cívce a proud kapacitorem skákat mohou.

> [!example] COZ1, př. 1.3: napětí na cívce
> Proud cívkou je do 2 ms nulový, do 4 ms lineárně vzroste na 10 mA a do 9 ms lineárně klesne na nulu. List indukčnost neuvádí; měřítko grafů mu odpovídá $L = 4\ \text{mH}$.
>
> ![Proud cívkou](images/cv-coz1-proud.svg "Zadaný průběh proudu cívkou")
>
> Na každém úseku $u = L\,\Delta i/\Delta t$:
> $$u_{2\text{–}4} = 4\cdot10^{-3}\cdot\frac{10\cdot10^{-3}}{2\cdot10^{-3}} = 4\cdot10^{-3}\cdot 5 = 20\ \text{mV}$$
> $$u_{4\text{–}9} = 4\cdot10^{-3}\cdot\frac{-10\cdot10^{-3}}{5\cdot10^{-3}} = 4\cdot10^{-3}\cdot(-2) = -8\ \text{mV}$$
> Mimo tyto úseky se proud nemění a napětí je nulové. Výsledek je obdélníkový průběh: 0, +20 mV, −8 mV, 0. Opačná úloha (př. 1.4) z obdélníkového napětí počítá plochy: $i(4\ \text{ms}) = \frac{1}{L}\cdot 20\ \text{mV}\cdot 2\ \text{ms} = 10\ \text{mA}$.
>
> **Jak to říct, když se zeptají:** „Napětí na cívce je $L$ krát derivace proudu. Proud je po úsecích lineární, takže derivace je směrnice úsečky a napětí je po úsecích konstantní. Kladné, když proud roste, záporné, když klesá.“

## Zdroje

- **Ideální zdroj napětí** drží napětí $U$ bez ohledu na proud ($R_\text{i} = 0$), **ideální zdroj proudu** drží proud $I$ bez ohledu na napětí ($G_\text{i} = 0$, $R_\text{i} \to \infty$).
- **Reálný zdroj** se modeluje jako zdroj napětí $U_\text{i}$ se sériovým $R_\text{i}$ (Théveninův model), nebo jako zdroj proudu $I_\text{i}$ s paralelní $G_\text{i}$ (Nortonův model). Oba jsou zvenku stejné, když $I_\text{i} = U_\text{i}/R_\text{i}$ a $G_\text{i} = 1/R_\text{i}$. Měření i grafy jsou v kapitole **Úloha 1: Parametry reálného zdroje**.

![Ekvivalentní modely zdroje](images/ekvivalence-zdroju.svg "Napěťový a proudový model téhož reálného zdroje")

Parametry modelu se určí ze dvou stavů. Z napětí naprázdno a napětí při zatížení: $U_\text{i} = U_0$, $R_\text{i} = (U_0 - U)/I$. Ze dvou zatížení známými rezistory se napíše $R I = U_\text{i} - R_\text{i} I$ dvakrát a rovnice se odečtou.

> [!example] COZ2, př. 2.2: model ze dvou zatížení
> Se zátěží $R_1 = 68\ \Omega$ teče $I_1 = 0{,}15\ \text{A}$, se zátěží $R_2 = 100\ \Omega$ teče $I_2 = 0{,}106\ \text{A}$.
> $$R_1 I_1 = U_\text{i} - R_\text{i} I_1, \qquad R_2 I_2 = U_\text{i} - R_\text{i} I_2$$
> Odečtením zmizí $U_\text{i}$:
> $$R_\text{i} = \frac{R_1 I_1 - R_2 I_2}{I_2 - I_1} = \frac{10{,}2 - 10{,}6}{0{,}106 - 0{,}15} = \frac{-0{,}4}{-0{,}044} = 9{,}091\ \Omega$$
> $$U_\text{i} = R_\text{i} I_1 + R_1 I_1 = 9{,}091\cdot0{,}15 + 10{,}2 = 11{,}56\ \text{V}$$
> Proudový model: $G_\text{i} = 1/9{,}091 = 0{,}11\ \text{S}$, $I_\text{i} = 11{,}56/9{,}091 = 1{,}272\ \text{A}$.
>
> **Jak to říct, když se zeptají:** „Pro obě zatížení platí, že svorkové napětí je vnitřní napětí minus úbytek na vnitřním odporu. Dvě rovnice o dvou neznámých, odečtením vypadne $U_\text{i}$.“

> [!trap] Paralelně spojené rozdílné články
> Dva zdroje s různým vnitřním napětím spojené paralelně si i bez zátěže posílají vyrovnávací proud $I_\text{v} = (U_\text{i1} - U_\text{i2})/(R_\text{i1} + R_\text{i2})$ a slabší článek se „nabíjí“. Proto se nové a staré články nespojují. V sérii zase může slabší článek při velkém proudu obrátit polaritu svého svorkového napětí.

**Řízené zdroje** mají napětí nebo proud daný jinou obvodovou veličinou: zdroj napětí řízený napětím (ZNŘN), napětí řízený proudem (ZNŘP), proud řízený napětím (ZPŘN) a proud řízený proudem (ZPŘP). Slouží jako modely tranzistorů a zesilovačů, počítá se s nimi v kapitole **Cvičení: řízené zdroje a metoda razítek**.
