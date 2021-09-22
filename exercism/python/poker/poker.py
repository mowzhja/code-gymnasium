from enum import Enum
from typing import Counter


def best_hands(hands: list[str]) -> list[str]:
    """Chooses the best poker hands out of a list of available ones."""
    # hand scores are given in the same order as the hands:
    # hand_scores[0] corresponds to the score of hands[0]
    # hand_scores[1] corresponds to the score of hands[1]
    # etc.
    hand_scores = [score_hand(hand) for hand in hands]

    best_idxs = [i for i, s in enumerate(hand_scores) if s == max(hand_scores)]
    return [hands[i] for i in best_idxs]


"""
SCORING
"""


class HandValues(Enum):
    # the intervals need to be large enough that increments cannot update a hand's value from one Enum option to the next
    HIGH_CARD = 10
    PAIR = 20
    TWO_PAIR = 30
    THREE_KIND = 40
    STRAIGHT = 50
    FLUSH = 60
    FULL_HOUSE = 70
    FOUR_KIND = 80
    STRAIGHT_FLUSH = 90


def score_hand(hand: list[str]) -> float:
    """Scores a single hand."""
    hand = hand.split(" ")

    if is_pair(hand):
        return HandValues.PAIR.value + linear_increment(hand)
    if is_two_pair(hand):
        return HandValues.TWO_PAIR.value + two_pair_increment(hand)
    if is_three_kind(hand):
        return HandValues.THREE_KIND.value + linear_increment(hand)
    # need to check for straight flush first
    if is_straight(hand) and is_flush(hand):
        return HandValues.STRAIGHT_FLUSH.value + 2 * straight_increment(hand)
    if is_straight(hand):
        return HandValues.STRAIGHT.value + straight_increment(hand)
    if is_flush(hand):
        return HandValues.FLUSH.value + linear_increment(hand)
    if is_full_house(hand):
        return HandValues.FULL_HOUSE.value + linear_increment(hand)
    if is_four_kind(hand):
        return HandValues.FOUR_KIND.value + linear_increment(hand)
    return HandValues.HIGH_CARD.value + high_card_increment(hand)


"""
CHECKS
"""


def is_pair(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)

    if len(ranks) - len(set(ranks)) == 1:
        # precisely one duplicate => one pair
        return True

    return False


def is_two_pair(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)
    rank_counts = Counter(ranks).values()

    if list(rank_counts).count(2) == 2:
        # 2 twos => can only happen if we have two pairs
        return True

    return False


def is_three_kind(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)

    if len(ranks) - len(set(ranks)) == 2:
        # precisely three duplicates
        return True

    return False


def is_straight(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)
    ranks.sort()

    if ranks[0] != -1:
        expected = list(range(ranks[0], ranks[-1] + 1))
        if ranks == expected:
            return True
    else:
        # got an ace
        expected = list(range(ranks[1], ranks[-1] + 1))
        if ranks[1:] == expected:
            return True

    return False


def is_flush(hand: list[str]) -> bool:
    _, suits = isolate_ranks_suits(hand)

    if len(set(suits)) == 1:
        return True

    return False


def is_full_house(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)
    rank_counter = Counter(ranks).values()

    if 3 in rank_counter and 2 in rank_counter:
        # precisely three of a kind + pair => full house
        return True

    return False


def is_four_kind(hand: list[str]) -> bool:
    ranks, _ = isolate_ranks_suits(hand)

    if len(ranks) - len(set(ranks)) == 3:
        # precisely four duplicates
        return True

    return False


"""
INCREMENTS
"""


def linear_increment(hand: list[str]) -> float:
    """Takes care of the increments in the most generic case."""
    ranks, _ = isolate_ranks_suits(hand)
    ranks.sort()

    s = 0
    for rank in ranks:
        # ace always has the highest value when calculating linear_increments
        s += rank * 0.1 if rank != -1 else 14 * 0.1
    return s


def two_pair_increment(hand: list[str]) -> float:
    ranks, _ = isolate_ranks_suits(hand)
    rank_counter = Counter(ranks)

    s = 0
    for k, v in rank_counter.items():
        if v == 2:
            # a pair => give more weight to the largest one
            s += k * 0.1 if k == max(rank_counter.keys()) else k * 0.01
        else:
            # the kicker weighs very little
            s += k * 0.001
    return s


def straight_increment(hand: list[str]) -> float:
    ranks, _ = isolate_ranks_suits(hand)
    ranks.sort()
    # ace has value 1 if lowest element of straight, else 14
    ranks[0] = 1 if ranks[1] == 2 else 14
    return sum(rank * 0.1 for rank in ranks)


def high_card_increment(hand: list[str]) -> float:
    ranks, _ = isolate_ranks_suits(hand)
    ranks.sort()
    ranks.reverse()

    s = 0
    for i, rank in enumerate(ranks):
        # the lower the rank, the lower the weight of the card
        s += rank * 10**(-(i + 1))
    return s


"""
UTILS
"""


def isolate_ranks_suits(hand: list[str]) -> tuple[list[int], list[chr]]:
    """Isolates the ranks/suits of the cards."""
    ranks = []
    suits = []

    for card in hand:
        rank = card[:-1]
        ranks.append(value_of(rank))
        suits.append(card[-1])
    return ranks, suits


def value_of(rank: chr) -> int:
    """Returns the numeric value of a rank."""
    if rank not in ["J", "Q", "K", "A"]:
        return int(rank)
    else:
        if rank == "J":
            return 11
        if rank == "Q":
            return 12
        if rank == "K":
            return 13
        if rank == "A":
            # handle highest/lowest duality as needed (aka elsewhere)
            return -1
