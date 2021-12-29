import argparse
import logging
import os; os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = 'true'  # noqa
from queue import Queue
from threading import Thread
from typing import Optional

from .rig import Rig
from .tracking.face import FaceTracker, TrackerControlEvent
from .tracking.report import TrackingReport
from .viewer import Viewer


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
    parser.add_argument('-x', '--output-width', type=int, default=800)
    parser.add_argument('-y', '--output-height', type=int, default=600)
    parser.add_argument('rig_path')
    return parser.parse_args()


def main() -> None:
    args = _parse_args()

    report_queue: Queue[Optional[TrackingReport]] = Queue()
    tracker_event_queue: Queue[TrackerControlEvent] = Queue()

    def run_tracker() -> None:
        FaceTracker(tracker_event_queue, report_queue, capture=args.camera).begin_loop()

    tracker_process = Thread(target=run_tracker, daemon=True)
    tracker_process.start()

    def run_viewer() -> None:
        Viewer(
            Rig(args.rig_path, (args.width, args.height)),
            report_queue,
            tracker_event_queue,
        ).begin_loop()

    viewer_process = Thread(target=run_viewer)
    viewer_process.start()
    viewer_process.join()


if __name__ == '__main__':
    main()
