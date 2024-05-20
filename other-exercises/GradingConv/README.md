Future idea:




# Multiple evaluation and yet one average

I attend a double degree between Trento (TN/Tn) and Tuebingen (TUE/Tue), which use different grading systems. Besides I attended a double degree in my bachelor as well between Trieste and Ljubljana, where also different grading systems where used, and this might be a future expansion of suhc small project.

Notice that all universities I attended register only sufficient grades, and had often only a limited number of attempts to get such sufficient grade. Besides, I have no conversion table for insufficient grades between trento and tuebingen, and I will thus not implement such insufficient grades.

## Slovenian grading system
A valid Slovenian grade is a float between 1 and 10 (10 is the highest).
A sufficient Slovenian grade is a float between 6 and 10 (10 is the highest).

This system seems to me the most natural system in which to evaluate something for a civilization which uses the decimal system. It is not particularly smart, but simply it is not dumb. The natural question is: why adopting another system? There are likely many good and bad answers, but for sure many different cultures created many different evaluation systems (below we see a couple of these examples, and we did not even touch the american grading system). I should not criticise, because this just evidentiate my lack of understanding of their perspective. Still the temptation is too strong to avoid to do it.

## Italian grading system

A valid Italian grade is an integer between 1 and 30 (30 is the highest).
A sufficient Italian grade is an integer between 18 and 30 (30 is the highest).

The grading system of italy, though strange, claims to be the original one historically, when universities were born. The legend (I have never confirmed that this story is true) says that people were graded with a grade from 1 to 10, 10 being the highest. To have a fair evaluation each student was originally examined by not only one, but three professors, each giving his numerical judgment from 1 to 10. Since the sufficiency in a grade from 1 to 10 was considered the number 6 (today this usually means doing more than the half of the points in a written exam), then in the sum of the 3 professors' judgment the sufficiency was considered the number 18, the scaled version of it.

Even if atypical, every system which just scales the decimal system is still mathematically easy to approach. When you are not convinced by my claim, dive with me in the German system.

PS: Still Italy uses a particularly illogical system for final graduation notes: from 1 to 110 (wtf?). I guess it is made so that the 'excellence' is visually prized by a number superior to 100. Still not particularly smart to me.

## German grading system

A valid german grade is a value in [5.0, 4.7, 4.3, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0] (1 is the highest).

A sufficient german grade is a value in [4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0] (1 is the highest).

Notice that:
1. The order is inverted with respect to the mathematical ordering of numbers: 1 it's the heighest grade!
2. the numbers are floating point, but in the end only a finite discrete set of values is allowed.
3. Even though the idea behind the evaluation is the same as in the other countries (at least more than half of the points to get a sufficient grade, i.e. a '6' in the decimal system), the sufficiency does not start in the middle of the scale (2.5+epsilon), but at the edge of it (4.0).
4. The intervals between the values are not even: |1.0-1.3|=0.3 but |1.3-1.7|=0.4. I guess the origin is that who invented the system wanted to write 3 even jumps between n and n+1 including the extrema, but you cannot do it without getting periodical numbers, thus he rounded such numbers. Anyway this is not only a pure lack of aestetics, but has the important consequence of making the average not precise (it is stretched unevenly because the jumps are uneven, think if instead of teh values num.3 and num.7 we had num.1 and num.9).

This system is to me ***EXTREMELY*** dumb (and, because of observation 4, slightly unfair, even if not significantly). Both from a mathematical and practical perspective.

## Conversion
The conversion tables between the two systems are the ones given with the phf maps:

```
static INT_TUE2TN : phf::Map<i32, i32> = phf_map!{
    Tue::new_from_int(1) => 18
    Tue::new_from_int(2) => 19,
    Tue::new_from_int(3) => 21,
    Tue::new_from_int(4) => 22,
    Tue::new_from_int(5) => 23,
    Tue::new_from_int(6) => 24,
    Tue::new_from_int(7) => 26,
    Tue::new_from_int(8) => 27,
    Tue::new_from_int(9) => 29,
    Tue::new_from_int(10) => 30,
};

static TN2INT_TUE : phf::Map<i32, i32> = phf_map!{
    18 => Tue::new_from_int(1),
    19 => Tue::new_from_int(2),
    20 => Tue::new_from_int(3),
    21 => Tue::new_from_int(3),
    22 => Tue::new_from_int(4),
    23 => Tue::new_from_int(5),
    24 => Tue::new_from_int(6),
    25 => Tue::new_from_int(7),
    26 => Tue::new_from_int(7),
    27 => Tue::new_from_int(8),
    28 => Tue::new_from_int(9),
    29 => Tue::new_from_int(9),
    30 => Tue::new_from_int(10),
};
```

Notice that the dumbness of the german system implies that we do NOT HAVE ANYMORE a commutative diagram in the case of the calculation of the weighted average:

weighted_avg( weighted_avg(exams of tuebingen), weighted_avg(exams of trento converted into tuebingen) ) != weighted_avg( weighted_avg(exams of tuebingen converted into trento), weighted_avg(exams of trento) ) != weighted_avg( weighted_avg(exams of tuebingen converted into trento, exams of trento) )



## Calcolo del voto finale a Trento

REGOLAMENTO DELLA PROVA FINALE PER IL CONSEGUIMENTO DELLA LAUREA MAGISTRALE IN
MATEMATICA
Università degli Studi di Trento
Emanato con DR n. 1161 del 23 novembre 2023
Pagina 6 di 7
a. il punteggio di partenza di ogni Laureando/a è dato dalla media dei voti, ottenuti nelle attività
formative, convertita in centodecimi e arrotondata a due cifre decimali. La media è ottenuta utilizzando
come pesi i relativi crediti. La votazione 30 e Lode, nel calcolo della media dei voti, è valutata 31;
b. il punteggio di partenza può essere aumentato fino a 1 punto se il/la candidato/a ha svolto un:
i. programma di doppia laurea o titolo congiunto con altre Università;
ii. progetto Erasmus +, con conseguimento di almeno 18 crediti per semestre, o con eventuali attività
di tirocinio o tesi di almeno 12 crediti. In questo caso l’aumento è attribuito sulla base della
documentazione presentata da una Commissione formata dal Delegato per i rapporti internazionali
del Dipartimento e dal Coordinatore del Corso di Laurea Magistrale.
b. il punteggio attribuito sulla base delle lettere a) e b) del presente comma è aumentato o diminuito
assegnando un punteggio fra 0 e 5 punti al lavoro di tesi ed un punteggio fra -1 e 1 alla sua
presentazione
