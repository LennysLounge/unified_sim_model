use std::collections::HashMap;

use crate::{
    games::iracing::{IRacingCamera, IRacingResult},
    model,
};

use super::IRacingProcessor;

pub struct CameraProcessor {
    cameras: HashMap<model::Camera, IRacingCamera>,
}

impl CameraProcessor {
    pub fn new() -> Self {
        Self {
            cameras: HashMap::new(),
        }
    }

    pub fn get_camera_def(&self, camera: &model::Camera) -> Option<&IRacingCamera> {
        self.cameras.get(camera)
    }
}

impl IRacingProcessor for CameraProcessor {
    fn static_data(&mut self, context: &mut super::IRacingProcessorContext) -> IRacingResult<()> {
        self.cameras.clear();
        context.model.available_cameras.clear();
        for group_def in context.data.static_data.camera_info.groups.iter() {
            let Some(group_num) = group_def.group_num else {continue};
            let Some(ref group_name) = group_def.group_name else {continue};

            let iracing_camera = IRacingCamera {
                group_num,
                group_name: group_name.clone(),
                camera_num: 0,
                camera_name: "".to_string(),
            };

            let camera = match group_name.as_str() {
                "Cockpit" => model::Camera::FirstPerson,
                "Chopper" => model::Camera::Hellicopter,
                "Chase" => model::Camera::Chase,
                "TV1" => model::Camera::TV,
                _ => model::Camera::Game(model::GameCamera::IRacing(iracing_camera.clone())),
            };
            context.model.available_cameras.insert(camera.clone());

            self.cameras.insert(camera, iracing_camera);
        }
        Ok(())
    }

    fn live_data(&mut self, context: &mut super::IRacingProcessorContext) -> IRacingResult<()> {
        let Some(active_group_num) = context.data.live_data.cam_group_number else {return Ok(())};

        for (model_camera, iracing_camera) in self.cameras.iter() {
            if iracing_camera.group_num == active_group_num {
                context.model.active_camera.set(model_camera.clone());
                return Ok(());
            }
        }
        context.model.active_camera.set(model::Camera::None);

        Ok(())
    }

    fn event(
        &mut self,
        _context: &mut super::IRacingProcessorContext,
        _event: &model::Event,
    ) -> IRacingResult<()> {
        Ok(())
    }
}
