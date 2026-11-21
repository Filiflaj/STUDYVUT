+++
title = "Úloha 17: Ručkové měřicí přístroje"
topics = ["Úloha 17: ručkové přístroje"]
+++

**Úkol:** zobrazit na osciloskopu harmonické napětí (asi 25 V z transformátoru), napětí jednocestně a dvoucestně usměrněné a změřit je třemi ručkovými voltmetry. Z údaje každého voltmetru se dopočítá amplituda $U_\mathrm{M}$ a její chyba.

## Efektivní a střední hodnota {#hodnoty}

- **Efektivní hodnota** $U_\mathrm{ef}$: stejnosměrné napětí, které na rezistoru vyvine za periodu **stejné teplo** jako dané proměnné napětí.
- **Střední hodnota** $U_\mathrm{s}$: stejnosměrné napětí, které za periodu **přenese stejný náboj** (stejnosměrná složka). Střídavé napětí je takové, jehož střední hodnota je nulová.

$$U_\mathrm{ef} = \sqrt{\frac1T\int_0^T u^2(t)\,\mathrm{d}t} \qquad U_\mathrm{s} = \frac1T\int_0^T u(t)\,\mathrm{d}t$$

![Tři průběhy napětí](images/lab-17-prubehy.svg "Amplituda, efektivní a střední hodnota pro tři průběhy")

| průběh | $U_\mathrm{ef}$ | $U_\mathrm{s}$ |
|---|---|---|
| harmonický $U_\mathrm{M}\sin\omega t$ | $U_\mathrm{M}/\sqrt2$ | $0$ |
| jednocestně usměrněný | $U_\mathrm{M}/2$ | $U_\mathrm{M}/\pi$ |
| dvoucestně usměrněný | $U_\mathrm{M}/\sqrt2$ | $2U_\mathrm{M}/\pi$ |

Jednocestný usměrňovač (jedna dioda) „uřízne“ záporné půlvlny. Dvoucestný (Graetzův můstek, 4 diody) je překlopí nahoru. Na osciloskopu se odečítá amplituda nebo rozkmit $2U_\mathrm{M}$ (špička–špička, $U_\text{šš}$), hodnoty z osciloskopu jsou jen orientační.

## Měřicí soustavy {#soustavy}

| voltmetr | výchylka úměrná | stupnice | použití |
|---|---|---|---|
| magnetoelektrický (s otočnou cívkou) | střední hodnotě | $U_\mathrm{s}$ | jen stejnosměrné, pozor na polaritu; na střídavém ukazuje nulu |
| magnetoelektrický s usměrňovačem | střední hodnotě usměrněného napětí | **efektivní hodnota pro harmonický průběh** (asi 50 Hz) | správně jen pro harmonický a dvoucestně usměrněný průběh |
| elektromagnetický (feromagnetický) | efektivní hodnotě | $U_\mathrm{ef}$ | libovolný průběh |

> [!trap] Voltmetr s usměrňovačem a jednocestné napětí
> Stupnice je přepočtená z $U_\mathrm{s}$ na $U_\mathrm{ef}$ činitelem tvaru harmonického signálu $\pi/(2\sqrt2) \doteq 1{,}11$. U jednocestně usměrněného napětí je proto údaj špatně a musí se přepočítat:
> $$U_\mathrm{s} = \frac{2\sqrt2}{\pi}\,U_\text{měř} \doteq 0{,}90\,U_\text{měř} \qquad U_\mathrm{M} = 2\sqrt2\,U_\text{měř} \doteq 2{,}83\,U_\text{měř} \qquad U_\mathrm{ef} = \sqrt2\,U_\text{měř}$$

## Údaje na stupnici {#stupnice}

- **Konstanta rozsahu** $k$ = rozsah / počet dílků celé stupnice, zapisuje se jako **nezkrácený zlomek** (např. 60/120), aby šlo zpětně zjistit rozsah. Naměřené napětí $U = \alpha\cdot k$ ($\alpha$ výchylka v dílcích).
- **Třída přesnosti** $T_\mathrm{p}$ udává mezní chybu v procentech **rozsahu**:
$$\delta(U) = \frac{T_\mathrm{p}}{100}\,U_\mathrm{R} \qquad \delta_\mathrm{r}(U) = \frac{\delta(U)}{U_\text{měř}}\cdot100\ \%$$
Absolutní chyba je stejná na celém rozsahu, relativní klesá s výchylkou. Proto má být ručka v poslední třetině stupnice.
- **Vnitřní odpor voltmetru** je udaný v $\Omega/\mathrm{V}$ a násobí se **rozsahem** (ne údajem ručky): 1000 Ω/V na rozsahu 120 V dává 120 kΩ.
- **Voltmetr má mít co největší vnitřní odpor, ampérmetr co nejmenší** (aby měřený obvod co nejméně ovlivnily).
- Značky na stupnici: druh soustavy (otočná cívka, s usměrňovačem, feromagnetická), pracovní poloha (⊥ svisle, dále vodorovně nebo šikmo, např. ∠60°), druh proudu (stejnosměrný, střídavý, obojí) s třídou přesnosti, zkušební napětí (hvězdička: bez čísla 500 V, s číslem v kV, s nulou bez zkoušky).

## Zpracování {#zpracovani}

Pro každý průběh a voltmetr:

1. $U_\text{měř} = \alpha k$ a mezní chyba $\delta(U_\text{měř})$ z třídy přesnosti a rozsahu.
2. Podle druhu voltmetru je $U_\text{měř}$ buď $U_\mathrm{ef}$, nebo $U_\mathrm{s}$. Ostatní hodnoty a amplituda se dopočítají ze vztahů v tabulce výše.
3. Amplituda je nepřímo měřená veličina $U_\mathrm{M} = c\cdot U_\text{měř}$ (např. $c = \sqrt2$, $\pi$, $\pi/2$, $2{,}83$), proto $\delta(U_\mathrm{M}) = c\cdot\delta(U_\text{měř})$. **Relativní chyba amplitudy je stejná jako relativní chyba údaje voltmetru.**
4. Přehledná tabulka amplitud ze všech voltmetrů a osciloskopu, zhodnocení s ohledem na chyby a využití rozsahů.

> [!trap]
> Magnetoelektrický voltmetr bez usměrňovače na harmonické napětí nepřipojovat: ukáže nulu a hrozí poškození. Graetzův můstek nezapojit obráceně (vstup za výstup), obvodem by tekl téměř zkratový proud.

> [!example] Příklad z měření
> U harmonického napětí ukázal elektromagnetický voltmetr (třída 0,5) $U_\mathrm{ef} = (23{,}20 \pm 0{,}13)\ \mathrm{V}$, z toho $U_\mathrm{M} = \sqrt2\,U_\mathrm{ef} = (32{,}81 \pm 0{,}19)\ \mathrm{V}$. Relativní chyba je u obou stejná, asi 0,6 %. Jmenovitých 25 V transformátoru je jen orientační údaj.

## Na co se ptají ve vstupním testu {#test}

> [!note] Okruhy k testu
> - Ohmův zákon; teplota, teplotní stupnice ($T = t + 273{,}15$),
> - význam značek na přístrojích, odečítání ze stupnice, **konstanta rozsahu** z rozsahu a počtu dílků,
> - **vnitřní odpor** voltmetru na zvoleném rozsahu (Ω/V × rozsah),
> - **třída přesnosti**, mezní absolutní chyba $\delta = T_\mathrm{p}/100\cdot$ rozsah a relativní chyba,
> - definice a výpočet **efektivní a střední hodnoty**, hodnoty pro harmonický průběh ($U_\mathrm{ef} = U_\mathrm{M}/\sqrt2$, $U_\mathrm{s} = 0$),
> - jaký vnitřní odpor má mít ampérmetr a jaký voltmetr.
>
> Typické příklady: $U_\mathrm{M} = 2\ \mathrm{V} \Rightarrow U_\mathrm{ef} \doteq 1{,}41\ \mathrm{V}$; 100 Ω/V na rozsahu 60 V = 6 kΩ; rozsah 90 V, stupnice 30 dílků, výchylka 18 dílků = 54 V; třída 0,05 na rozsahu 200 mA = 0,10 mA.
