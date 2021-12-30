from __future__ import annotations

import logging
import os
from queue import Queue
from typing import List, Literal, Optional

import cv2

from scipy.spatial.transform import Rotation

from .report import TrackingReport
from .utils import average_of_2d_vectors, flip, px_to_center_offset_2d, subtract
from ..utils.cv import PINK, draw_dot_on_frame
from ..vendor.OpenSeeFace.input_reader import InputReader
from ..vendor.OpenSeeFace.tracker import FaceInfo, Tracker


logger = logging.getLogger('face tracking')

REQUEST_INPUT_WIDTH = 800
REQUEST_INPUT_HEIGHT = 600

TrackerControlEvent = Literal['calibrate', 'next_frame']
CALIBRATE: TrackerControlEvent = 'calibrate'
NEXT_FRAME: TrackerControlEvent = 'next_frame'


class FaceTracker:
    control_queue: Queue[TrackerControlEvent]
    report_queue: Queue[Optional[TrackingReport]]
    reader: InputReader
    tracker: Tracker
    height: int
    width: int
    neutral_report: TrackingReport

    def __init__(
        self, control_queue: Queue[TrackerControlEvent], report_queue: Queue[Optional[TrackingReport]], capture: int = 0
    ) -> None:
        self.control_queue = control_queue
        self.report_queue = report_queue
        self.reader = InputReader(
            capture=capture, raw_rgb=False, width=REQUEST_INPUT_WIDTH, height=REQUEST_INPUT_HEIGHT, fps=30
        )

        ret, frame = self.reader.read()
        if not ret:
            raise RuntimeError('video capture did not start')
        self.height, self.width, channels = frame.shape
        logger.debug(f'w: {self.width}, h: {self.height}, c: {channels}')

        self.tracker = Tracker(self.width, self.height, silent=True)
        self.calibrate()

    def calibrate(self) -> None:
        while True:
            report = self.get_raw_report()
            if report is not None:
                self.neutral_report = report
                break

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

    def get_raw_report(self) -> Optional[TrackingReport]:
        face = self.get_face()

        if face is None:
            return None

        size = (self.width, self.height)
        eye_blink = face.eye_blink or (1, 1)
        _blink, left_gaze_y, left_gaze_x, _confidence = face.eye_state[0]
        _blink, right_gaze_y, right_gaze_x, _confidence = face.eye_state[1]

        left_gaze = px_to_center_offset_2d((left_gaze_x, left_gaze_y), size)
        right_gaze = px_to_center_offset_2d((right_gaze_x, right_gaze_y), size)

        features = face.current_features

        return dict(
            floats=dict(
                blink_left=eye_blink[0],
                blink_right=eye_blink[1],
                blink=sum(eye_blink) / len(eye_blink),

                eyebrow_quirk_left=features['eyebrow_quirk_l'],
                eyebrow_quirk_right=features['eyebrow_quirk_r'],
                eyebrow_quirk=(features['eyebrow_quirk_l'] + features['eyebrow_quirk_r']) / 2,

                eyebrow_steepness_left=features['eyebrow_steepness_l'],
                eyebrow_steepness_right=features['eyebrow_steepness_r'],
                eyebrow_steepness=(features['eyebrow_steepness_l'] + features['eyebrow_steepness_r']) / 2,

                eyebrow_updown_left=features['eyebrow_updown_l'],
                eyebrow_updown_right=features['eyebrow_updown_r'],
                eyebrow_updown=(features['eyebrow_updown_l'] + features['eyebrow_updown_r']) / 2,

                mouth_open=features['mouth_open'],
                mouth_wide=features['mouth_wide'],
            ),
            rotations=dict(
                head_rotation=Rotation.from_quat(face.quaternion),
            ),
            vec2s=dict(
                face_position=px_to_center_offset_2d(flip(*face.coord), size),
                left_gaze=left_gaze,
                right_gaze=right_gaze,
                gaze=average_of_2d_vectors(left_gaze, right_gaze)
            ),
        )

    def get_report(self) -> Optional[TrackingReport]:
        report = self.get_raw_report()
        if report is None:
            return report

        for vk, vv in report['vec2s'].items():
            report['vec2s'][vk] = subtract(vv, self.neutral_report['vec2s'][vk])

        for rk, rv in report['rotations'].items():
            report['rotations'][rk] = (rv * self.neutral_report['rotations'][rk].inv())

        return report

    def begin_loop(self) -> None:
        while self.reader.is_open():
            event = self.control_queue.get()
            if event == CALIBRATE:
                self.calibrate()
            elif event == NEXT_FRAME:
                self.report_queue.put(self.get_report())
