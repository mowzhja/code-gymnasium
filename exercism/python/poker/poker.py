from enum import Enum
from typing import Counter


class HandValues(Enum):
    HIGH_CARD = 3
    PAIR = 6
    TWO_PAIR = 9
    THREE_KIND = 12
    STRAIGHT = 15
    FLUSH = 18
    FULL_HOUSE = 21
    FOUR_KIND = 24
    STRAIGHT_FLUSH = 27


def best_hands(hands):
    hand_scores = []
    for hand in hands:
        # hands are scored in the same order in which they're given
        hand_scores.append(score_hand(hand))

    highest_score = max(hand_scores)
    best_idxs = [i for i, s in enumerate(hand_scores) if s == highest_score]

    # FIXME i don't deal with ties at all (all the failing test cases are ties)

    return [hands[i] for i in best_idxs]


def score_hand(hand):
    """
    Scores a single hand.
    """
    hand = hand.split(" ")  # make it into a list

    if is_pair(hand):
        if is_three_kind(hand):
            if is_four_kind(hand):
                return HandValues.FOUR_KIND.value
            else:
                return HandValues.THREE_KIND.value
        elif is_two_pair(hand):
            return HandValues.TWO_PAIR.value
        else:
            return HandValues.PAIR.value
    elif is_straight(hand):
        if is_flush(hand):
            return HandValues.STRAIGHT_FLUSH.value
        else:
            return HandValues.STRAIGHT.value
    elif is_flush(hand):
        return HandValues.FLUSH.value
    elif is_full_house(hand):
        return HandValues.FULL_HOUSE.value
    else:
        return HandValues.HIGH_CARD.value + high_card_value(hand)


def is_pair(hand):
    ranks, _ = isolate_ranks_suits(hand)

    if len(ranks) - len(set(ranks)) == 1:
        # precisely one duplicate => one pair
        return True
    return False


def is_two_pair(hand):
    ranks, _ = isolate_ranks_suits(hand)

    rank_counts = Counter(ranks).values()
    if list(rank_counts).count(2) == 2:
        # 2 twos => can only happen if we have two pairs
        return True

    return False


def is_three_kind(hand):
    ranks, _ = isolate_ranks_suits(hand)

    for rank in ranks:
        n_same_rank = ranks.count(rank)
        if n_same_rank == 3:
            return True

    return False


def is_straight(hand):
    ranks, _ = isolate_ranks_suits(hand)
    ranks = sorted([int(r) for r in ranks])

    expected = list(range(ranks[0], ranks[-1] + 1))
    if ranks == expected:
        return True

    return False


def is_flush(hand):
    _, suits = isolate_ranks_suits(hand)

    if len(set(suits)) == 1:
        return True

    return False


def is_full_house(hand):
    ranks, _ = isolate_ranks_suits(hand)
    if len(set(ranks)) == 2:
        return True

    return False


def is_four_kind(hand):
    ranks, _ = isolate_ranks_suits(hand)

    for rank in ranks:
        n_same_rank = ranks.count(rank)
        if n_same_rank == 4:
            return True

    return False


def high_card_value(hand):
    ranks, _ = isolate_ranks_suits(hand)
    ranks = [int(r) for r in ranks]

    high_card_rank = max(ranks)
    return high_card_rank * 0.1


def isolate_ranks_suits(hand):
    """
    :returns: a tuple with the first element containing all the ranks and the second all the suits

    Isolates the ranks/suits of the cards.
    """
    ranks = []
    suits = []
    for card in hand:
        rank = card[:-1]
        ranks.append(value_of(rank))
        suits.append(card[-1])

    return ranks, suits


def value_of(rank):
    """
    Returns the numeric value of a rank.
    """
    if rank not in ["J", "Q", "K", "A"]:
        return rank
    else:
        if rank == "J":
            return "11"
        if rank == "Q":
            return "12"
        if rank == "K":
            return "13"
        if rank == "A":
            return "1"
