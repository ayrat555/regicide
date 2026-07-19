pub fn print_help_full() {
    println!(
        r#"
COMMANDS
  play 1        Play card #1
  play 1 3      Play animal companion + card, or a combo
  1 3           Shorthand for play
  yield         Skip attack; take enemy damage
  jester        Solo only: discard hand & refill (2 per game)
  save          Write save file
  quit          Exit (prompts to save)
  help          This text

Type `rules` from the main menu for the full rulebook summary.
"#
    );
    print_rules_short();
}

pub fn print_rules() {
    println!(
        r#"
════════════════════════════════════════
              REGICIDE RULES
════════════════════════════════════════

AIM
  Work together to defeat 12 enemies: 4 Jacks,
  then 4 Queens, then 4 Kings. Win by killing
  the last King. Lose if anyone cannot discard
  enough to survive an enemy attack, or cannot
  play a card or yield.

SETUP
  Castle deck: shuffled Jacks on Queens on Kings.
  Tavern deck: 2–10 in all suits, 4 Animal
  Companions (A), plus Jesters by player count.
  Discard starts empty. Deal to max hand size.

  Players  Jesters  Hand size
     1        0*       8
     2        0        7
     3        1        6
     4        2        5
  * Solo: 2 side Jesters (not in the Tavern).

TURN (4 STEPS)
  1. Play card(s) or yield
  2. Activate suit power(s)
  3. Deal damage; if enemy dies, skip step 4
     and the same player starts a new turn
  4. Suffer enemy attack (discard to cover)

PLAYING CARDS
  • One card: damage = its value
  • Animal (A, value 1): alone, or paired with
    one other non-Jester card (powers of both)
  • Combo: 2–4 cards of the same number with
    total ≤10 (e.g. pair of 5s, triple 3s)
  • Jester (*): alone, value 0 — cancels the
    enemy's suit immunity; skip damage & attack;
    choose who goes next

SUIT POWERS (use the play's total attack value)
  ♥ Hearts   Move that many cards from the
             shuffled discard under the Tavern
  ♦ Diamonds Draw that many cards (skip full hands)
  ♣ Clubs    Double damage this turn
  ♠ Spades   Reduce this enemy's attack by that
             much (stacks until the enemy dies)

  Same suit as the enemy → that power is blocked
  (Jester removes the block). Same suit twice in
  one play still only applies once.

ENEMIES
  Jack   ATK 10   HP 20
  Queen  ATK 15   HP 30
  King   ATK 20   HP 40

DEFEATING AN ENEMY
  Damage is cumulative across turns.
  Exact kill → enemy goes on top of the Tavern
  (can be drawn later: J=10, Q=15, K=20).
  Overkill → enemy goes to the discard.
  Cards played against them go to discard; next
  Castle card is revealed.

DEFENDING
  Discard cards whose values sum to at least the
  enemy's current attack (base − spade shield).
  A = 1, Jester = 0 when discarded. Empty hand
  is fine if attack is 0.

YIELDING
  Skip steps 2–3 and go to defend. You may not
  yield if every other player yielded on their
  last turn (solo: always allowed).

SOLO JESTERS
  Twice per game, discard your hand and refill
  to 8 (not a Diamond draw). Use at the start of
  step 1 or before defending.
  Win with 0 / 1 / 2 used → Gold / Silver / Bronze.

Official rules: badgersfrommars.com / regicidegame.com
"#
    );
}

pub fn print_rules_short() {
    println!(
        r#"PLAY RULES (short)
  • Single card, or animal (A) paired with one other card
  • Combos: 2–4 of the same number totaling ≤10
  • Jester (*) alone: cancel immunity, pick next player
  • Exact enemy kill → enemy on top of the Tavern; else → discard
  • Spades shield stacks until the enemy dies
  • Matching the enemy's suit blocks that suit's power

ENEMY STATS
  Jack  ATK 10 HP 20 | Queen ATK 15 HP 30 | King ATK 20 HP 40
"#
    );
}
