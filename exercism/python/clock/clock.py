from datetime import time


class Clock:
    def __init__(self, hour, minute):
        hour += minute // 60
        # (zeit == time, in german)
        self.zeit = time(hour % 24, minute % 60)

    def __repr__(self):
        # set resolution to minutes
        return self.zeit.isoformat(timespec="minutes")

    def __eq__(self, other):
        # datetime.time() objects are already comparable
        return True if self.zeit == other.zeit else False

    def __add__(self, minutes):
        return Clock(self.zeit.hour, self.zeit.minute + minutes)

    def __sub__(self, minutes):
        return Clock(self.zeit.hour, self.zeit.minute - minutes)
