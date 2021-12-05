from .tracking.face import LayertubeTracker


tracker = LayertubeTracker()

while tracker.reader.is_open():
    tracker.loop()
