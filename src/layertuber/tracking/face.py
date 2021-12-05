import logging
import os
from typing import List

import cv2

from .utils import draw_dot_on_frame
from ..vendor.OpenSeeFace.input_reader import InputReader
from ..vendor.OpenSeeFace.tracker import FaceInfo, Tracker


logger = logging.getLogger('face tracking')

PINK = (0x66, 0x66, 0xdd)
GREEN = (0x66, 0xdd, 0x66)

REQUEST_INPUT_WIDTH = 800
REQUEST_INPUT_HEIGHT = 600


class LayertubeTracker:
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

    def loop(self) -> FaceInfo:
        ret, input_frame = self.reader.read()

        if not ret:
            raise RuntimeError('video capture stopped')

        faces: List[FaceInfo] = self.tracker.predict(input_frame)

        if not faces:
            return

        face, = faces  # we only allow one

        if os.environ.get('SHOW_FEATURES'):
            # `lms` here means `landmarks`
            for _part_number, (feature_x, feature_y, _c) in enumerate(face.lms):
                draw_dot_on_frame(input_frame, PINK, 2, feature_x, feature_y)

            cv2.imshow('first', input_frame)
            cv2.waitKey(1)

        return FaceInfo
