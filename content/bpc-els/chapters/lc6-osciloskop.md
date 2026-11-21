+++
title = "LC6 (v plánu LC5): Práce s osciloskopem"
topics = ["LC6: Práce s osciloskopem"]
+++

Laboratorní úloha s osciloskopem (10. týden). Návod je označený **LC6**, v letošním plánu je to **LC5**. Podle úvodní prezentace ji ke kontrole stačí naměřit. Cílem je naučit se se základními funkcemi číslicového osciloskopu: zobrazit signály z generátoru a změřit jejich napěťové a časové parametry automaticky i kurzory. Teorie je v kapitole **10 Práce s osciloskopem**.

**Přístroje:** číslicový osciloskop SIGLENT SDS 1102X+, sonda, nekovový šroubovák na kompenzaci sondy, generátor průběhů (sinus, trojúhelník, obdélník) napájený z 12 V DC.

## Postup

1. **Napájení generátoru:** multimetrem nastavit zdroj na stole na 12 V DC a připojit generátor. Přepínač rozsahu kmitočtů na generátoru do polohy B.
2. **Výchozí stav:** zapnout osciloskop a stisknout **Default**.
3. **Poměr sondy:** na hrotu sondy přepnout **10×** a v menu kanálu 1 nastavit **Probe: 10×**. Obě nastavení musí souhlasit.
4. **Kompenzace sondy:** sondu do BNC vstupu, hrot na kalibrační výstup (Cal), **Auto Setup** a trimrem na sondě dostavit dokonalý obdélník (ne překmity, ne zaoblené hrany).
5. **Připojení k generátoru:** zemnicí krokosvorka sondy na GND generátoru, hrot na sinusový výstup, **Auto Setup**.
6. **Vazba AC:** v menu kanálu **Coupling: AC** odstraní stejnosměrnou složku signálu z generátoru; znovu Auto Setup. Měřítka pak jde doladit ručně.
7. **Vyzkoušet ovládání:** Vertical Position (svislý posun, stiskem na 0), Vertical Variable (V/dílek, stiskem hrubě/jemně), Horizontal Position (vodorovný posun, stiskem na 0), Horizontal Scale (s/dílek, stiskem zoom).
8. **Napěťové parametry:** Measure → Type, vybrat **Pk-Pk** ($U_\text{pp}$), **Ampl** ($U_\text{amp}$), **Mean** ($U_\text{avg}$), **RMS** ($U_\text{rms}$). Zapsat pro sinus, pak pro trojúhelník a obdélník (tab. 6.1).
9. **Rozsah kmitočtu:** na sinusu měřit **Freq** a **Prd**; potenciometr generátoru na minimum dá $f_\text{min}$ a $T_\text{max}$; přepínač do polohy A a potenciometr na maximum dá $f_\text{max}$ a $T_\text{min}$. Totéž pro trojúhelník (tab. 6.2).
10. **Hrany trojúhelníku:** nastavit kmitočet 1 kHz (měřený osciloskopem) a změřit **Rise** a **Fall**.
11. **Obdélník automaticky:** Freq, Prd, Rise, Fall, +Duty do řádku „automaticky“ (tab. 6.3).
12. **Obdélník kurzory:** Cursor → Mode → Manual; svislými kurzory X1, X2 odměřit periodu, hrany (mezi 10 % a 90 %) a šířku impulzu (v 50 %), vodorovnými Y úrovně. Zapsat do řádku „kurzory“.

## Tabulky

| tab. 6.1 | $U_\text{pp}$ (V) | $U_\text{amp}$ (V) | $U_\text{avg}$ (V) | $U_\text{rms}$ (V) |
|---|---|---|---|---|
| sinus | | | | |
| trojúhelník | | | | |
| obdélník | | | | |

| tab. 6.2 | $f_\text{min}$ (Hz) | $f_\text{max}$ (Hz) | $T_\text{min}$ (s) | $T_\text{max}$ (s) | $t_\text{rise}$ (s) | $t_\text{fall}$ (s) |
|---|---|---|---|---|---|---|
| sinus | | | | | – | – |
| trojúhelník | | | | | | |

| tab. 6.3 obdélník | $f$ (Hz) | $T$ (s) | $t_\text{rise}$ (s) | $t_\text{fall}$ (s) | střída (%) |
|---|---|---|---|---|---|
| automaticky | | | | | |
| kurzory | | | | | |

## Jak zkontrolovat, že čísla dávají smysl

S vazbou AC (bez stejnosměrné složky) a s amplitudou $U_\text{m} = U_\text{pp}/2$:

| Průběh | $U_\text{amp}$ | $U_\text{avg}$ | $U_\text{rms}$ |
|---|---|---|---|
| sinus | ≈ $U_\text{pp}$ | ≈ 0 | $U_\text{pp}/(2\sqrt{2}) \approx 0{,}354\,U_\text{pp}$ |
| trojúhelník | ≈ $U_\text{pp}$ | ≈ 0 | $U_\text{pp}/(2\sqrt{3}) \approx 0{,}289\,U_\text{pp}$ |
| obdélník (střída 50 %) | $U_\text{top} - U_\text{base}$ | ≈ 0 | $U_\text{pp}/2$ |

$U_\text{amp}$ (Ampl) je u osciloskopu rozdíl $U_\text{top} - U_\text{base}$, tedy u sinusu skoro celý rozkmit, ne polovina.

> [!example] Kontrola naměřených hodnot
> Na sinusu změříš $U_\text{pp}$ = 2,40 V. Efektivní hodnota má vyjít asi $2{,}40/(2\sqrt2) = 2{,}40/2{,}828 = 0{,}849$ V. Když osciloskop ukáže 8,49 V, je špatně poměr sondy (10× jen na jedné straně).
>
> U obdélníku s periodou 1 ms a kladným impulzem 0,5 ms je střída $0{,}5/1 = 50\ \%$; automatické měření a kurzory by se měly lišit jen o přesnost nastavení kurzorů.
>
> **Jak to říct, když se zeptají:** „Kompenzací sondy se vyrovnají parazitní kapacity sondy a kabelu; bez ní by obdélník měl překmity nebo zaoblené hrany a doby hran by byly špatně. Doba hrany je mezi 10 % a 90 % amplitudy, šířka impulzu v polovině výšky.“

## Závěr

- Co vše jde osciloskopem zobrazit a měřit: časový průběh napětí (a přes převodník i jiných veličin), periodu a kmitočet, rozkmit, střední a efektivní hodnotu, doby hran, šířku impulzu a střídu, u DSO i spektrum (FFT) a jednorázové děje.
- Porovnat hodnoty z tab. 6.3 změřené automaticky a kurzory a vysvětlit rozdíly (nepřesné nastavení kurzorů na úrovně 10 %, 50 %, 90 %, rozlišení časové osy).
