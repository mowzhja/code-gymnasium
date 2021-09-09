from datetime import timedelta


def add(moment):
    """Adds a gigasecond (1 * 10^9 seconds) to the current moment in time."""
    return moment + timedelta(seconds=10**9)
