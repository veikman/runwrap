use runwrap;
use pretty_assertions::assert_eq;
use rstest::rstest;

/// Check that certain types of content are unaffected by all operations.
#[rstest]
#[case("")]
#[case(" ")]
#[case("  ")]
#[case("\n")]
#[case("\n\n")]
#[case("\n \n")]
#[case("\n\n\n")]
#[case("\n  \n\n ")]
#[case("So far had Douglas presented his picture when someone put a question.")]
#[case(r#"<?xml version="1.0" standalone='yes'?>"#)]
#[case(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>title</title>
    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>
  </head>
  <body>
    <!-- page content -->
  </body>
</html>"#)]  // HTML without intervening blank line.
#[case(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>title</title>
    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>
  </head>

  <body>
    <!-- page content -->
  </body>
</html>"#)]  // HTML with intervening blank line.
#[case(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <title>From chapter XVII</title>
  </head>
  <body>
    <p>He seemed to wonder; he smiled with the same loveliness. But he clearly gained time; he waited, he called for guidance. “Haven’t I?” It wasn’t for me to help him—it was for the thing I had met!</p>
    <p>Something in his tone and the expression of his face, as I got this from him, set my heart aching with such a pang as it had never yet known; so unutterably touching was it to see his little brain puzzled and his little resources taxed to play, under the spell laid on him, a part of innocence and consistency. “No, never—from the hour you came back. You’ve never mentioned to me one of your masters, one of your comrades, nor the least little thing that ever happened to you at school. Never, little Miles—no, never—have you given me an inkling of anything that may have happened there. Therefore you can fancy how much I’m in the dark. Until you came out, that way, this morning, you had, since the first hour I saw you, scarce even made a reference to anything in your previous life. You seemed so perfectly to accept the present.” It was extraordinary how my absolute conviction of his secret precocity (or whatever I might call the poison of an influence that I dared but half to phrase) made him, in spite of the faint breath of his inward trouble, appear as accessible as an older person—imposed him almost as an intellectual equal. “I thought you wanted to go on as you are.”</p>
  </body>
</html>"#)]  // HTML with long unwrapped paragraphs of text.
#[case(r#"“Well, then, finish it!”

```rust
#[case("He gazed up at me again. “Before what?”")]
```

“Before you came back. And before you went away.”"#)]  // Short fenced code block.
#[case(r#"
```
She had helplessly gloomed at the upper regions. “You leave him—?”

“So long with Quint? Yes—I don’t mind that now.”
```
"#)]  // Multiple short paragraphs in code block.
#[case(r#"
```
“She’s not alone, and at such times she’s not a child: she’s an old, old woman.” I scanned all the visible shore while Mrs. Grose took again, into the queer element I offered her, one of her plunges of submission; then I pointed out that the boat might perfectly be in a small refuge formed by one of the recesses of the pool, an indentation masked, for the hither side, by a projection of the bank and by a clump of trees growing close to the water.

“But if the boat’s there, where on earth’s she?” my colleague anxiously asked.
```
"#)]  // Multiple longer paragraphs in code block.
#[case("## History")]  // Short heading; “##” are read as indentation by textwrap alone.
#[case("## Night of the Day of the Dawn of the Son of the Bride of the Return of the Revenge of the Terror of the Attack of the Evil, Mutant, Alien, Flesh Eating, Hellbound, Zombified Living Dead Part 2: In Shocking 2-D")]  // Long heading
#[case("* “From you—from you!” she cried.")]  // Trivial bullet list.
#[test]
fn idempotent(#[case] oracle: &str) {
    assert_eq!(runwrap::wrap(oracle, 72), oracle);
    assert_eq!(runwrap::rewrap(oracle, 72), oracle);
    assert_eq!(runwrap::unwrap(oracle), oracle);
}

/// Check going there and back again.
#[rstest]
#[case(
"Oh, how I looked at her now! “And did you see anyone?”

“Ah, no!” she returned, almost with the full privilege of childish
inconsequence, resentfully, though with a long sweetness in her little
drawl of the negative.
",
"Oh, how I looked at her now! “And did you see anyone?”

“Ah, no!” she returned, almost with the full privilege of childish inconsequence, resentfully, though with a long sweetness in her little drawl of the negative.
")]
#[case(
"* I quickly rose, and I think I must have shown her a queerer face than
ever yet. “You see me asking him for a visit?” No, with her eyes on
my face she evidently couldn’t. Instead of it even—as a woman reads
another—she could see what I myself saw: his derision, his amusement,
his contempt for the breakdown of my resignation at being left alone
and for the fine machinery I had set in motion to attract his attention
to my slighted charms. She didn’t know—no one knew—how proud I had been
to serve him and to stick to our terms; yet she nonetheless took the
measure, I think, of the warning I now gave her. “If you should so lose
your head as to appeal to him for me—”

  She was really frightened. “Yes, miss?”
* “I would leave, on the spot, both him and you.”
",
"* I quickly rose, and I think I must have shown her a queerer face than ever yet. “You see me asking him for a visit?” No, with her eyes on my face she evidently couldn’t. Instead of it even—as a woman reads another—she could see what I myself saw: his derision, his amusement, his contempt for the breakdown of my resignation at being left alone and for the fine machinery I had set in motion to attract his attention to my slighted charms. She didn’t know—no one knew—how proud I had been to serve him and to stick to our terms; yet she nonetheless took the measure, I think, of the warning I now gave her. “If you should so lose your head as to appeal to him for me—”

  She was really frightened. “Yes, miss?”
* “I would leave, on the spot, both him and you.”
")]
fn twoway(#[case] wrapped: &str, #[case] unwrapped: &str) {
    assert_eq!(runwrap::wrap(wrapped, 72), wrapped);
    assert_eq!(runwrap::wrap(unwrapped, 72), wrapped);
    assert_eq!(runwrap::rewrap(wrapped, 72), wrapped);
    assert_eq!(runwrap::rewrap(unwrapped, 72), wrapped);
    assert_eq!(runwrap::unwrap(wrapped), unwrapped);
    assert_eq!(runwrap::unwrap(unwrapped), unwrapped);
}

#[test]
fn unwrap_textonly_1paragraph_long_preunwrapped() {
    const VAL: &str = "He had been left, by the death of their parents in India, guardian to a small nephew and a small niece, children of a younger, a military brother, whom he had lost two years before. These children were, by the strangest of chances for a man in his position—a lone man without the right sort of experience or a grain of patience—very heavily on his hands. It had all been a great worry and, on his own part doubtless, a series of blunders, but he immensely pitied the poor chicks and had done all he could; had in particular sent them down to his other house, the proper place for them being of course the country, and kept them there, from the first, with the best people he could find to look after them, parting even with his own servants to wait on them and going down himself, whenever he might, to see how they were doing. The awkward thing was that they had practically no other relations and that his own affairs took up all his time. He had put them in possession of Bly, which was healthy and secure, and had placed at the head of their little establishment—but below stairs only—an excellent woman, Mrs. Grose, whom he was sure his visitor would like and who had formerly been maid to his mother. She was now housekeeper and was also acting for the time as superintendent to the little girl, of whom, without children of her own, she was, by good luck, extremely fond. There were plenty of people to help, but of course the young lady who should go down as governess would be in supreme authority. She would also have, in holidays, to look after the small boy, who had been for a term at school—young as he was to be sent, but what else could be done?—and who, as the holidays were about to begin, would be back from one day to the other. There had been for the two children at first a young lady whom they had had the misfortune to lose. She had done for them quite beautifully—she was a most respectable person—till her death, the great awkwardness of which had, precisely, left no alternative but the school for little Miles. Mrs. Grose, since then, in the way of manners and things, had done as she could for Flora; and there were, further, a cook, a housemaid, a dairywoman, an old pony, an old groom, and an old gardener, all likewise thoroughly respectable.";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}
#[test]
fn unwrap_textonly_1paragraph_long_haphazard() {
    const UNWRAPPED: &str = "My sense of how he received this suffered for a minute from something that I can describe only as a fierce split of my attention—a stroke that at first, as I sprang straight up, reduced me to the mere blind movement of getting hold of him, drawing him close, and, while I just fell for support against the nearest piece of furniture, instinctively keeping him with his back to the window. The appearance was full upon us that I had already had to deal with here: Peter Quint had come into view like a sentinel before a prison. The next thing I saw was that, from outside, he had reached the window, and then I knew that, close to the glass and glaring in through it, he offered once more to the room his white face of damnation. It represents but grossly what took place within me at the sight to say that on the second my decision was made; yet I believe that no woman so overwhelmed ever in so short a time recovered her grasp of the act. It came to me in the very horror of the immediate presence that the act would be, seeing and facing what I saw and faced, to keep the boy himself unaware. The inspiration—I can call it by no other name—was that I felt how voluntarily, how transcendently, I might. It was like fighting with a demon for a human soul, and when I had fairly so appraised it I saw how the human soul—held out, in the tremor of my hands, at arm’s length—had a perfect dew of sweat on a lovely childish forehead. The face that was close to mine was as white as the face against the glass, and out of it presently came a sound, not low nor weak, but as if from much further away, that I drank like a waft of fragrance.";
    const WRAPPED: &str = "My sense of how he received this suffered for a minute from something that I can describe only as a fierce split of my attention—a stroke that at first, as I sprang straight up, reduced me to the mere blind movement of getting hold of him, drawing him close, and, while I just fell for support against the nearest piece of furniture, instinctively keeping him with his back to the window.
The appearance was full upon us that I had already had to deal with here:
Peter Quint had come into view like a sentinel before a prison.
The next thing I saw was that, from outside,
he had reached the window,
and then I knew that,
close to
the glass and glaring in through it, he offered once more to the room his white face of damnation. It represents but grossly what took place within me at the sight to say that on the second my decision was made; yet I believe that no woman so overwhelmed ever in so short a time recovered her grasp of the act. It came to me in the very horror
of
the immediate presence that the act would be, seeing and facing what I saw and faced, to keep the boy himself unaware. The inspiration—I can call it by no other name—was that I felt how voluntarily, how transcendently, I might. It was like fighting with a demon for a human soul, and when I had fairly so appraised it I saw how the human
soul—held out,
in
the
tremor of my hands, at arm’s length—had a perfect dew of sweat on a lovely childish forehead. The face that was close to mine was as white as the face against the glass, and out of it presently came a sound, not low nor weak, but as if from much further away, that I drank like a waft of
fragrance.";
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}

#[test]
fn unwrap_textonly_2paragraph_wrapped() {
    const UNWRAPPED: &str = "“The last governess? She was also young and pretty—almost as young and almost as pretty, miss, even as you.”

“Ah, then, I hope her youth and her beauty helped her!” I recollect throwing off. “He seems to like us young and pretty!”";
    const WRAPPED: &str = "“The\nlast governess?\nShe\nwas also young and pretty—almost as young and almost as pretty, miss, even as you.”

“Ah, then, I hope her youth and her beauty helped her!” I recollect throwing off.\n“He\nseems\nto like us young and pretty!”";
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}

// This behaviour is not implemented.
#[test]
#[ignore]
fn unwrap_markdown_list_unordered_without_subsequent_indent() {
    const UNWRAPPED: &str = "* “From you—from you!” she cried.";
    const WRAPPED: &str = "* “From you—from you!”\nshe cried.";
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}

// This behaviour is not implemented.
#[test]
#[ignore]
fn unwrap_markdown_list_unordered_with_subsequent_indent() {
    const UNWRAPPED: &str = "* “From you—from you!” she cried.";
    const WRAPPED: &str = "* “From you—from you!”\n  she cried.";
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}

#[test]
fn unwrap_mixed_preunwrapped() {
    // Adjusted for suboptimal non-treatment of single-paragraph list items.
    const UNWRAPPED: &str = r#"An HTML document should contain, at minimum:

<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <title>title</title>
    <link rel="stylesheet"
          href="nine_hundred_years_and_two_months_of_bikeshedding_and_cozy_chats.css">
    <script src="script.js"></script>
  </head>

  <body>
    ...
  </body>

</html>

### Details on each section

Observe:

* The stated “charset” is in part
  a fallback in the absence of an HTTP server header.
  * It’s useful when:
    * The server is broken.
    * There is no server.

      This is often the case in local development, as when you run a Flask-like project from the CLI.
 * If the “charset” is omitted from the HTML, old specifications say the default
   is ISO-8859-1, but user preference tends to win out,
   and now there’s UTF-8.



Three cheers for UTF-8!

"#;
    const WRAPPED: &str = r#"An HTML document should
contain, at minimum:

<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <title>title</title>
    <link rel="stylesheet"
          href="nine_hundred_years_and_two_months_of_bikeshedding_and_cozy_chats.css">
    <script src="script.js"></script>
  </head>

  <body>
    ...
  </body>

</html>

### Details on each section

Observe:

* The stated “charset” is in part
  a fallback in the absence of an HTTP server header.
  * It’s useful when:
    * The server is broken.
    * There is no server.

      This is often the case in local development,
      as when you run a Flask-like project from the CLI.
 * If the “charset” is omitted from the HTML, old specifications say the default
   is ISO-8859-1, but user preference tends to win out,
   and now there’s UTF-8.



Three cheers for UTF-8!

"#;
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}
