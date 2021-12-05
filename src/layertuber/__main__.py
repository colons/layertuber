from .tracking.face import LayertubeTracker


tracker = LayertubeTracker()

while True:
    tracker.loop()
