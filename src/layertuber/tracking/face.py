from ..vendor.OpenSeeFace.input_reader import InputReader
from ..vendor.OpenSeeFace.tracker import Tracker


class LayertubeTracker:
    reader: InputReader
    tracker: Tracker

    def __init__(self) -> None:
        self.reader = InputReader()
        self.tracker = Tracker()

    def loop(self) -> None:
        from time import sleep
        sleep(.1)
