import argparse
import logging
import os
from queue import Queue
from threading import Thread
from typing import Optional


os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = 'true'

logging.basicConfig(
    level=os.environ.get('LOGLEVEL', 'INFO').upper(),
    style='{',
    format='{name}: {message}'
)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--camera', type=int, default=0, help=(
        'The index of the camera to use. If your computer has only one webcam, you can leave this at its default 0.'
    ))
    parser.add_argument('--show-features', action='store_true', help=(
        'Show an additional window with your webcam feed and facial feature detection spots overlaid on it.'
    ))
    return parser.parse_args()


def main() -> None:
    from .tracking.face import FaceTracker, TrackerControlEvent
    from .tracking.report import TrackingReport
    from .reporter import Reporter

    args = _parse_args()

    report_queue: Queue[Optional[TrackingReport]] = Queue()
    tracker_event_queue: Queue[TrackerControlEvent] = Queue()

    def run_tracker() -> None:
        FaceTracker(
            tracker_event_queue, report_queue, capture=args.camera, show_features=args.show_features
        ).begin_loop()

    tracker_process = Thread(target=run_tracker, daemon=True)
    tracker_process.start()

    def run_reporter() -> None:
        Reporter(report_queue, tracker_event_queue).begin_loop()

    reporter_process = Thread(target=run_reporter)
    reporter_process.start()
    reporter_process.join()


if __name__ == '__main__':
    main()
