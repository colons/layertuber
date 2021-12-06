import logging
import os
from typing import List, Optional

import cv2

from .model import TrackingReport
from ..utils.cv import PINK, draw_dot_on_frame
from ..vendor.OpenSeeFace.input_reader import InputReader
from ..vendor.OpenSeeFace.tracker import FaceInfo, Tracker


logger = logging.getLogger('face tracking')

REQUEST_INPUT_WIDTH = 800
REQUEST_INPUT_HEIGHT = 600


class FaceTracker:
    reader: InputReader
    tracker: Tracker
    height: int
    width: int

    def __init__(self) -> None:
        self.reader = InputReader(
            capture=0, raw_rgb=False, width=REQUEST_INPUT_WIDTH, height=REQUEST_INPUT_HEIGHT, fps=30
        )

        ret, frame = self.reader.read()
        if not ret:
            raise RuntimeError('video capture did not start')
        self.height, self.width, channels = frame.shape
        logger.debug(f'w: {self.width}, h: {self.height}, c: {channels}')

        self.tracker = Tracker(self.width, self.height, silent=True)

    def get_face(self) -> Optional[FaceInfo]:
        ret, input_frame = self.reader.read()

        if not ret:
            raise RuntimeError('video capture stopped')

        faces: List[FaceInfo] = self.tracker.predict(input_frame)

        if not faces:
            return None

        face, = faces  # we only allow one

        if os.environ.get('SHOW_FEATURES'):
            # `lms` here means `landmarks`
            for _part_number, (feature_x, feature_y, _c) in enumerate(face.lms):
                draw_dot_on_frame(input_frame, PINK, 2, feature_x, feature_y)

            cv2.imshow('first', input_frame)
            cv2.waitKey(1)

        return face

    def get_report(self) -> Optional[TrackingReport]:
        face = self.get_face()

        if face is None:
            return None

        return dict(
            floats=dict(
                left_blink=face.eye_blink[0],
                right_blink=face.eye_blink[1],
            )
        )
