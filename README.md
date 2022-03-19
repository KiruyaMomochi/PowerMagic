# Power Calcluation

This is just an experiment.

$$
\renewcommand{\vec}[1]{\overrightarrow{#1}}
\newcommand{\rmvec}[1]{\overrightarrow{\mathrm{#1}}}
\newcommand{\round}{\operatorname{round}}
$$

## Basic 

### Unit

An unit have

1. Rank (promotion)
2. Level
3. Rarity (number of stars)

## Power Calculation

$$
\mathrm{power} = \left( \mathrm{skill} \cdot L + \rmvec{status} \cdot \vec{S} \right) ^ {C}
$$

## Skill level

Unit skill consists of 
1. `union_burst`: Union burst \
    Union burst is evolution if $\mathrm{rarity} \geq 6$.
2. `main_skill`: Main skill \
    Main skill is evolution if unique equipment is equipped.
3. `ex_skill`: Extra skill \
Ex skill is evolution if $\mathrm{rarity} \geq 5$.
4. `free_skill`: Free skill \
Free skill has no evolutions.

Ub and main skill level is calculated by
$$
\mathrm{skill}(l) = 
\begin{cases}
l \cdot \mathrm{slv} + \mathrm{evo}\quad&\text{if evolution},\\
l\quad&\text{otherwise}.
\end{cases}
$$

For extra skill level
$$
\mathrm{ex}(l) = 
\begin{cases}
l + \mathrm{evo}\quad&\text{if } l > 0 \text{ and rarity} \geq 5,\\
l\quad&\text{otherwise}.
\end{cases}
$$

For free skill level
$$
\mathrm{free}(l) = l
$$

Then
$$
\mathrm{skill} = \sum\mathrm{ub} + \sum\mathrm{main} + \sum\mathrm{ex} + \sum\mathrm{free}
$$


## Status

Unit status constists of

1. `hp`
1. `atk`
2. `def`
3. `magic_str`
4. `magic_def`
5. `physical_critical`
6. `magic_critical`
7. `wave_hp_recovery`
8. `wave_energy_recovery`
9. `hp_recovery_rate`
10. `physical_penetrate`
11. `magic_penetrate`
12. `life_steal`
13. `dodge`
14. `energy_reduce_rate`
15. `energy_recovery_rate`
16. `accuracy`

We define these status as a vector, then

$$
\rmvec{status} = \rmvec{base} + \rmvec{equip} + \rmvec{story}.
$$

### Basic Status

We define rank as $r$, level as $l$, rarity as $s$.

Rank up bonus
$$
\rmvec{rb}(r) = (\overbrace{r, r, r, r, r}^{5}, \overbrace{1, 1, \cdots, 1}^{12})
$$

Rarity parameter
$$
\rmvec{rarity}(s, l, r) = \rmvec{s.status} + l \cdot\rmvec{s.growth} + \rmvec{rb}(r) \circ \rmvec{s.growth} 
$$

Promotion (rank) parameter
$$
\rmvec{promotion}(r) = \rmvec{r.status} + \rmvec{r.bonus}
$$

Then
$$
\rmvec{base} = \round\left(\rmvec{rarity} + \rmvec{promotion}\right)
$$

### Equipment

Each rank level has 6 equipment slots, each slot $s$ has a equipment id. \
However, some slots may not be used, their id is `999999`.

Status of equipped equipment is
$$
\rmvec{rankequip}(e, l) = \rmvec{e.status} + \left\lceil l \cdot \rmvec{e.bonus} \right\rceil
$$

There are also one unique equipment $u$. \
Also a constant `enhance_lv_offset = -1`.

Status of equipped unique equipment is
$$
\rmvec{unique}(u, l) = \rmvec{u.status} + \left\lceil (l + \mathrm{u.offset}) \cdot \rmvec{u.enhance} \right\rceil
$$

If unit has rarity 6, then there are 3 slots.

1. Memory piece of unit
2. Pure memory piece of unit
3. Princess orb, which has 6 levels

Status of each slot is
$$
\rmvec{rarity6}(e, l) = \rmvec{e.status}(l)
$$

Then
$$
\rmvec{equip} = \round\left( \rmvec{rankequip} + \rmvec{unique} + \rmvec{rarity6} \right)
$$

### Bonus parameters

By reading story, we can get bonus parameters.
Each story may only add to some parameter.
