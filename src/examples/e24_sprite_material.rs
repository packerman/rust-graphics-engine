use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use glm::vec2;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::Geometry,
    base::{
        application::{self, Application, AsyncCreator},
        color,
        convert::FromWithContext,
        input::KeyState,
        math::angle::Angle,
        util::shared_ref::{self, SharedRef},
        web,
    },
    classic::renderer::{Renderer, RendererOptions},
    core::{
        camera::{Camera, Perspective},
        material::Material,
        mesh::Mesh,
        node::Node,
        scene::Scene,
        texture::{Texture, TextureUnit},
    },
    extras::{camera_controller::CameraController, grid_helper::GridHelper},
    geometry::rectangle::Rectangle,
    material::{
        self,
        sprite::{Properties, SpriteMaterial},
    },
};

struct Example {
    renderer: Renderer,
    scene: Scene,
    camera: SharedRef<Camera>,
    controller: CameraController,
    sprite_material: SharedRef<SpriteMaterial>,
    tiles_per_second: f32,
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let renderer = Renderer::initialize(
            context,
            RendererOptions {
                clear_color: color::dark_slate_gray(),
                ..Default::default()
            },
            None,
        );
        let mut scene = Scene::new_empty();

        let camera = Camera::new(Perspective::default());
        {
            let camera = Node::new_with_camera(Rc::clone(&camera));
            camera.borrow_mut().set_position(&glm::vec3(0.0, 0.5, 3.0));
            scene.add_node(camera);
        }
        let controller =
            CameraController::make_for_camera(&camera).expect("Camera controller is created");
        let sprite_material = create_sprite_material(
            context,
            Properties {
                billboard: true,
                tile_count: vec2(4.0, 4.0),
                tile_number: 0.0,
                ..Default::default()
            },
        )
        .await?;
        let sprite = create_sprite(context, Rc::clone(&sprite_material)).await?;
        {
            scene.add_node(sprite);
        }
        {
            let grid = Node::new_with_mesh(<Rc<Mesh>>::from_with_context(
                context,
                GridHelper::default(),
            )?);
            grid.borrow_mut().rotate_x(-Angle::RIGHT);
            scene.add_node(grid);
        }
        Ok(Box::new(Example {
            renderer,
            scene,
            camera,
            controller,
            sprite_material,
            tiles_per_second: 8.0,
        }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "Sprite material"
    }

    fn update(&mut self, key_state: &KeyState) {
        let tile_number = ((web::now().unwrap() as f32) * self.tiles_per_second / 1000.0).floor();
        self.sprite_material
            .borrow_mut()
            .set_tile_number(tile_number);
        self.controller.update(key_state);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.renderer.render(context, &self.scene, &self.camera)
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}

async fn create_sprite_material(
    context: &WebGl2RenderingContext,
    properties: material::sprite::Properties,
) -> Result<SharedRef<SpriteMaterial>> {
    let material = shared_ref::new(SpriteMaterial {
        properties,
        texture: Texture::fetch(context, "images/rolling-ball.png").await?,
        unit: TextureUnit(0),
    });
    Ok(material)
}

async fn create_sprite(
    context: &WebGl2RenderingContext,
    sprite_material: SharedRef<SpriteMaterial>,
) -> Result<SharedRef<Node>> {
    let geometry = Geometry::from_with_context(context, Rectangle::default())?;
    let sprite = Node::new_with_mesh(Mesh::initialize(
        context,
        &geometry,
        <Rc<Material>>::from_with_context(context, sprite_material)?,
    )?);
    Ok(sprite)
}
