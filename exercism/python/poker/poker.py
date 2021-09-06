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
    hand_scores = []
    for shand in enumerate(hands):
        lhand = shand.split(" ")  # lhand = list hand
        # hands are scored in the same order in which they're given
        hand_scores.append(score_hand[lhand])

    highest_score = max(hand_scores)
    best_idxs = [i for i, s in enumerate(hand_scores) if s == highest_score]

    return [hands[i] for i in best_idxs]


def score_hand(hand):
    """
    Scores a single hand.
    """
    if is_pair(hand):
        if is_three_kind(hand):
            if is_four_kind(hand):
                return HandValues.FOUR_KIND
            else:
                return HandValues.THREE_KIND
        elif is_two_pair(hand):
            return HandValues.TWO_PAIR
        else:
            return HandValues.PAIR
    elif is_straight(hand):
        if is_flush(hand):
            return HandValues.STRAIGHT_FLUSH
        else:
            return HandValues.STRAIGHT
    elif is_flush(hand):
        return HandValues.FLUSH
    elif is_full_house(hand):
        return HandValues.FULL_HOUSE
    else:
        # the high card matters => gotta show that somehow (maybe with decimals?)
        return HandValues.HIGH_CARD


def is_pair(hand):
    ranks, _ = isolate_ranks_suits(hand)

    if len(ranks) - len(set(ranks)) == 1:
        # precisely one duplicate => one pair
        return True
    return False


def is_two_pair(hand):
    ranks, _ = isolate_ranks_suits(hand)

    ns_same_rank = [hand.count(rank) for rank in ranks]
    if ns_same_rank.count(2) == 2:
        # 2 twos => can only happen if we have two pairs
        return True

    return False


def is_three_kind(hand):
    ranks, _ = isolate_ranks_suits(hand)

    for rank in ranks:
        n_same_rank = hand.count(rank)
        if n_same_rank == 3:
            return True

    return False


def is_straight(hand):
    copy = list(hand)
    copy.sort()

    expected = list(range(copy[0], copy[-1]))
    if copy == expected:
        return True

    return False


def is_flush(hand):
    _, suits = isolate_ranks_suits(hand)

    if len(set(suits)) == 1:
        return True

    return False


def is_full_house(hand):
    # FIXME not sure about this one
    if is_pair(hand) and is_three_kind(hand):
        return True

    return False


def is_four_kind(hand):
    ranks, _ = isolate_ranks_suits(hand)

    for rank in ranks:
        n_same_rank = hand.count(rank)
        if n_same_rank == 4:
            return True

    return False


def isolate_ranks_suits(hand):
    """
    :returns: a tuple with the first element containing all the ranks and the second all the suits

    Isolates the ranks/suits of the cards.
    """
    ranks = []
    suits = []
    for card in hand:
        ranks.append(card[0])
        suits.append(card[1])

    return ranks, suits
