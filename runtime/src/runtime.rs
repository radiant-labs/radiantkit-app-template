use crate::{RadiantMessage, RadiantNodeType, RadiantResponse};
use parking_lot::RwLockWriteGuard;
use radiantkit_core::{
    RadiantRectangleNode, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable,
    RectangleTool, Runtime, Vec3, View,
};
use radiantkit_image::{image_loader, RadiantImageNode};
use radiantkit_text::RadiantTextNode;
use radiantkit_winit::RadiantView;
use uuid::Uuid;

pub struct RadiantRuntime {
    pub view: RadiantView<RadiantMessage, RadiantNodeType>,
}

impl RadiantRuntime {
    pub async fn new(_client_id: u64, collaborate: bool, size: Option<Vec3>, padding: Vec3) -> Self {
        let mut view = RadiantView::new(size, padding).await;
        view.scene_mut().tool_manager.register_tool(
            1u32,
            Box::new(RectangleTool::new()),
        );
        if collaborate {
            // Todo: implement collaboration
        }
        Self { view }
    }
}

impl Runtime<'_, RadiantMessage, RadiantNodeType, RadiantResponse> for RadiantRuntime {
    type View = RadiantView<RadiantMessage, RadiantNodeType>;

    fn view(&self) -> &RadiantView<RadiantMessage, RadiantNodeType> {
        &self.view
    }

    fn view_mut(&mut self) -> &mut RadiantView<RadiantMessage, RadiantNodeType> {
        &mut self.view
    }

    fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::SceneMessage(message) => {
                let response = self.view.scene_mut().handle_message(message);
                if let Some(response) = response {
                    match response {
                        RadiantSceneResponse::Message { message } => {
                            return self.handle_message(message.into())
                        }
                        _ => return Some(response.into()),
                    }
                }
            }
            RadiantMessage::TextMessage(message) => {
                let id = message.id();
                let update_interactions;
                {
                    let mut scene = self.view.scene_mut();
                    let document = &mut scene.document;
                    let Some(mut document) = document.try_write() else {
                        return None;
                    };
                    let Some(node) = document.get_node_mut(id) else {
                        return None;
                    };
                    let Ok(mut text_node) = RwLockWriteGuard::try_map(node, |node| match node {
                        RadiantNodeType::Text(text_node) => { Some(text_node) },
                        _ => { None }
                    }) else {
                        return None;
                    };
                    update_interactions = text_node.handle_message(message);
                }
                if update_interactions {
                    return self
                        .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
                }
            }
            RadiantMessage::AddRectangle {
                id,
                position,
                scale,
            } => {
                let id = id.unwrap_or(Uuid::new_v4());
                let node = RadiantRectangleNode::new(id, position, scale);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
            RadiantMessage::AddImage { path, name } => {
                let screen_descriptor = self.view.scene().screen_descriptor;
                let texture_manager = self.view.scene_mut().texture_manager.clone();
                let document = self.view.scene_mut().document.clone();
                image_loader::load_image(path, move |response| {
                    let image = response
                        .unwrap_or(epaint::ColorImage::new([400, 100], epaint::Color32::RED));
                    let size = image.size;
                    if let Some(mut document) = document.try_write() {
                        let texture_handle =
                            texture_manager.load_texture(name, image, Default::default());
                        let id = Uuid::new_v4();
                        let mut node = RadiantImageNode::new(
                            id,
                            [100.0, 200.0],
                            [size[0] as f32, size[1] as f32],
                            texture_handle,
                        );
                        node.attach(&screen_descriptor);
                        document.add(node.into());
                    }
                });
            }
            RadiantMessage::AddText { text, position } => {
                let id = Uuid::new_v4();
                let node = RadiantTextNode::new(id, text, position, [100.0, 100.0]);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
        }
        None
    }
}
