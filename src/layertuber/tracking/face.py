from ..vendor.OpenSeeFace.input_reader import InputReader
from ..vendor.OpenSeeFace.tracker import Tracker


class LayertubeTracker:
    reader: InputReader
    tracker: Tracker

    def __init__(self) -> None:
        width = 800
        height = 600
        self.reader = InputReader(capture='0', raw_rgb=False, width=width, height=height, fps=30)
        self.tracker = Tracker(width, height)

    def loop(self) -> None:
        print('looping')
        from time import sleep
        sleep(.2)
