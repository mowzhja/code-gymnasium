def recite(start_verse, end_verse):
    """ Recite the verses from The Twelve Days of Christmas, starting at `start_verse` and ending at `end_verse`. """
    return [recite_single_verse(verse) for verse in range(start_verse, end_verse + 1)]


def recite_single_verse(verse):
    """ Recite the single verse at `verse`. """

    ordinal = {
        1: "first",
        2: "second",
        3: "third",
        4: "fourth",
        5: "fifth",
        6: "sixth",
        7: "seventh",
        8: "eighth",
        9: "ninth",
        10: "tenth",
        11: "eleventh",
        12: "twelfth",
    }

    incipit = f"On the {ordinal[verse]} day of Christmas my true love gave to me: "

    gifts = [
        "twelve Drummers Drumming, ",
        "eleven Pipers Piping, ",
        "ten Lords-a-Leaping, ",
        "nine Ladies Dancing, ",
        "eight Maids-a-Milking, ",
        "seven Swans-a-Swimming, ",
        "six Geese-a-Laying, ",
        "five Gold Rings, ",
        "four Calling Birds, ",
        "three French Hens, ",
        "two Turtle Doves, and ",
    ]

    ending = "a Partridge in a Pear Tree."

    if verse == 1:
        return incipit + ending

    return incipit + "".join(gifts[12 - verse:]) + ending
