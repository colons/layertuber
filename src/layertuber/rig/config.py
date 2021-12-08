from __future__ import annotations

import logging
from typing import Dict, Optional

from pydantic import BaseModel

from ..tracking.report import FloatFromTrackingReport, RotationFromTrackingReport, Vec2FromTrackingReport


logger = logging.getLogger('config')


class ThresholdConfig(BaseModel):
    option: FloatFromTrackingReport
    greater_than: float


class Vec2ScaledConfig(BaseModel):
    option: Vec2FromTrackingReport
    scale: float = 1


class ScalarQuatConfig(BaseModel):
    option: RotationFromTrackingReport
    scale: float = 1


class LayerConfig(BaseModel):
    visible: bool = True
    visible_when: Optional[ThresholdConfig]
    invisible_when: Optional[ThresholdConfig]
    follow: Optional[Vec2ScaledConfig]
    follow_facing_point: Optional[ScalarQuatConfig]


class RigConfig(BaseModel):
    layers: Dict[str, LayerConfig]
