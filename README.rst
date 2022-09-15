``layertuber``
--------------

``layertuber`` is a vtuber puppet application with a focus on allowing anyone to very quickly create a simple but lively avatar for themselves, using `OpenSeeFace <https://github.com/emilianavt/OpenSeeFace>`_ to track your face with a webcam.

A ``layertuber`` rig is a combination of an `OpenRaster <https://www.openraster.org/>`_ drawing and a set of instructions tying each layer to some output of the OpenSeeFace process. Once stuff has settled down a bit, I'll lay out the options in this README; for now, the `demo puppets <puppets/>`_ and `Config struct <src/puppet/config/mod.rs>`_ will hopefully point you in the right direction.
