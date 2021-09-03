from enum import Enum


class HandValues(Enum):
    HIGH_CARD = 3
    PAIR = HIGH_CARD + 3
    TWO_PAIR = PAIR + 3
    THREE_KIND = TWO_PAIR + 3
    STRAIGHT = THREE_KIND + 3
    FLUSH = STRAIGHT + 3
    FULL_HOUSE = FLUSH + 3
    FOUR_KIND = FULL_HOUSE + 3
    STRAIGHT_FLUSH = FOUR_KIND + 3


def best_hands(hands):
    pass


def score_hand(hand):
    """
    Scores a single hand.
    """
    hand_value = 0

    if is_pair(hand):
        hand_value += HandValues.PAIR
    elif is_two_pair(hand):
        hand_value += HandValues.TWO_PAIR
    # etc...

    # not the prettiest way, but it should work
