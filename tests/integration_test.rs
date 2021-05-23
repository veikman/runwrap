use runwrap;

#[test]
fn unwrap_empty() {
    const VAL: &str = "";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_space() {
    const VAL: &str = " ";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_1newline() {
    const VAL: &str = "\n";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_2newline() {
    const VAL: &str = "\n\n";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_3newline() {
    const VAL: &str = "\n\n\n";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_textonly_1paragraph_1sentence_preunwrapped() {
    const VAL: &str = "So far had Douglas presented his picture when someone put a question.";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_textonly_1paragraph_long_preunwrapped() {
    const VAL: &str = "He had been left, by the death of their parents in India, guardian to a small nephew and a small niece, children of a younger, a military brother, whom he had lost two years before. These children were, by the strangest of chances for a man in his position—a lone man without the right sort of experience or a grain of patience—very heavily on his hands. It had all been a great worry and, on his own part doubtless, a series of blunders, but he immensely pitied the poor chicks and had done all he could; had in particular sent them down to his other house, the proper place for them being of course the country, and kept them there, from the first, with the best people he could find to look after them, parting even with his own servants to wait on them and going down himself, whenever he might, to see how they were doing. The awkward thing was that they had practically no other relations and that his own affairs took up all his time. He had put them in possession of Bly, which was healthy and secure, and had placed at the head of their little establishment—but below stairs only—an excellent woman, Mrs. Grose, whom he was sure his visitor would like and who had formerly been maid to his mother. She was now housekeeper and was also acting for the time as superintendent to the little girl, of whom, without children of her own, she was, by good luck, extremely fond. There were plenty of people to help, but of course the young lady who should go down as governess would be in supreme authority. She would also have, in holidays, to look after the small boy, who had been for a term at school—young as he was to be sent, but what else could be done?—and who, as the holidays were about to begin, would be back from one day to the other. There had been for the two children at first a young lady whom they had had the misfortune to lose. She had done for them quite beautifully—she was a most respectable person—till her death, the great awkwardness of which had, precisely, left no alternative but the school for little Miles. Mrs. Grose, since then, in the way of manners and things, had done as she could for Flora; and there were, further, a cook, a housemaid, a dairywoman, an old pony, an old groom, and an old gardener, all likewise thoroughly respectable.";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_textonly_2paragraph_preunwrapped() {
    const VAL: &str = "I can still see Mrs. Grose’s broad face as she took this in. “In Harley Street?”

“In Harley Street.”";
    assert_eq!(VAL, runwrap::unwrap(VAL));
}

#[test]
fn unwrap_textonly_2paragraph_wrapped() {
    const UNWRAPPED: &str = "“The last governess? She was also young and pretty—almost as young and almost as pretty, miss, even as you.”

“Ah, then, I hope her youth and her beauty helped her!” I recollect throwing off. “He seems to like us young and pretty!”";
    const WRAPPED: &str = "“The\nlast governess?\nShe\nwas also young and pretty—almost as young and almost as pretty, miss, even as you.”

“Ah, then, I hope her youth and her beauty helped her!” I recollect throwing off.\n“He\nseems\nto like us young and pretty!”";
    assert_eq!(UNWRAPPED, runwrap::unwrap(WRAPPED));
}
