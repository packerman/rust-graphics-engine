use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    application::{self, Application, AsyncCreator},
    camera::Camera,
    color::Color,
    convert::FromWithContext,
    extras::GridHelper,
    geometry::{Geometry, Rectangle},
    input::KeyState,
    material::{self, sprite::SpriteMaterial},
    matrix::Angle,
    mesh::Mesh,
    node::Node,
    renderer::{Renderer, RendererOptions},
    texture::{Texture, TextureData, TextureUnit},
    web,
};

struct Example {
    renderer: Renderer,
    scene: Rc<Node>,
    camera: Rc<RefCell<Camera>>,
    rig: Rc<Node>,
    sprite: Rc<Node>,
    tiles_per_second: f32,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Self> {
        let renderer = Renderer::new(
            context,
            RendererOptions {
                clear_color: Color::dark_slate_gray(),
                ..Default::default()
            },
        );
        let scene = Node::new_group();

        let camera = Rc::new(RefCell::new(Camera::default()));
        let rig = Node::new_movement_rig(Default::default());
        {
            rig.set_position(&glm::vec3(0.0, 0.5, 3.0));
            let camera = Node::new_camera(Rc::clone(&camera));
            rig.add_child(&camera);
            scene.add_child(&rig);
        }
        let sprite = create_sprite(context).await?;
        {
            scene.add_child(&sprite);
        }
        {
            let grid = Node::new_mesh(Box::new(Mesh::from_with_context(
                context,
                GridHelper::default(),
            )?));
            grid.rotate_x(-Angle::RIGHT, Default::default());
            scene.add_child(&grid);
        }
        Ok(Example {
            renderer,
            scene,
            camera,
            rig,
            sprite,
            tiles_per_second: 8.0,
        })
    }
}

impl Application for Example {
    fn update(&mut self, key_state: &KeyState) {
        self.rig.update(key_state);
        let tile_number = ((web::now().unwrap() as f32) * self.tiles_per_second / 1000.0).floor();
        if let Some(mesh) = self.sprite.mesh() {
            if let Some(uniform) = mesh.material().uniform("tileNumber") {
                if let Ok(data) = <&mut f32>::try_from(uniform.data_ref_mut().deref_mut()) {
                    *data = tile_number;
                }
            }
        }
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}

async fn create_sprite(context: &WebGl2RenderingContext) -> Result<Rc<Node>> {
    let geometry = Geometry::from_with_context(context, Rectangle::default())?;
    let tile_set = Texture::new(
        context,
        TextureData::load_from_source("images/rolling-ball.png").await?,
        Default::default(),
    )?;
    let material = Rc::new(material::sprite::create(
        context,
        tile_set,
        TextureUnit::from(0),
        SpriteMaterial {
            billboard: true,
            tile_count: glm::vec2(4.0, 4.0),
            tile_number: 0.0,
            ..Default::default()
        },
    )?);
    let sprite = Node::new_mesh(Box::new(Mesh::new(context, geometry, material)?));
    Ok(sprite)
}
