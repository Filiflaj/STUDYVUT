+++
title = "Jak na seminář: podmínky zápočtu a plán"
topics = ["Organizace předmětu"]
+++

Elektrotechnický seminář je určený hlavně studentům, kteří nepřišli z elektrotechnické střední školy. Doplňuje a rozvádí látku **Elektrotechniky 1 (BPC-EL1)** a většinou ji o kousek předbíhá: na semináři se metoda (dělič, MSP, MUN, Thévenin…) probere a procvičí dřív, než přijde na řadu v EL1. Výuka je 2 hodiny týdně a každé cvičení má část **laboratorní (LC)** a část **výpočetní (VC)**. Předmět končí **zápočtem**, zkouška ani body nejsou.

## Co je potřeba k zápočtu

> [!key] Zápočet = 6 výpočetních zadání + 5 laboratorních úloh
> Všechno musí být samostatně vypracované a zkontrolované vyučujícím. Poslední možná kontrola je ve **13. týdnu** semestru.

**Laboratorní úlohy (LC)**

- Každou úlohu je nutné osobně změřit, vyhodnotit a zpracovat.
- Záznam se píše do **sešitu formátu A4**: příprava, naměřené hodnoty, grafy, výpočty, výsledky a jejich zhodnocení.
- Kdo nezná základy měřené úlohy (z návodu), nesmí ji měřit. Na úlohu je tedy potřeba přijít připravený.
- Vypracovaná úloha se ukazuje ke kontrole **na dalším laboratorním cvičení**.
- Podle úvodní prezentace u osciloskopu (LC5) stačí úlohu naměřit.

**Výpočetní cvičení (VC)**

- Zadání příkladů dostaneš na cvičení, vypočítáš je během hodiny a odevzdáš ke kontrole. Co nestihneš, dokončíš doma; zkontrolované musí být nejpozději na další hodině.
- Nejasnosti se řeší hned s vyučujícím.

**Odpadlá výuka.** Když cvičení odpadne (třeba státní svátek), úloha se automaticky uzná a nenahrazuje se. Kdo chce téma přesto slyšet, může se podívat na záznam z loňska nebo přijít s jinou skupinou.

V podmínkách nejsou žádné písemky, body ani povinná docházka nad rámec toho, že úlohy LC je nutné osobně změřit.

> [!example] Kontrolní seznam na konec semestru
> - [ ] LC1, LC2, LC3, LC4 změřené, zpracované v sešitu a zkontrolované
> - [ ] LC5 (osciloskop) změřená
> - [ ] VC1 až VC6 vypočítané a zkontrolované
> - [ ] za odpadlá cvičení nic nedoháníš, jen si ověř, že je máš uznaná
> - [ ] všechno nejpozději ve 13. týdnu (7. 12. – 11. 12.)

## Plán semestru

Výuka probíhá v učebně SC2.86.

| Týden | Termín | Laboratorní cvičení | Výpočetní cvičení | Kapitola |
|---|---|---|---|---|
| 1 | 14. 9. – 18. 9. | úvod do laboratoří a elektrotechniky | výroba, přenos a rozvod energie | 01 |
| 2 | 21. 9. – 25. 9. | **LC1** Děliče napětí a proudu | | 02 |
| 3 | 28. 9. – 2. 10. | **LC2** Postupné zjednodušování | | 03 |
| 4 | 5. 10. – 9. 10. | **LC3** Princip superpozice | | 04 |
| 5 | 12. 10. – 16. 10. | | **VC1** Metoda smyčkových proudů | 05 |
| 6 | 19. 10. – 23. 10. | | **VC2** Metoda uzlových napětí | 06 |
| 7 | 26. 10. – 30. 10. | **LC4** Metoda náhradního zdroje | **VC3** Metoda náhradního zdroje | 07 |
| 8 | 2. 11. – 6. 11. | | **VC4** Nelineární prvky a obvody | 08 |
| 9 | 9. 11. – 13. 11. | | **VC5** MUN, metoda razítek; úvod do MicroCapu | 09 |
| 10 | 16. 11. – 20. 11. | **LC5** Práce s osciloskopem | | 10 |
| 11 | 23. 11. – 27. 11. | | **VC6** Magnetické obvody | 11 |
| 12 | 30. 11. – 4. 12. | opakování, diskuse | | |
| 13 | 7. 12. – 11. 12. | závěrečná kontrola, zápočet | | |

> [!note] Číslování v podkladech
> - Návod k úloze s osciloskopem je označený **LC6** (starší číslování). V letošním plánu je to **LC5**; jiný návod LC5 v podkladech není a kapitola **LC6: Práce s osciloskopem** odpovídá této úloze.
> - Zadání *Magnetické obvody* má v záhlaví **VC5** z dřívějšího roku; letos je to **VC6**. Jako VC5 se letos počítá *MUN, metoda razítek*. V aplikaci jsou kvízy pojmenované podle letošního plánu: `vc5` = razítka, `vc6` = magnetické obvody.

## Příprava na laboratorní úlohu

Do sešitu si doma připrav kostru záznamu:

1. název úlohy,
2. velmi stručný teoretický úvod (základní vzorce a definice, ne opis návodu),
3. schéma měření,
4. prázdné tabulky.

V laboratoři se do připravených tabulek doplní naměřené hodnoty, spočítá se, co se spočítat má, a nakonec se napíše **závěr**. Ke každé tabulce vypočtených hodnot patří **obecný vztah a jeden dosazený příklad** (pro jeden řádek).

**Tabulky** mají být přehledné, veličiny a jednotky podle SI. **Grafy** na milimetrový papír nebo z počítače: popsané osy (veličina a jednotka), nadpis, odlišené křivky; body se prokládají plynulou křivkou, ne lomenou čarou (u nespojitých funkcí se bod nespojitosti nespojuje).

**Relativní odchylka** podle prezentace semináře:

$$\delta_X = \frac{X_\text{M} - X_\text{S}}{X_\text{M}} \cdot 100\ \%$$

kde $X_\text{M}$ je hodnota naměřená přístrojem a $X_\text{S}$ správná (vypočtená) hodnota.

> [!trap] Jmenovatel odchylky
> Seminář dělí **naměřenou** hodnotou $X_\text{M}$. V laboratořích EL1 se dělí hodnotou teoretickou. U malých odchylek vyjde skoro totéž, ale vzorec v sešitu má odpovídat předmětu, pro který úlohu píšeš.

## Zapojování a bezpečnost

- Nejdřív se zapojí **proudový (sériový) obvod**: všechno, co je v sérii se zdrojem, včetně ampérmetrů.
- Potom **napěťové (paralelní) větve** s voltmetry.
- U stejnosměrných přístrojů hlídej polaritu (+ a COM).
- **Zdroj (vypnutý) se zapojuje jako poslední** a před zapnutím musí zapojení zkontrolovat vyučující.
- Voltmetr se připojuje **paralelně** k prvku, ampérmetr **do série**, ohmmetr jen na rezistor **odpojený od obvodu**.

Pro práci v laboratoři je potřeba kvalifikace podle § 4 nařízení vlády 194/2022 Sb.; tu studenti získají v rámci BPC-EL1. Ve 2. týdnu proběhne seznámení s laboratoří (provozní řád, nouzová tlačítka STOP, hasicí přístroj, telefon, lékárnička, AED na vrátnici) a podpis záznamu o seznámení.

Laboratorní řád stručně: čistá obuv, žádné vlastní instalace ani přenášení přístrojů, závady hlásit, pracovat jen na svém pracovišti, jídlo a pití jsou zakázané (čistou vodu v uzavřené lahvi může povolit vyučující), na konci vypnout zdroje a uvést pracoviště do původního stavu.

## Materiály

- **E-learning** předmětu: prezentace ze cvičení, zadání příkladů a laboratorních úloh, videozáznamy z roku 2020 (nejsou plnou náhradou výuky).
- **Skripta** *Elektrotechnický seminář* (vydání 2024, elektronicky); návody k úlohám se v prezentacích odkazují „do skript“.
- Sešit A4 na úlohy.

> [!note] Jak s aplikací
> Ke každému týdnu je kapitola s teorií a vzorovým výpočtem a kvíz. Kvízy `vc1` až `vc6` obsahují příklady z výpočetních cvičení i s mezivýsledky; sada **Výpočetní cvičení nanečisto** je míchá dohromady. Laboratorní úlohy mají vlastní kapitoly `lc…` a kvíz **Laboratorní úlohy**.
